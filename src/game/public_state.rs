use super::Status;
use crate::{
    card, player, scoring, trick, Bid, Card, Player, Score, TeamRoundResult,
    Trick,
};

/// Game state that is viewable by all players.
#[derive(Clone, Debug)]
pub struct PublicState {
    /// The current score.
    scores: [Score; 2],
    /// The results from past rounds.
    round_results: Vec<[TeamRoundResult; 2]>,
    /// The index of the dealer for this round.
    dealer: Player,
    /// If each player has seen their cards.
    seen_cards: player::Array<bool>,
    /// If a trump card has been played yet.
    trump_broken: bool,
    /// The player that bid nil and is waiting for partner confirmation.
    pending_nil_player: Option<Player>,
    /// If each player has had a nil rejected this turn, meaning that
    /// they can not attempt to bid nil again this round.
    nil_rejected: player::Array<bool>,
    /// Each player's bid.
    bids: player::Array<Option<Bid>>,
    /// The number of tricks each player has taken.
    tricks_taken: player::Array<u8>,
    /// The current trick.
    trick: Trick,
}

impl Default for PublicState {
    fn default() -> Self {
        Self {
            scores: [Score::default(), Score::default()],
            round_results: Vec::new(),
            dealer: Player::One,
            seen_cards: player::Array::from_value(&false),
            trump_broken: false,
            pending_nil_player: None,
            nil_rejected: player::Array::default(),
            bids: player::Array::default(),
            tricks_taken: player::Array::from_value(&0),
            trick: Trick::new(Player::Two),
        }
    }
}

impl PublicState {
    /// Gets the score of both teams.
    pub fn get_scores(&self) -> [Score; 2] {
        self.scores
    }

    /// Get the results of all completed rounds.
    pub fn get_round_results(&self) -> &Vec<[TeamRoundResult; 2]> {
        &self.round_results
    }

    /// Gets if the user can see their cards.
    pub fn can_see_cards(&self, player: Player) -> bool {
        self.seen_cards[player]
    }

    /// Gets if trump is broken.
    ///
    /// This means that a trump card was played in a previous trick.
    pub fn is_trump_broken(&self) -> bool {
        self.trump_broken
    }

    /// Gets if a nil bid has been rejected this round,
    /// which prevents the player from bidding nil again this round.
    pub fn get_nil_rejected(&self, player: Player) -> bool {
        self.nil_rejected[player]
    }

    /// Gets a player's bid, if it has been made.
    pub fn get_bid(&self, player: Player) -> Option<Bid> {
        self.bids[player]
    }

    /// Gets the number of tricks that a player has taken this round.
    pub fn get_num_tricks(&self, player: Player) -> u8 {
        self.tricks_taken[player]
    }

    /// Gets a copy of the current trick.
    pub fn get_trick(&self) -> Trick {
        self.trick
    }

    /// Gets the current status of this game.
    pub fn get_status(&self) -> Status {
        if scoring::get_winning_team_index(self.get_scores()).is_some() {
            return Status::GameOver;
        }

        if let Some(bidding_nil) = self.pending_nil_player {
            return Status::WaitingForNilConfirmation(bidding_nil.teammate());
        }

        for player in self.dealer.next().iter() {
            if self.bids[player].is_none() {
                return Status::WaitingForBid(player);
            }
        }

        match self.trick.get_status() {
            trick::Status::Waiting(player) => Status::WaitingForPlay(player),
            _ => {
                panic!("Reached unreachable code in PublicState::get_status()")
            }
        }
    }

    /// Internal function that gets the bids of every player or returns
    /// an error due to a missing bid.
    fn get_bids(&self) -> Result<player::Array<Bid>, String> {
        let mut bids = player::Array::from_value(&Bid::Nil);
        for player in Player::One.iter() {
            bids[player] = self.bids[player].ok_or_else(|| {
                format!("Internal error, no bid for player {}", player)
            })?;
        }
        Ok(bids)
    }

    /// Internal function called after a card has been played.
    ///
    /// Used by on_card_played() and unchecked_on_card_played().
    fn after_card_played(&mut self) -> Result<(), String> {
        if let trick::Status::Won(winning_player, winning_card) =
            self.trick.get_status()
        {
            // handle the end of the trick
            self.tricks_taken[winning_player] += 1;
            if winning_card.suite == crate::card::Suite::Spade {
                self.trump_broken = true;
            }
            self.trick = Trick::new(winning_player);

            let tricks_complete: u8 = self.tricks_taken.iter().sum();
            if tricks_complete == 13 {
                // handle the end of the round
                let results = TeamRoundResult::create_pair(
                    self.get_bids()?,
                    self.tricks_taken,
                );
                self.round_results.push(results);
                self.scores[0] += results[0].get_score();
                self.scores[1] += results[1].get_score();
                self.dealer = self.dealer.next();
                self.seen_cards.fill(&false);
                self.trump_broken = false;
                self.nil_rejected.fill(&false);
                self.bids.fill(&None);
                self.tricks_taken.fill(&0);
                self.trick = Trick::new(self.dealer.next());
            }
        }
        Ok(())
    }

    /// Call when a player plays a card and we have the player's hand available
    /// to ensure that it is valid for them to play the card.
    ///
    /// Will also remove the card from the hand on success.
    pub fn on_card_played(
        &mut self,
        player: Player,
        card: Card,
        hand: &mut card::Set,
    ) -> Result<(), String> {
        if !hand.contains(card) {
            return Err("You can not play a card not in your hand.".to_string());
        };
        match self.get_status() {
            Status::WaitingForBid(_) | Status::WaitingForNilConfirmation(_) => {
                Err("Can not play a card, bidding is not complete.".to_string())
            }
            Status::GameOver => {
                Err("Can not play a card, the game is over.".to_string())
            }
            Status::WaitingForPlay(_) => {
                // attempt to play the card
                if !self
                    .trick
                    .get_playable_cards(*hand, self.trump_broken)
                    .contains(card)
                {
                    return Err("Can not play the given card.".to_string());
                }
                self.trick.play_card(player, card)?;
                hand.remove(card);
                self.after_card_played()
            }
        }
    }

    /// Call when a player plays a card and we don't have their
    /// hand to validate if they can play the card they played.
    pub fn unchecked_on_card_played(
        &mut self,
        player: Player,
        card: Card,
    ) -> Result<(), String> {
        match self.get_status() {
            Status::WaitingForBid(_) | Status::WaitingForNilConfirmation(_) => {
                Err("Can not play a card, bidding is not complete.".to_string())
            }
            Status::GameOver => {
                Err("Can not play a card, the game is over.".to_string())
            }
            Status::WaitingForPlay(_) => {
                self.trick.play_card(player, card)?;
                self.after_card_played()
            }
        }
    }

    /// Handles a player making their bid.
    pub fn on_bid(&mut self, player: Player, bid: Bid) -> Result<(), String> {
        if self.get_status() != Status::WaitingForBid(player) {
            return Err("It is not your turn to bid.".to_string());
        }
        if bid == Bid::BlindNil && self.seen_cards[player] {
            return Err("Can not bid blind nil as you have seen your cards."
                .to_string());
        }
        if let Some(bid_error) =
            bid.get_compatibility_error(self.bids[player.teammate()])
        {
            return Err(bid_error.to_string());
        }

        if bid == Bid::Nil {
            if self.nil_rejected[player] {
                return Err("You can not bid nil if your partner has \
                already rejected your nil bid this bidding round."
                    .to_string());
            }
            self.pending_nil_player = Some(player);
        } else {
            self.bids[player] = Some(bid);
        }
        Ok(())
    }

    /// Handles a player wishing to see their cards, forfeiting
    /// their right to bid blind nil.
    pub fn on_cards_seen(&mut self, player: Player) {
        self.seen_cards[player] = true;
    }

    /// Handles a player indicating if they approve of their teammates nil bid.
    pub fn on_nil_approval(
        &mut self,
        player: Player,
        is_approved: bool,
    ) -> Result<(), String> {
        if let Some(bidding_nil) = self.pending_nil_player {
            if bidding_nil.teammate() == player {
                if is_approved {
                    self.bids[bidding_nil] = Some(Bid::Nil);
                } else {
                    self.nil_rejected[bidding_nil] = true;
                }
                self.pending_nil_player = None;
                Ok(())
            } else {
                Err("Can not confirm a nil bid, your teammate does not have \
                a nil bid pending."
                    .to_string())
            }
        } else {
            Err("Can not confirm a nil bid, no one has a nil bid pending."
                .to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bid() {
        let mut state = PublicState::default();
        for player in Player::Two.iter() {
            state.on_cards_seen(player);
            assert_eq!(state.get_status(), Status::WaitingForBid(player));
            assert_eq!(state.get_bid(player), None);
            state.on_bid(player, Bid::Take(player.to_index())).unwrap();
            assert_eq!(
                state.get_bid(player),
                Some(Bid::Take(player.to_index()))
            );
        }
    }

    #[test]
    fn nil_bid() {
        let mut state = PublicState::default();

        // player 2 bids nil
        state.on_cards_seen(Player::Two);
        state.on_bid(Player::Two, Bid::Nil).unwrap();

        // player 4 accepts it
        state.on_nil_approval(Player::Four, true).unwrap();
        assert_eq!(state.get_status(), Status::WaitingForBid(Player::Three));
    }

    #[test]
    fn blind_nil_bid() {
        let mut state = PublicState::default();

        // player 2 bids blind nil
        state.on_bid(Player::Two, Bid::BlindNil).unwrap();

        // player 3 fails to bid blind nil do to already seeing their cards
        assert_eq!(state.get_status(), Status::WaitingForBid(Player::Three));
        state.on_cards_seen(Player::Three);
        assert!(state.on_bid(Player::Three, Bid::BlindNil).is_err());
    }

    #[test]
    fn bid_out_of_turn_fails() {
        let mut state = PublicState::default();
        assert!(state.on_bid(Player::Three, Bid::Take(2)).is_err());
    }

    #[test]
    fn double_nil_fails() {
        let mut state = PublicState::default();
        state.on_bid(Player::Two, Bid::BlindNil).unwrap();
        state.on_bid(Player::Three, Bid::Nil).unwrap();
        state
            .on_nil_approval(Player::Three.teammate(), true)
            .unwrap();

        // bidding nil and blind nil when your teammate bid blind
        // nil is an error
        assert!(state.on_bid(Player::Four, Bid::Nil).is_err());
        assert!(state.on_bid(Player::Four, Bid::BlindNil).is_err());

        state.on_bid(Player::Four, Bid::Take(4)).unwrap();

        // same if the teammate bid nil
        assert!(state.on_bid(Player::One, Bid::Nil).is_err());
        assert!(state.on_bid(Player::One, Bid::BlindNil).is_err());
    }

    #[test]
    fn on_card_played() {
        let mut state = PublicState::default();

        // bid arbitrarily
        for player in Player::Two.iter() {
            state.on_cards_seen(player);
            state.on_bid(player, Bid::Take(3)).unwrap();
        }

        {
            assert_eq!(Status::WaitingForPlay(Player::Two), state.get_status());
            let mut hand = card::Set::suite(card::Suite::Diamond);
            let card = Card::new(card::Suite::Diamond, card::Value::Ace);
            state.on_card_played(Player::Two, card, &mut hand).unwrap();
            assert_eq!(12, hand.len());
            assert!(!hand.contains(card));
        }

        {
            assert_eq!(
                Status::WaitingForPlay(Player::Three),
                state.get_status()
            );
            let mut hand = card::Set::suite(card::Suite::Heart);
            let card = Card::new(card::Suite::Heart, card::Value::Number(4));
            state
                .on_card_played(Player::Three, card, &mut hand)
                .unwrap();
            assert_eq!(12, hand.len());
            assert!(!hand.contains(card));
        }

        {
            assert_eq!(
                Status::WaitingForPlay(Player::Four),
                state.get_status()
            );
            let mut hand = card::Set::suite(card::Suite::Spade);
            let card = Card::new(card::Suite::Spade, card::Value::Number(2));
            state.on_card_played(Player::Four, card, &mut hand).unwrap();
            assert_eq!(12, hand.len());
            assert!(!hand.contains(card));
        }

        {
            assert_eq!(Status::WaitingForPlay(Player::One), state.get_status());
            let mut hand = card::Set::suite(card::Suite::Club);
            let card = Card::new(card::Suite::Club, card::Value::Number(7));
            state.on_card_played(Player::One, card, &mut hand).unwrap();
            assert_eq!(12, hand.len());
            assert!(!hand.contains(card));
        }

        // player four should have won
        assert_eq!(1, state.get_num_tricks(Player::Four));
        // and therefore they are next to play
        assert_eq!(Status::WaitingForPlay(Player::Four), state.get_status());
        // no other players should have any tricks taken
        for player in Player::Four.iter().skip(1) {
            assert_eq!(0, state.get_num_tricks(player));
        }
        // trump was broken
        assert!(state.is_trump_broken());
    }

    #[test]
    fn unchecked_on_card_played() {
        let mut state = PublicState::default();

        // bid arbitrarily
        for player in Player::Two.iter() {
            state.on_cards_seen(player);
            state.on_bid(player, Bid::Take(3)).unwrap();
        }

        state
            .unchecked_on_card_played(
                Player::Two,
                Card::new(card::Suite::Diamond, card::Value::Number(5)),
            )
            .unwrap();
        state
            .unchecked_on_card_played(
                Player::Three,
                Card::new(card::Suite::Diamond, card::Value::Ace),
            )
            .unwrap();
        state
            .unchecked_on_card_played(
                Player::Four,
                Card::new(card::Suite::Diamond, card::Value::Number(4)),
            )
            .unwrap();
        state
            .unchecked_on_card_played(
                Player::One,
                Card::new(card::Suite::Diamond, card::Value::Number(2)),
            )
            .unwrap();

        // player three should have won
        assert_eq!(1, state.get_num_tricks(Player::Three));
        // and therefore is next to play
        assert_eq!(Status::WaitingForPlay(Player::Three), state.get_status());
        // no other players should have any tricks taken
        for player in Player::Three.iter().skip(1) {
            assert_eq!(0, state.get_num_tricks(player));
        }
        // trump was not broken
        assert!(!state.is_trump_broken());
    }

    #[test]
    fn end_round() {
        let mut state = PublicState::default();

        // bid arbitrarily
        for player in Player::Two.iter() {
            state.on_cards_seen(player);
            state.on_bid(player, Bid::Take(3)).unwrap();
        }

        let cards = player::Array::from_array([
            Card::new(card::Suite::Diamond, card::Value::Number(3)),
            Card::new(card::Suite::Diamond, card::Value::Ace),
            Card::new(card::Suite::Diamond, card::Value::Number(2)),
            Card::new(card::Suite::Diamond, card::Value::Number(4)),
        ]);

        // run 13 rounds
        for _ in 0..13 {
            for player in Player::Two.iter() {
                state
                    .unchecked_on_card_played(player, cards[player])
                    .unwrap();
            }
        }

        // the score should be updated
        assert_eq!([Score::new(-6, 0), Score::new(6, 7)], state.get_scores());
        // the rounds result should now have an entry
        assert_eq!(1, state.get_round_results().len());

        // now player two is the dealer, so player three bids next
        assert_eq!(Status::WaitingForBid(Player::Three), state.get_status());
    }

    #[test]
    fn play_at_wrong_time_fails() {
        let mut state = PublicState::default();
        let card = Card::new(card::Suite::Spade, card::Value::Ace);

        // playing during bidding is wrong
        assert!(state.unchecked_on_card_played(Player::Two, card).is_err());

        for player in Player::Two.iter() {
            state.on_cards_seen(player);
        }

        // still invalid after the cards are seen
        assert!(state.unchecked_on_card_played(Player::Two, card).is_err());

        for player in Player::Two.iter() {
            state.on_bid(player, Bid::Take(3)).unwrap();
        }

        // invalid when it is not their turn
        assert!(state.unchecked_on_card_played(Player::Three, card).is_err());
    }
}

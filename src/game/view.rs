use super::{Action, Event, Notification, PublicState, Response, Status};
use crate::{card, scoring, Bid, Card, Player, Score, TeamRoundResult, Trick};

/// A player's view of the state of the game.
///
/// Contains only the information that a single user knows.
/// In particular does not contain other user's hands.
#[derive(Clone, Debug)]
pub struct View {
    /// The player whose view this is of the game.
    player: Player,
    /// The public game state.
    public_state: PublicState,
    /// The user's hand, if they have selected seen cards.
    hand: Option<card::Set>,
}

impl View {
    /// Creates a view wrapping a public state.
    /// Is only called from spades::game::State::create_view().
    pub(super) fn from_public_state(
        player: Player,
        public_state: &PublicState,
        hand: card::Set,
    ) -> Self {
        View {
            player,
            public_state: public_state.clone(),
            hand: if public_state.can_see_cards(player) {
                Some(hand)
            } else {
                None
            },
        }
    }

    /// Creates a view of a brand new game from a player's perspective.
    pub fn new(player: Player) -> Self {
        View {
            player,
            public_state: PublicState::default(),
            hand: None,
        }
    }
}

/// Getters that describe the current state of the game.
impl View {
    /// Gets the scores of both teams.
    pub fn get_scores(&self) -> [Score; 2] {
        self.public_state.get_scores()
    }

    /// Get the results of all completed rounds.
    pub fn get_round_results(&self) -> &Vec<[TeamRoundResult; 2]> {
        self.public_state.get_round_results()
    }

    /// Gets if a player can see their cards.
    pub fn can_see_cards(&self, player: Player) -> bool {
        self.public_state.can_see_cards(player)
    }

    /// Gets if trump is broken.
    ///
    /// This means that a trump card was played in a previous trick.
    pub fn is_trump_broken(&self) -> bool {
        self.public_state.is_trump_broken()
    }

    /// Gets if a nil bid has been rejected this round,
    /// which prevents the player from bidding nil again this round.
    pub fn get_nil_rejected(&self, player: Player) -> bool {
        self.public_state.get_nil_rejected(player)
    }

    /// Gets a player's bid, if they have made one yet.
    pub fn get_bid(&self, player: Player) -> Option<Bid> {
        self.public_state.get_bid(player)
    }

    /// Gets the number of tricks that a player has taken.
    pub fn get_num_tricks(&self, player: Player) -> u8 {
        self.public_state.get_num_tricks(player)
    }

    /// Gets the a copy of the active trick.
    ///
    /// This contains the cards that have been played by each player.
    pub fn get_trick(&self) -> Trick {
        self.public_state.get_trick()
    }

    /// Gets the status of this game.
    pub fn get_status(&self) -> Status {
        self.public_state.get_status()
    }

    /// Gets the player that this view is for.
    pub fn get_player(&self) -> Player {
        self.player
    }

    /// Gets the hand of this player.
    ///
    /// Returns None if the game is over or if the player has
    /// not yet seen their hand.
    pub fn get_hand(&self) -> Option<card::Set> {
        self.hand
    }
}

/// Manipulates the game through Actions, Notifications, and Responses.
impl View {
    /// Sets the hand of the player.
    ///
    /// The player's hand is acquired by sending a SeeCards event to
    /// the server.
    fn set_hand(&mut self, hand: card::Set) {
        self.hand = Some(hand);
        self.public_state.on_cards_seen(self.player);
    }

    /// Makes a bid as the player.
    fn make_bid(&mut self, bid: Bid) -> Result<Event, String> {
        self.public_state.on_bid(self.player, bid)?;
        Ok(Event::MakeBid(bid))
    }

    /// Approves this player's teammate's nil bid.
    fn approve_nil(&mut self) -> Result<Event, String> {
        self.public_state.on_nil_approval(self.player, true)?;
        Ok(Event::ApprovesNil(true))
    }

    /// Rejects this player's teammate's nil bid.
    fn reject_nil(&mut self) -> Result<Event, String> {
        self.public_state.on_nil_approval(self.player, false)?;
        Ok(Event::ApprovesNil(false))
    }

    /// Internal function called after a card is played.
    ///
    /// Used to set this player's hand to None if the round has ended.
    fn after_card_played(&mut self) {
        if !self.public_state.can_see_cards(self.player) {
            self.hand = None;
        }
    }

    /// Plays a card as this player.
    fn play_card(&mut self, card: Card) -> Result<Event, String> {
        self.public_state.on_card_played(
            self.player,
            card,
            self.hand.as_mut().ok_or_else(|| {
                "Can not play a card without seeing your hand."
            })?,
        )?;
        self.after_card_played();
        Ok(Event::PlayCard(card))
    }

    /// Gets the actions that this player may perform at the current time.
    pub fn get_allowed_actions(&self) -> std::collections::HashSet<Action> {
        let mut set = std::collections::HashSet::default();
        if !self.can_see_cards(self.player) {
            set.insert(Action::SeeCards);
        }
        match self.get_status() {
            Status::WaitingForBid(player) => {
                if player != self.player {
                    set.insert(Action::Wait);
                } else if !self.can_see_cards(self.player) {
                    if Bid::BlindNil
                        .get_compatibility_error(
                            self.get_bid(self.player.teammate()),
                        )
                        .is_none()
                    {
                        set.insert(Action::MakeBid(Bid::BlindNil));
                    }
                } else {
                    set.extend(
                        scoring::bid::Generator::default()
                            .filter(|bid| *bid != Bid::BlindNil)
                            .filter(|bid| {
                                *bid != Bid::Nil
                                    || !self.get_nil_rejected(self.player)
                            })
                            .filter(|bid| {
                                bid.get_compatibility_error(
                                    self.get_bid(self.player.teammate()),
                                )
                                .is_none()
                            })
                            .map(|bid| Action::MakeBid(bid)),
                    );
                }
            }
            Status::WaitingForNilConfirmation(player) => {
                if player != self.player {
                    set.insert(Action::Wait);
                } else {
                    set.insert(Action::AllowNil);
                    set.insert(Action::RejectNil);
                }
            }
            Status::WaitingForPlay(player) => {
                if player != self.player {
                    set.insert(Action::Wait);
                } else {
                    set.extend(
                        self.get_trick()
                            .get_playable_cards(
                                self.hand.unwrap_or_default(),
                                self.is_trump_broken(),
                            )
                            .iter()
                            .map(|card| Action::PlayCard(card)),
                    )
                }
            }
            Status::GameOver => {
                // no valid actions when the game is over
                return std::collections::HashSet::default();
            }
        }
        set
    }

    /// Performs an action.
    pub fn perform_action(
        &mut self,
        action: Action,
    ) -> Result<Option<Event>, String> {
        match action {
            Action::Wait => {
                let player = match self.get_status() {
                    Status::WaitingForBid(player) => player,
                    Status::WaitingForNilConfirmation(player) => player,
                    Status::WaitingForPlay(player) => player,
                    Status::GameOver => {
                        return Err(
                            "Can not wait when the game is over".to_string()
                        );
                    }
                };
                if self.player == player {
                    Err("Can not wait when the game is waiting on you."
                        .to_string())
                } else {
                    Ok(None)
                }
            }
            Action::SeeCards => {
                if self.get_status() == Status::GameOver {
                    Err("Can not request to see your cards when the game \
                    is over"
                        .to_string())
                } else if self.public_state.can_see_cards(self.player) {
                    Err("Can not request to see your cards when you can \
                    already see them."
                        .to_string())
                } else {
                    Ok(Some(Event::SeeCards))
                }
            }
            Action::AllowNil => self.approve_nil().map(|x| Some(x)),
            Action::RejectNil => self.reject_nil().map(|x| Some(x)),
            Action::MakeBid(bid) => self.make_bid(bid).map(|x| Some(x)),
            Action::PlayCard(card) => self.play_card(card).map(|x| Some(x)),
        }
    }

    /// Handles a response from the server.
    pub fn handle_response(
        &mut self,
        response: Response,
    ) -> Result<(), String> {
        match response {
            Response::Ok => Ok(()),
            Response::Cards(cards) => {
                self.set_hand(cards);
                Ok(())
            }
            Response::Err(error) => Err(error),
        }
    }

    /// Handles a notification from the server.
    pub fn handle_notification(
        &mut self,
        notification: Notification,
    ) -> Result<(), String> {
        if notification.player == self.player {
            return Err("Notifications from a player can not be applied to \
                the player's own view of the game."
                .to_string());
        }
        match notification.event {
            Event::SeeCards => {
                self.public_state.on_cards_seen(notification.player);
            }
            Event::MakeBid(bid) => {
                self.public_state.on_bid(notification.player, bid)?;
            }
            Event::ApprovesNil(approves) => self
                .public_state
                .on_nil_approval(notification.player, approves)?,
            Event::PlayCard(card) => {
                self.public_state
                    .unchecked_on_card_played(notification.player, card)?;
                self.after_card_played();
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn initial_allowed_actions() {
        let first_bidder = Player::Two;

        // The first player to bid may both see their cards and bid blind nil.
        {
            let view = View::new(first_bidder);
            let allowed_actions = view.get_allowed_actions();
            assert!(allowed_actions.contains(&Action::SeeCards));
            assert!(allowed_actions.contains(&Action::MakeBid(Bid::BlindNil)));
            assert_eq!(2, allowed_actions.len());
        }

        // All other players can only request to see their cards or
        // wait for it to be their turn to bid.
        for player in first_bidder.iter().skip(1) {
            let view = View::new(player);
            let allowed_actions = view.get_allowed_actions();
            assert!(allowed_actions.contains(&Action::SeeCards));
            assert!(allowed_actions.contains(&Action::Wait));
            assert_eq!(2, allowed_actions.len());
        }
    }

    /// Every player is allowed to request to see their cards with a new game.
    #[test]
    fn see_cards() {
        for player in Player::One.iter() {
            let mut view = View::new(player);
            assert_eq!(None, view.get_hand());

            // request is valid
            let event = view.perform_action(Action::SeeCards).unwrap();
            assert_eq!(event, Some(Event::SeeCards));

            // handles a response with the cards well
            let hand = card::Set::suite(card::Suite::Spade);
            view.handle_response(Response::Cards(hand)).unwrap();

            assert_eq!(hand, view.get_hand().unwrap());
        }
    }

    /// Handles other players seeing their cards well and is able to
    /// see their cards afterwards.
    #[test]
    fn see_cards_after_others() {
        for player in Player::One.iter() {
            let mut view = View::new(player);
            // all other players see their cards
            for other_player in player.iter().skip(1) {
                view.handle_notification(Notification {
                    player: other_player,
                    event: Event::SeeCards,
                })
                .unwrap();
            }

            let event = view.perform_action(Action::SeeCards).unwrap();
            assert_eq!(event, Some(Event::SeeCards));

            let hand = card::Set::suite(card::Suite::Spade);
            view.handle_response(Response::Cards(hand)).unwrap();

            assert_eq!(hand, view.get_hand().unwrap());
        }
    }

    #[test]
    fn blind_nil() {
        for player in Player::One.iter() {
            let mut view = View::new(player);

            // all previous players must bid
            for bid_player in Player::Two
                .iter()
                .take_while(|bid_player| *bid_player != player)
            {
                view.handle_notification(Notification {
                    player: bid_player,
                    event: Event::SeeCards,
                })
                .unwrap();
                view.handle_notification(Notification {
                    player: bid_player,
                    event: Event::MakeBid(Bid::Take(3)),
                })
                .unwrap();
            }

            let allowed_actions = HashSet::from_iter(
                [Action::MakeBid(Bid::BlindNil), Action::SeeCards]
                    .iter()
                    .copied(),
            );
            assert_eq!(allowed_actions, view.get_allowed_actions());

            view.perform_action(Action::MakeBid(Bid::BlindNil)).unwrap();
        }
    }

    #[test]
    fn blind_nil_invalid_due_to_seen_cards() {
        for player in Player::One.iter() {
            let mut view = View::new(player);

            // all previous players must bid
            for bid_player in Player::Two
                .iter()
                .take_while(|bid_player| *bid_player != player)
            {
                view.handle_notification(Notification {
                    player: bid_player,
                    event: Event::SeeCards,
                })
                .unwrap();
                view.handle_notification(Notification {
                    player: bid_player,
                    event: Event::MakeBid(Bid::Take(3)),
                })
                .unwrap();
            }

            view.perform_action(Action::SeeCards).unwrap();
            view.handle_response(Response::Cards(card::Set::suite(
                card::Suite::Spade,
            )))
            .unwrap();
            assert!(!view
                .get_allowed_actions()
                .contains(&Action::MakeBid(Bid::BlindNil)));
            assert!(view
                .perform_action(Action::MakeBid(Bid::BlindNil))
                .is_err());
        }
    }

    #[test]
    fn nil_accept() {
        let mut view = View::new(Player::Two);

        // see cards
        view.perform_action(Action::SeeCards).unwrap();
        view.handle_response(Response::Cards(card::Set::suite(
            card::Suite::Spade,
        )))
        .unwrap();

        // bid nil
        view.perform_action(Action::MakeBid(Bid::Nil)).unwrap();

        // partner accepts
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::ApprovesNil(true),
        })
        .unwrap();

        // bid was accepted
        assert_eq!(Some(Bid::Nil), view.get_bid(view.get_player()));
    }

    #[test]
    fn nil_reject() {
        let mut view = View::new(Player::Two);

        // see cards
        view.perform_action(Action::SeeCards).unwrap();
        view.handle_response(Response::Cards(card::Set::suite(
            card::Suite::Spade,
        )))
        .unwrap();

        // bid nil
        view.perform_action(Action::MakeBid(Bid::Nil)).unwrap();

        // partner rejects
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::ApprovesNil(false),
        })
        .unwrap();

        // is my turn again to bid
        assert_eq!(Status::WaitingForBid(view.player), view.get_status());

        // can not attempt to bid nil again
        assert!(!view
            .get_allowed_actions()
            .contains(&Action::MakeBid(Bid::Nil)));
        assert!(view.perform_action(Action::MakeBid(Bid::Nil)).is_err());
    }

    #[test]
    fn partner_nil_accept() {
        let mut view = View::new(Player::Four);

        // opponent see cards and bids
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::SeeCards,
        })
        .unwrap();
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::MakeBid(Bid::Nil),
        })
        .unwrap();

        let allowed_actions = HashSet::from_iter(
            [Action::SeeCards, Action::AllowNil, Action::RejectNil]
                .iter()
                .copied(),
        );
        assert_eq!(allowed_actions, view.get_allowed_actions());

        // accept nil
        view.perform_action(Action::AllowNil).unwrap();

        assert_eq!(Some(Bid::Nil), view.get_bid(view.player.teammate()));
    }

    #[test]
    fn partner_nil_reject() {
        let mut view = View::new(Player::Four);

        // opponent see cards and bids
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::SeeCards,
        })
        .unwrap();
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::MakeBid(Bid::Nil),
        })
        .unwrap();

        // deny nil
        view.perform_action(Action::RejectNil).unwrap();

        assert_eq!(None, view.get_bid(view.player.teammate()));
    }

    #[test]
    fn bid_compatibility_error() {
        let mut view = View::new(Player::Four);

        // everyone sees their cards
        view.perform_action(Action::SeeCards).unwrap();
        view.handle_response(Response::Cards(card::Set::suite(
            card::Suite::Spade,
        )))
        .unwrap();
        for player in view.player.iter().skip(1) {
            view.handle_notification(Notification {
                player,
                event: Event::SeeCards,
            })
            .unwrap();
        }

        // teammate bids
        view.handle_notification(Notification {
            player: view.player.teammate(),
            event: Event::MakeBid(Bid::Take(7)),
        })
        .unwrap();

        // opponent bids
        view.handle_notification(Notification {
            player: view.player.teammate().next(),
            event: Event::MakeBid(Bid::Take(3)),
        })
        .unwrap();

        // my bid
        assert!(view.perform_action(Action::MakeBid(Bid::Take(7))).is_err());
    }

    #[test]
    fn play_card() {
        let mut view = View::new(Player::Two);

        // everyone sees their cards
        view.perform_action(Action::SeeCards).unwrap();
        view.handle_response(Response::Cards(card::Set::suite(
            card::Suite::Spade,
        )))
        .unwrap();
        for player in view.player.iter().skip(1) {
            view.handle_notification(Notification {
                player,
                event: Event::SeeCards,
            })
            .unwrap();
        }

        // everyone bids
        view.perform_action(Action::MakeBid(Bid::Take(3))).unwrap();
        for player in Player::Two.iter().skip(1) {
            view.handle_notification(Notification {
                player,
                event: Event::MakeBid(Bid::Take(3)),
            })
            .unwrap();
        }

        // play a card
        view.perform_action(Action::PlayCard(Card::new(
            card::Suite::Spade,
            card::Value::Ace,
        )))
        .unwrap();

        // opponent plays a card
        view.handle_notification(Notification {
            player: view.player.next(),
            event: Event::PlayCard(Card::new(
                card::Suite::Diamond,
                card::Value::King,
            )),
        })
        .unwrap();
    }

    /// Possible that a player has not yet seen their cards
    /// when it is their turn to play a card.
    /// Only possible with a bid of blind nil.
    #[test]
    fn must_play_with_unseen_cards() {
        let mut view = View::new(Player::Two);

        // everyone but our player sees their cards
        for player in Player::Two.iter().skip(1) {
            view.handle_notification(Notification {
                player,
                event: Event::SeeCards,
            })
            .unwrap();
        }

        // we bid blind nil
        view.perform_action(Action::MakeBid(Bid::BlindNil)).unwrap();

        // all other players bid
        for player in Player::Two.iter().skip(1) {
            view.handle_notification(Notification {
                player,
                event: Event::MakeBid(Bid::Take(4)),
            })
            .unwrap();
        }

        // now is our turn to play
        assert_eq!(Status::WaitingForPlay(Player::Two), view.get_status());

        // but we can only request to see our cards
        let mut allowed_actions = HashSet::default();
        allowed_actions.insert(Action::SeeCards);
        assert_eq!(allowed_actions, view.get_allowed_actions());
    }
}

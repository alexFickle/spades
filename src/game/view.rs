use super::{Event, Notification, PublicState, Status};
use crate::{card, Bid, Card, Player, Score, TeamRoundResult, Trick};

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
    /// Creates a new view from a player's perspective.
    pub(super) fn new(
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

    /// Gets uf trump is broken.
    ///
    /// This means that a trump card was played in a previous trick.
    pub fn is_trump_broken(&self) -> bool {
        self.public_state.is_trump_broken()
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
    /// This contains information like the currently played cards,
    /// the lead suite, and functions to filter a hand into playable cards.
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

    /// Sets the hand of the player.
    ///
    /// The player's hand is acquired by sending a SeeCards event to
    /// the server.
    pub fn set_hand(&mut self, hand: card::Set) {
        self.hand = Some(hand);
        self.public_state.on_cards_seen(self.player);
    }

    /// Makes a bid as the player.
    pub fn make_bid(&mut self, bid: Bid) -> Result<Event, String> {
        self.public_state.on_bid(self.player, bid)?;
        Ok(Event::MakeBid(bid))
    }

    /// Approves this player's teammate's nil bid.
    pub fn approve_nil(&mut self) -> Result<Event, String> {
        self.public_state.on_nil_approval(self.player, true)?;
        Ok(Event::ApprovesNil(true))
    }

    /// Rejects this player's teammate's nil bid.
    pub fn reject_nil(&mut self) -> Result<Event, String> {
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
    pub fn play_card(&mut self, card: Card) -> Result<Event, String> {
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

    /// Handles a notification from the server.
    pub fn handle_notification(
        &mut self,
        notification: Notification,
    ) -> Result<(), String> {
        match notification.event {
            Event::SeeCards => {
                self.public_state.on_cards_seen(notification.player)
            }
            Event::MakeBid(bid) => {
                self.public_state.on_bid(notification.player, bid)?
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

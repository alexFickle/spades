use super::{dealer, Event, Notification, Response, Status, View};
use crate::{card, player, Player};

/// The state of the game.
///
/// Contains the hands of every player, so must not be sent to clients.
pub struct State {
    /// The state observable to every player.
    public_state: super::PublicState,
    /// The dealer that populates every hand.
    dealer: Box<dyn dealer::Dealer>,
    /// Each player's hands.
    hands: player::Array<card::Set>,
}

impl std::fmt::Debug for State {
    /// Debug prints State, with ignoring the dealer field.
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("State")
            .field("public_state", &self.public_state)
            .field("hands", &self.hands)
            .finish()
    }
}

impl Default for State {
    /// Creates a new game::State that uses game::dealer::ShuffledDealer to
    /// deal cards.
    fn default() -> Self {
        Self::new(Box::new(dealer::ShuffledDealer::default()))
    }
}

impl State {
    /// Creates a new game::State from a boxed dealer.
    pub fn new(dealer: Box<dyn dealer::Dealer>) -> Self {
        let mut game = Self {
            public_state: super::PublicState::default(),
            dealer,
            hands: player::Array::default(),
        };
        game.hands = game.dealer.deal_cards();
        game
    }

    /// Handles an event caused by a player's action.
    ///
    /// Returns a Response that should be sent back to the client sending
    /// this event and optionally a notification that should be sent to all
    /// other clients.
    pub fn handle_event(
        &mut self,
        player: Player,
        event: Event,
    ) -> (Response, Option<Notification>) {
        match event {
            Event::SeeCards => {
                self.public_state.on_cards_seen(player);
                (
                    Response::Cards(self.hands[player]),
                    Some(Notification { player, event }),
                )
            }
            Event::MakeBid(bid) => {
                if let Err(error) = self.public_state.on_bid(player, bid) {
                    (Response::Err(error), None)
                } else {
                    (Response::Ok, Some(Notification { player, event }))
                }
            }
            Event::PlayCard(card) => {
                if let Err(error) = self.public_state.on_card_played(
                    player,
                    card,
                    &mut self.hands[player],
                ) {
                    (Response::Err(error), None)
                } else {
                    if let Ok(Status::WaitingForBid(_)) =
                        self.public_state.get_status()
                    {
                        // start of new round
                        self.hands = self.dealer.deal_cards();
                    }
                    (Response::Ok, Some(Notification { player, event }))
                }
            }
            Event::ApprovesNil(approves) => {
                if let Err(error) =
                    self.public_state.on_nil_approval(player, approves)
                {
                    (Response::Err(error), None)
                } else {
                    (Response::Ok, Some(Notification { player, event }))
                }
            }
        }
    }

    /// Gets if this game is over or not.
    pub fn is_game_over(&self) -> Result<bool, String> {
        Ok(self.public_state.get_status()? == Status::GameOver)
    }

    /// Creates a player's view of the game.
    pub fn create_view(&self, player: Player) -> View {
        View::new(player, &self.public_state, self.hands[player])
    }
}

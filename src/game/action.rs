use crate::{Bid, Card};

/// Contains all of the possible actions for a player to perform.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Action {
    /// Do nothing.  Waiting for another player to perform an action.
    Wait,
    /// See the player's card.
    SeeCards,
    /// Permit the player's partner to bid nil.
    AllowNil,
    /// Reject the player's partner nil bid.
    RejectNil,
    /// Make a bid.
    MakeBid(Bid),
    /// Play a card.
    PlayCard(Card),
}

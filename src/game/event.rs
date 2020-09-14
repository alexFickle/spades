use crate::card::Card;
use crate::Bid;

/// Actions that a player can perform that changes a game's state.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    /// A player wishes to see their cards, forfeiting their right to bid
    /// blind nil if they have not already done so.
    SeeCards,
    /// A player makes a bid.
    /// May be blind nil only if the player has not yet seen their cards.
    /// If the bid is a regular nil then it requires confirmation by
    /// their teammate.
    MakeBid(Bid),
    /// A player approves or disproves of their partner bidding nil.
    ApprovesNil(bool),
    /// A player plays a card.
    PlayCard(Card),
}

/// A player's bid.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Bid {
    /// A player must take no tricks.  They decided before they saw their cards.
    BlindNil,
    /// A player must take no tricks.  They decided after they saw their cards.
    Nil,
    /// The player claims that they will take the wrapper number of tricks.
    Take(u8),
}

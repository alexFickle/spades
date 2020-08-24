use crate::Player;

/// The status of the game.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Status {
    /// The game is waiting for a player to bid.
    WaitingForBid(Player),
    /// The game is waiting for a player to confirm their teammate's nil.
    WaitingForNilConfirmation(Player),
    /// The game is waiting for a player to play a card.
    WaitingForPlay(Player),
    /// The game is over.
    GameOver,
}

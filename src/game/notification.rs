use super::Event;
use crate::Player;

/// When a client performs some action it sends a `game::Event` to the server.
/// If the server determines the action is valid it sends this notification
/// to all other clients so that they may update their `game::View`.
#[derive(Debug, Clone)]
pub struct Notification {
    /// The player whose action caused the event.
    pub player: Player,
    /// The event.
    pub event: Event,
}

//! Contains `State` and `View` that contain the state of a game.
//!
//! Also contains `Event` that can be sent from a client to a server
//! to manipulate the game, `Response` that is sent back to a client
//! in response to an event, and `Status` that describes what the game
//! is waiting for.

pub mod dealer;

mod event;
pub use event::Event;

mod notification;
pub use notification::Notification;

mod public_state;
use public_state::PublicState;

mod response;
pub use response::Response;

mod state;
pub use state::State;

mod status;
pub use status::Status;

mod view;
pub use view::View;

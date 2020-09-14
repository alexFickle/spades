//! Contains types that contain and manipulate the state of the game.
//!
//! The entirety of the game's state is held in [`State`].
//! A single player's view of the game is held in a [`View`] that is
//! created using [`State::create_view()`].
//!
//! When a client wishes to manipulate the game state by performing some
//! action the client uses its [`View`]'s public functions.
//! All actions that a client can perform can be encoded with an
//! [`Action`] that can be passed to [`View::perform_action()`].
//! A set of all valid actions that may be performed can be acquired
//! from a [`View`] using [`View::get_allowed_actions()`].
//!
//! Certain actions result in an [`Event`] that must be sent to the server.
//! The server always responds with a [`Response`].  A valid [`Event`] that is
//! sent to a server also causes [`Notification`]s to be sent to each other
//! client. These [`Notification`]s are used to update each client's [`View`]
//! of the game using [`View::handle_notification()`].
//!
//! TODO: If a nil bid request has been denied do not let the player
//! attempt to bid nil again.
//!
//! [`State`]: struct.State.html
//! [`View`]: struct.View.html
//! [`Action`]: enum.Action.html
//! [`Event`]: enum.Event.html
//! [`Response`]: enum.Response.html
//! [`Notification`]: struct.Notification.html
//! [`State::create_view()`]: struct.State.html#method.create_view
//! [`View::perform_action()`]: struct.View.html#method.perform_action
//! [`View::get_allowed_actions()`]: struct.View.html#method.get_allowed_actions
//! [`View::handle_notification()`]: struct.View.html#method.handle_notification

mod action;
pub use action::Action;

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

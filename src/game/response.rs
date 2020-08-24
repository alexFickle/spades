use crate::card;

/// Sent from a server to a client in response to a `Event` being sent
/// by a client.
#[derive(Debug, Clone)]
pub enum Response {
    /// Response to every event except for SeeCards when no error occurs.
    Ok,
    /// Response to the SeeCards event when no error occurs.
    Cards(card::Set),
    /// Response to any event when an error occurs.
    Err(String),
}

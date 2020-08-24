//! Library that provides a serializable data model and logic for spades.
//!
//! This is intended to be used to create a sever, GUI client,
//! and AI for the variant of spades that my family plays.
//! The main difference from other variants is a minimum team bid of four tricks,
//! bidding 10 tricks is worth 200 points, and not making the number of tricks
//! bid causes nils to fail even if the player bidding nil succeeds.

#![warn(missing_docs)]

pub mod card;
pub use card::Card;

pub mod game;

pub mod player;
pub use player::Player;

pub mod scoring;
pub use scoring::{Bid, Score, TeamRoundResult};

pub mod trick;
pub use trick::Trick;

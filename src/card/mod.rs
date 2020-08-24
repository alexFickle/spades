//! Contains the `Card` struct, the `Suite` and `Value` enums used by it,
//! and a `Set` type that can contain them.

mod suite;
pub use suite::Suite;

mod value;
pub use value::Value;

pub mod set;
pub use set::Set;

/// Uniquely identifies a card within a deck.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    /// The suite of this card.
    pub suite: Suite,
    /// The value of this card.
    pub value: Value,
}

impl Card {
    /// Creates a card from its suite and value.
    pub fn new(suite: Suite, value: Value) -> Self {
        Card { suite, value }
    }

    /// Converts a number in the range of [0, 52) to a card.
    pub fn from_index(index: u8) -> Result<Self, String> {
        if !(index < 52) {
            Err(format!("Invalid card index: {}", index))
        } else {
            Ok(Self::new(
                Suite::from_index(index / 13)?,
                Value::from_index(index % 13)?,
            ))
        }
    }

    /// Converts a card to a number in the range of [0, 52).
    pub fn to_index(self) -> u8 {
        (self.suite.to_index() * 13) + self.value.to_index()
    }

    /// Creates a card from its string representation.
    pub fn from_chars(chars: [char; 2]) -> Result<Self, String> {
        Ok(Self::new(
            Suite::from_char(chars[0])?,
            Value::from_char(chars[1])?,
        ))
    }

    /// Converts a card to its string representation.
    pub fn to_chars(self) -> [char; 2] {
        [self.suite.to_char(), self.value.to_char()]
    }
}

/// Makes a randomly shuffled deck.
pub fn make_shuffled() -> Vec<Card> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut vec = Vec::new();
    vec.reserve(52);
    for i in 0..52 {
        vec.push(Card::from_index(i).unwrap());
    }
    vec.shuffle(&mut thread_rng());

    vec
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn make_shuffled_has_every_card() {
        let cards = make_shuffled();
        assert_eq!(cards.len(), 52);
        let mut cards_set = HashSet::new();
        for card in cards.iter() {
            assert!(cards_set.insert(*card));
        }
    }
}

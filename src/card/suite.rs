/// Enumeration for the suite of a card.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Suite {
    /// The trump suite.
    Spade,
    #[allow(missing_docs)]
    Heart,
    #[allow(missing_docs)]
    Club,
    #[allow(missing_docs)]
    Diamond,
}

impl Suite {
    /// Converts a value in the range of [0, 4) to a Suite.
    pub fn from_index(index: u8) -> Result<Self, String> {
        match index {
            0 => Ok(Suite::Spade),
            1 => Ok(Suite::Heart),
            2 => Ok(Suite::Club),
            3 => Ok(Suite::Diamond),
            _ => Err(format!("Invalid suite index: {}", index)),
        }
    }

    /// Converts a Suite to a value in the range of [0, 4).
    pub fn to_index(self) -> u8 {
        match self {
            Suite::Spade => 0,
            Suite::Heart => 1,
            Suite::Club => 2,
            Suite::Diamond => 3,
        }
    }

    /// Creates a Suite from its character representation.
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'S' => Ok(Suite::Spade),
            'H' => Ok(Suite::Heart),
            'C' => Ok(Suite::Club),
            'D' => Ok(Suite::Diamond),
            _ => Err(format!("Invalid card suite character: '{}'", c)),
        }
    }

    /// Converts a Suite into a single character.
    pub fn to_char(self) -> char {
        match self {
            Suite::Spade => 'S',
            Suite::Heart => 'H',
            Suite::Club => 'C',
            Suite::Diamond => 'D',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_index() {
        for i in 0..4 {
            assert_eq!(i, Suite::from_index(i).unwrap().to_index());
        }
    }

    #[test]
    fn round_trip_char() {
        for c in "SHCD".chars() {
            assert_eq!(c, Suite::from_char(c).unwrap().to_char());
        }
    }
}

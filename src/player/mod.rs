//! Contains the `Player` enum and related types.

mod iterator;
pub use iterator::Iterator;

mod array;
pub use array::Array;

/// The possible players.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Player {
    /// Starts the game as the dealer.
    One,
    #[allow(missing_docs)]
    Two,
    /// Is on player one's team.
    Three,
    /// Is on player two's team.
    Four,
}

impl Player {
    /// Creates a Player from an index in the range of [0, 4).
    pub fn from_index(index: u8) -> Result<Self, String> {
        match index {
            0 => Ok(Player::One),
            1 => Ok(Player::Two),
            2 => Ok(Player::Three),
            3 => Ok(Player::Four),
            _ => Err(format!("Invalid player index: {}", index)),
        }
    }

    /// Player to index.
    pub fn to_index(self) -> u8 {
        match self {
            Player::One => 0,
            Player::Two => 1,
            Player::Three => 2,
            Player::Four => 3,
        }
    }

    /// Gets the next player, with wrapping.
    pub fn next(self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::Three,
            Player::Three => Player::Four,
            Player::Four => Player::One,
        }
    }

    /// Gets the previous player, with wrapping.
    pub fn previous(self) -> Self {
        match self {
            Player::One => Player::Four,
            Player::Two => Player::One,
            Player::Three => Player::Two,
            Player::Four => Player::Three,
        }
    }

    /// Gets a player's teammate.
    pub fn teammate(self) -> Self {
        self.next().next()
    }

    /// Gets an iterator that will iterate over all players
    /// starting at this player without repetition.
    pub fn iter(self) -> Iterator {
        Iterator::new(self)
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::One => write!(f, "Player::One"),
            Player::Two => write!(f, "Player::Two"),
            Player::Three => write!(f, "Player::Three"),
            Player::Four => write!(f, "Player::Four"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_index() {
        for i in 0..4 {
            assert_eq!(i, Player::from_index(i).unwrap().to_index());
        }
    }

    #[test]
    fn ordering() {
        let ordered_pairs = [
            (Player::One, Player::Two),
            (Player::Two, Player::Three),
            (Player::Three, Player::Four),
            (Player::Four, Player::One),
        ];

        for pair in ordered_pairs.iter() {
            assert_eq!(pair.0.next(), pair.1);
            assert_eq!(pair.0, pair.1.previous());
        }
    }

    #[test]
    fn teammate() {
        let teams = [(Player::One, Player::Three), (Player::Two, Player::Four)];

        for team in teams.iter() {
            assert_eq!(team.0.teammate(), team.1);
            assert_eq!(team.1.teammate(), team.0);
        }
    }

    #[test]
    fn iter_starts_at_self() {
        assert_eq!(Player::One, Player::One.iter().next().unwrap());
        assert_eq!(Player::Two, Player::Two.iter().next().unwrap());
        assert_eq!(Player::Three, Player::Three.iter().next().unwrap());
        assert_eq!(Player::Four, Player::Four.iter().next().unwrap());
    }
}

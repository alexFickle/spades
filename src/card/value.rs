/// Enumeration for the value of a card.
///
/// The values are ordered as 2, ..., 10, Jack, Queen, King, Ace.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// The inner number is in the range of [2, 10].
    Number(u8),
    #[allow(missing_docs)]
    Jack,
    #[allow(missing_docs)]
    Queen,
    #[allow(missing_docs)]
    King,
    #[allow(missing_docs)]
    Ace,
}

impl Value {
    /// Converts a number in the range of [0, 13) to a Value
    pub fn from_index(index: u8) -> Result<Self, String> {
        match index {
            0..=8 => Ok(Value::Number(index + 2)),
            9 => Ok(Value::Jack),
            10 => Ok(Value::Queen),
            11 => Ok(Value::King),
            12 => Ok(Value::Ace),
            _ => Err(format!("Invalid card value index: {}", index)),
        }
    }

    /// Converts a Value into a number in the range of [0, 13)
    pub fn to_index(self) -> u8 {
        match self {
            Value::Number(number) => number - 2,
            Value::Jack => 9,
            Value::Queen => 10,
            Value::King => 11,
            Value::Ace => 12,
        }
    }

    /// Converts a character into a Value.
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            '2'..='9' => Ok(Value::Number(c as u8 - '0' as u8)),
            'X' => Ok(Value::Number(10)),
            'J' => Ok(Value::Jack),
            'Q' => Ok(Value::Queen),
            'K' => Ok(Value::King),
            'A' => Ok(Value::Ace),
            _ => Err(format!("Invalid card value character: '{}'", c)),
        }
    }

    /// Converts a Value into its single character representation.
    pub fn to_char(self) -> char {
        match self {
            Value::Number(10) => 'X',
            Value::Number(number) => ('0' as u8 + number) as char,
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_index() {
        for i in 0..13 {
            assert_eq!(i, Value::from_index(i).unwrap().to_index());
        }
    }

    #[test]
    fn round_trip_char() {
        for c in "23456789XJQKA".chars() {
            assert_eq!(c, Value::from_char(c).unwrap().to_char())
        }
    }

    #[test]
    fn ordering() {
        for i in 2..10 {
            assert!(Value::Number(i) < Value::Number(i + 1));
        }
        assert!(Value::Number(10) < Value::Jack);
        assert!(Value::Jack < Value::Queen);
        assert!(Value::Queen < Value::King);
        assert!(Value::King < Value::Ace);
    }
}

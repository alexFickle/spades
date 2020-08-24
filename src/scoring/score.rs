/// Represents a team's score.
///
/// TODO: implement add traits
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Score {
    tens: i64,
    extras: u8,
}

impl Score {
    /// Creates a new Score given the number of extras and net number of tricks.
    pub fn new(num_tens: i64, num_extras: u8) -> Self {
        let mut score = Score {
            tens: num_tens,
            extras: 0,
        };
        score.add_extras(num_extras);
        score
    }

    /// Adds some number to the tens position of this score.
    /// This is the number of tricks taken plus 10 for nil or 20 for blind nil.
    pub fn add_tens(&mut self, num_tens: u8) {
        self.tens += num_tens as i64;
    }

    /// Subtracts some number to the tens position of this score.
    /// This is the number of tricks taken plus 10 for nil or 20 for blind nil.
    pub fn sub_tens(&mut self, num_tens: u8) {
        self.tens -= num_tens as i64;
    }

    /// Adds a number of extras to this score.
    pub fn add_extras(&mut self, num_extras: u8) {
        self.extras += num_extras;
        while self.extras >= 10 {
            self.sub_tens(10);
            self.extras -= 10;
        }
    }

    /// Converts this score to an integer that can be displayed on a score board.
    pub fn to_display_int(self) -> i64 {
        self.tens * 10
            + if self.tens < 0 {
                -1 * self.extras as i64
            } else {
                self.extras as i64
            }
    }

    /// Gets the net number of tricks (adjusted by nils and groups of 10 extras) gotten by this score.
    pub fn get_tens(self) -> i64 {
        self.tens
    }

    /// Gets the number of extras in this score.
    pub fn get_extras(self) -> u8 {
        self.extras
    }
}

impl std::ops::AddAssign for Score {
    fn add_assign(&mut self, other: Self) {
        self.tens += other.tens;
        self.add_extras(other.extras);
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut copy = self;
        copy += other;
        copy
    }
}

#[cfg(test)]
mod test {
    use super::Score;

    #[test]
    fn default() {
        let score = Score::default();
        assert_eq!(0, score.get_extras());
        assert_eq!(0, score.get_tens());
    }

    #[test]
    fn positive() {
        let score = Score::new(20, 5);
        assert_eq!(205, score.to_display_int());
        assert_eq!(20, score.get_tens());
        assert_eq!(5, score.get_extras());
    }

    #[test]
    fn negative() {
        let score = Score::new(-20, 5);
        assert_eq!(-205, score.to_display_int());
        assert_eq!(-20, score.get_tens());
        assert_eq!(5, score.get_extras());
    }

    #[test]
    fn add_tens() {
        let mut score = Score::new(20, 5);
        score.add_tens(24);
        assert_eq!(44, score.get_tens());
        assert_eq!(5, score.get_extras());
    }

    #[test]
    fn sub_tens() {
        let mut score = Score::new(20, 5);
        score.sub_tens(24);
        assert_eq!(-4, score.get_tens());
        assert_eq!(5, score.get_extras());
    }

    #[test]
    fn add_extras() {
        let mut score = Score::new(20, 5);
        score.add_extras(3);
        assert_eq!(20, score.get_tens());
        assert_eq!(8, score.get_extras());
    }

    #[test]
    fn add_extras_overflow() {
        let mut score = Score::new(20, 5);
        score.add_extras(8);
        assert_eq!(10, score.get_tens());
        assert_eq!(3, score.get_extras());
    }

    #[test]
    fn add_extras_exact_overflow() {
        let mut score = Score::new(20, 5);
        score.add_extras(5);
        assert_eq!(10, score.get_tens());
        assert_eq!(0, score.get_extras());
    }
}

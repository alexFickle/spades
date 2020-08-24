use super::Player;

/// Iterator over players.
///
/// Iterates in the order of play with wrapping and without repetition
/// starting at a specified player.
///
/// Created with [`Player`]'s [`iter()`] function.
///
/// [`iter()`]: enum.Player.html#method.iter
/// [`Player`]: enum.Player.html
pub struct Iterator {
    start: Player,
    next: Option<Player>,
}

impl Iterator {
    /// Creates a new iterator at a given starting player.
    pub fn new(start: Player) -> Self {
        Self {
            start,
            next: Some(start),
        }
    }
}

impl std::iter::Iterator for Iterator {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;

        self.next = self.next.map(|x| x.next());
        if self.next == Some(self.start) {
            self.next = None;
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_one() {
        let mut iter = Iterator::new(Player::One);
        assert_eq!(iter.next(), Some(Player::One));
        assert_eq!(iter.next(), Some(Player::Two));
        assert_eq!(iter.next(), Some(Player::Three));
        assert_eq!(iter.next(), Some(Player::Four));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn start_two() {
        let mut iter = Iterator::new(Player::Two);
        assert_eq!(iter.next(), Some(Player::Two));
        assert_eq!(iter.next(), Some(Player::Three));
        assert_eq!(iter.next(), Some(Player::Four));
        assert_eq!(iter.next(), Some(Player::One));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn start_three() {
        let mut iter = Iterator::new(Player::Three);
        assert_eq!(iter.next(), Some(Player::Three));
        assert_eq!(iter.next(), Some(Player::Four));
        assert_eq!(iter.next(), Some(Player::One));
        assert_eq!(iter.next(), Some(Player::Two));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn start_four() {
        let mut iter = Iterator::new(Player::Four);
        assert_eq!(iter.next(), Some(Player::Four));
        assert_eq!(iter.next(), Some(Player::One));
        assert_eq!(iter.next(), Some(Player::Two));
        assert_eq!(iter.next(), Some(Player::Three));
        assert_eq!(iter.next(), None);
    }
}

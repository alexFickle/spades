//! Contains a set type for cards.

use super::{Card, Suite};

/// A set type for cards.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Set {
    int: u64,
}

/// Iterator over the cards in a set.
pub struct Iterator {
    int: u64,
}

impl Set {
    /// Creates a set containing every card.
    pub fn full() -> Self {
        Self {
            int: ((1 as u64) << 52) - 1,
        }
    }

    /// Creates a set containing every card of a suite.
    pub fn suite(suite: Suite) -> Self {
        // NOTE: this makes assumptions about card.to_index(),
        // is covered by tests
        Self {
            int: (((1 as u64) << 13) - 1) << (suite.to_index() as u64 * 13),
        }
    }

    /// Inserts a card into a set.
    ///
    /// Returns true if the insert took place.
    /// Returns false if nothing happened due to the card already being inserted.
    pub fn insert(&mut self, card: Card) -> bool {
        let mask = 1 << card.to_index() as u64;
        let contained = (self.int & mask) != 0;
        self.int |= mask;
        !contained
    }

    /// Removes a card from a set.
    ///
    /// Returns true if the removal took place.
    /// Returns false if nothing happened due to the card not being in the set.
    pub fn remove(&mut self, card: Card) -> bool {
        let mask = 1 << card.to_index() as u64;
        let contained = (self.int & mask) != 0;
        self.int &= !mask;
        contained
    }

    /// Removes every card from this set.
    pub fn clear(&mut self) {
        self.int = 0;
    }

    /// Gets if a card is in this set.
    pub fn contains(self, card: Card) -> bool {
        let mask = 1 << card.to_index() as u64;
        (self.int & mask) != 0
    }

    /// Gets if this set is empty.
    pub fn is_empty(self) -> bool {
        self.int == 0
    }

    /// Gets the number of cards in this set.
    pub fn len(self) -> usize {
        self.int.count_ones() as usize
    }

    /// Creates an iterator over all of the cards in this set.
    ///
    /// The iteration order is from lowest to highest index.
    /// This means that all cards in the same suite will be ordered from lowest value to highest value.
    pub fn iter(self) -> Iterator {
        Iterator { int: self.int }
    }
}

// debug printing
impl std::fmt::Debug for Set {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        f.debug_set().entries(self.iter()).finish()
    }
}

// inserting from an iterator, TODO: see if duplication for reference is needed
impl std::iter::Extend<Card> for Set {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        for card in iter {
            self.insert(card);
        }
    }
}
impl<'a> std::iter::Extend<&'a Card> for Set {
    fn extend<T: IntoIterator<Item = &'a Card>>(&mut self, iter: T) {
        for card in iter {
            self.insert(*card);
        }
    }
}

// creating from an iterator
impl std::iter::FromIterator<Card> for Set {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut set = Self::default();
        set.extend(iter);
        set
    }
}
impl<'a> std::iter::FromIterator<&'a Card> for Set {
    fn from_iter<I: IntoIterator<Item = &'a Card>>(iter: I) -> Self {
        let mut set = Self::default();
        set.extend(iter);
        set
    }
}

impl std::ops::BitOr for Set {
    type Output = Self;

    /// Set union operator.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output {
            int: self.int | rhs.int,
        }
    }
}

impl std::ops::BitAnd for Set {
    type Output = Self;

    /// Set intersection operator.
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output {
            int: self.int & rhs.int,
        }
    }
}

impl std::ops::Sub for Set {
    type Output = Self;

    /// Set difference operator.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            int: self.int & !rhs.int,
        }
    }
}

impl std::ops::Not for Set {
    type Output = Self;

    /// Set negation operator.
    fn not(self) -> <Self as std::ops::Not>::Output {
        Self::full() - self
    }
}

impl std::iter::Iterator for Iterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.int == 0 {
            None
        } else {
            let index = self.int.trailing_zeros() as u8;
            let mask = 1 << (index as u64);
            self.int &= !mask;
            Some(Card::from_index(index).unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Value;
    use super::*;

    #[test]
    fn default() {
        let set = Set::default();
        assert_eq!(0, set.len());
        assert!(set.is_empty());
    }

    #[test]
    fn full() {
        let set = Set::full();
        assert_eq!(52, set.len());
        for i in 0..52 {
            assert!(set.contains(Card::from_index(i).unwrap()));
        }
    }

    #[test]
    fn suite() {
        for i in 0..4 {
            let suite = Suite::from_index(i).unwrap();
            let set = Set::suite(suite);
            assert_eq!(13, set.len());
            for j in 0..13 {
                assert!(set
                    .contains(Card::new(suite, Value::from_index(j).unwrap())));
            }
        }
    }

    #[test]
    fn insert_one() {
        let mut set = Set::default();
        let card = Card::new(Suite::Spade, Value::King);
        assert!(set.insert(card));

        assert_eq!(1, set.len());
        assert!(!set.is_empty());
        assert!(set.contains(card));
    }

    #[test]
    fn insert_repeated() {
        let mut set = Set::default();
        let card = Card::new(Suite::Diamond, Value::Number(7));
        assert!(set.insert(card));
        assert!(!set.insert(card));

        assert_eq!(1, set.len());
        assert!(!set.is_empty());
        assert!(set.contains(card));
    }

    #[test]
    fn remove() {
        let mut set = Set::default();
        for i in 5..=7 {
            assert!(set.insert(Card::from_index(i).unwrap()));
        }
        assert!(set.remove(Card::from_index(6).unwrap()));
        assert!(!set.contains(Card::from_index(6).unwrap()));
        assert_eq!(2, set.len());
    }

    #[test]
    fn remove_missing() {
        let mut set = Set::default();
        assert!(!set.remove(Card::new(Suite::Spade, Value::Jack)));
        assert_eq!(0, set.len());
        assert!(set.is_empty());
    }

    #[test]
    fn remove_repeated() {
        let mut set = Set::full();

        let card = Card::from_index(20).unwrap();
        assert!(set.remove(card));
        assert!(!set.contains(card));
        assert_eq!(51, set.len());

        assert!(!set.remove(card));
        assert!(!set.contains(card));
        assert_eq!(51, set.len());
    }

    #[test]
    fn clear() {
        let mut set = Set::full();
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn iteration() {
        let mut hash_set = std::collections::HashSet::<Card>::default();
        for card in Set::full().iter() {
            assert!(hash_set.insert(card));
        }
        assert_eq!(52, hash_set.len());

        for _ in Set::default().iter() {
            unreachable!();
        }
    }

    #[test]
    fn extend() {
        let mut set = Set::default();
        set.extend([0, 1, 2, 0].iter().map(|x| Card::from_index(*x).unwrap()));
        assert_eq!(3, set.len());
        for i in 0..3 {
            assert!(set.contains(Card::from_index(i).unwrap()));
        }
    }

    #[test]
    fn collect() {
        let set: Set = [0, 1, 2, 0]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        assert_eq!(3, set.len());
        for i in 0..3 {
            assert!(set.contains(Card::from_index(i).unwrap()));
        }
    }

    #[test]
    fn equal() {
        let set1: Set = [1, 2, 3]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let set2 = Set::default();

        assert_eq!(set1, set1);
        assert_eq!(set2, set2);
        assert_ne!(set1, set2);
    }

    #[test]
    fn union() {
        let set1: Set = [1, 2, 3]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let set2: Set = [13, 1, 2]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let expected_result: Set = [1, 2, 3, 13]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        assert_eq!(set1 | set2, expected_result);
    }

    #[test]
    fn intersection() {
        let set1: Set = [1, 2, 3]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let set2: Set = [2, 3, 20]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let expected_result: Set = [2, 3]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        assert_eq!(set1 & set2, expected_result);
    }

    #[test]
    fn difference() {
        let set1: Set = [1, 2, 3, 4]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let set2: Set = [2, 3, 5]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        let expected_result: Set = [1, 4]
            .iter()
            .map(|x| Card::from_index(*x).unwrap())
            .collect();
        assert_eq!(set1 - set2, expected_result);
    }

    #[test]
    fn negate() {
        let mut set = Set::full();
        assert!(set.remove(Card::from_index(0).unwrap()));
        let negated = !set;
        assert_eq!(1, negated.len());
        assert!(negated.contains(Card::from_index(0).unwrap()));
    }
}

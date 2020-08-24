/// Internal module for querying information from bids related to scoring.
use crate::Bid;

/// Gets if this bid is a any kind of nil bid (nil or blind nil) or not.
pub fn is_any_nil(bid: Bid) -> bool {
    match bid {
        Bid::BlindNil => true,
        Bid::Nil => true,
        Bid::Take(_) => false,
    }
}

/// Gets the bonus value in equivalent number of tricks due to a bid being nil or blind nil.
pub fn nil_bonus(bid: Bid) -> u8 {
    match bid {
        Bid::BlindNil => 20,
        Bid::Nil => 10,
        Bid::Take(_) => 0,
    }
}

/// Gets the number of tricks that a bid will take.
pub fn num_tricks(bid: Bid) -> u8 {
    if let Bid::Take(count) = bid {
        count
    } else {
        0
    }
}

/// Gets the number of tricks that a team will take, taking into
/// account the minimum bid of four.
pub fn num_team_tricks(bid1: Bid, bid2: Bid) -> u8 {
    std::cmp::max(4, num_tricks(bid1) + num_tricks(bid2))
}

/// Gets the bonus value in equivalent number of tricks due to a team bidding at least 10 tricks.
pub fn high_trick_bonus(bid1: Bid, bid2: Bid) -> u8 {
    if num_tricks(bid1) + num_tricks(bid2) >= 10 {
        10
    } else {
        0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn blind_nil() {
        assert!(is_any_nil(Bid::BlindNil));
        assert_eq!(20, nil_bonus(Bid::BlindNil));
        assert_eq!(0, num_tricks(Bid::BlindNil));
    }

    #[test]
    fn nil() {
        assert!(is_any_nil(Bid::Nil));
        assert_eq!(10, nil_bonus(Bid::Nil));
        assert_eq!(0, num_tricks(Bid::Nil));
    }

    #[test]
    fn take0() {
        assert!(!is_any_nil(Bid::Take(0)));
        assert_eq!(0, nil_bonus(Bid::Take(0)));
        assert_eq!(0, num_tricks(Bid::Take(0)));
    }

    #[test]
    fn take3() {
        assert!(!is_any_nil(Bid::Take(3)));
        assert_eq!(0, nil_bonus(Bid::Take(3)));
        assert_eq!(3, num_tricks(Bid::Take(3)));
    }

    #[test]
    fn minimum_bid() {
        // ensure that the minimum bid is respected
        assert_eq!(4, num_team_tricks(Bid::Take(0), Bid::Take(1)));
        assert_eq!(4, num_team_tricks(Bid::Take(1), Bid::Take(2)));
        assert_eq!(4, num_team_tricks(Bid::Take(3), Bid::Take(0)));

        // ensure that over minimum passes through unchanged
        assert_eq!(5, num_team_tricks(Bid::Take(3), Bid::Take(2)));
        assert_eq!(5, num_team_tricks(Bid::Take(2), Bid::Take(3)));
        assert_eq!(5, num_team_tricks(Bid::Take(5), Bid::Take(0)));
    }

    #[test]
    fn ten_for_two() {
        // should get bonus
        assert_eq!(10, high_trick_bonus(Bid::Take(4), Bid::Take(6)));
        assert_eq!(10, high_trick_bonus(Bid::Take(7), Bid::Take(4)));

        // should not get bonus
        assert_eq!(0, high_trick_bonus(Bid::Take(3), Bid::Take(6)));
        assert_eq!(0, high_trick_bonus(Bid::Take(6), Bid::Take(3)));
    }
}

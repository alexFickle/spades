//! Contains functions and types that store and give information about team's scores.

mod score;
pub use score::Score;

pub(crate) mod bid;
pub use bid::Bid;

mod team_round_result;
pub use team_round_result::TeamRoundResult;

mod bid_util;

/// Gets the value of a team's bid.
///
/// This is how many points the team will make if they make their bet divided by 10.
/// For example, if a team bid 4 tricks total their value is 4.
/// If they bid 5 tricks and one player going nil their value is 15.
/// If a team bids less than 4 tricks then they effectively bid the minimum of 4.
pub fn get_bid_value(bid1: Bid, bid2: Bid) -> u8 {
    bid_util::num_team_tricks(bid1, bid2)
        + bid_util::nil_bonus(bid1)
        + bid_util::nil_bonus(bid2)
        + bid_util::high_trick_bonus(bid1, bid2)
}

/// Gets the index of the winning team.
///
/// Returns None if no team has won yet.
pub fn get_winning_team_index(scores: [Score; 2]) -> Option<u8> {
    // over 50 tens and more tens than opponent
    if scores[0].get_tens() >= 50 && scores[0].get_tens() > scores[1].get_tens()
    {
        return Some(0);
    }
    if scores[1].get_tens() >= 50 && scores[1].get_tens() > scores[0].get_tens()
    {
        return Some(1);
    }

    // mercy rule
    if scores[0].get_tens() - scores[1].get_tens() >= 50 {
        return Some(0);
    }
    if scores[1].get_tens() - scores[0].get_tens() >= 50 {
        return Some(1);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tricks_value() {
        assert_eq!(5, get_bid_value(Bid::Take(2), Bid::Take(3)));
        assert_eq!(6, get_bid_value(Bid::Take(0), Bid::Take(6)));
    }

    #[test]
    fn nil_value() {
        assert_eq!(4 + 10, get_bid_value(Bid::Take(4), Bid::Nil));
        assert_eq!(7 + 10, get_bid_value(Bid::Nil, Bid::Take(7)));
    }

    #[test]
    fn blind_nil_value() {
        assert_eq!(4 + 20, get_bid_value(Bid::Take(4), Bid::BlindNil));
        assert_eq!(6 + 20, get_bid_value(Bid::BlindNil, Bid::Take(6)));
    }

    #[test]
    fn ten_for_two_value() {
        assert_eq!(10 + 10, get_bid_value(Bid::Take(5), Bid::Take(5)));
        assert_eq!(11 + 10, get_bid_value(Bid::Take(6), Bid::Take(5)));
    }

    #[test]
    fn best_value() {
        assert_eq!(13 + 20 + 10, get_bid_value(Bid::BlindNil, Bid::Take(13)));
    }

    #[test]
    fn no_winner() {
        let scores_array = [
            (Score::default(), Score::default()),
            (Score::default(), Score::new(49, 9)),
            (Score::new(-5, 0), Score::new(44, 0)),
            (Score::new(51, 0), Score::new(51, 5)),
        ];

        for scores in scores_array.iter() {
            assert_eq!(None, get_winning_team_index([scores.0, scores.1]));
            assert_eq!(None, get_winning_team_index([scores.1, scores.0]));
        }
    }

    #[test]
    fn winner() {
        let winner_loser_array = [
            (Score::new(50, 0), Score::new(49, 5)),
            (Score::new(51, 5), Score::new(50, 0)),
            (Score::new(45, 5), Score::new(-5, 0)),
        ];

        for (winner, loser) in winner_loser_array.iter() {
            assert_eq!(Some(0), get_winning_team_index([*winner, *loser]));
            assert_eq!(Some(1), get_winning_team_index([*loser, *winner]));
        }
    }
}

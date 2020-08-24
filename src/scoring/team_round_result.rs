use super::bid_util;
use super::Score;
use crate::player;
use crate::Bid;
use crate::Player;

/// Contains a team's bid and number of tricks taken in a round.
///
/// Is a building block of ScoreBoard.
#[derive(Copy, Clone, Debug)]
pub struct TeamRoundResult {
    /// The bids of each player in a team for a round.
    pub bids: [Bid; 2],
    /// The number of tricks taken by each player in a team in a round.
    pub tricks_taken: [u8; 2],
}

impl TeamRoundResult {
    /// Creates a pair of TeamRoundResult from an array of bids
    /// and the number of tricks taken.
    ///
    /// TODO: test
    pub fn create_pair(
        bids: player::Array<Bid>,
        tricks_taken: player::Array<u8>,
    ) -> [Self; 2] {
        [
            Self {
                bids: [bids[Player::One], bids[Player::Three]],
                tricks_taken: [
                    tricks_taken[Player::One],
                    tricks_taken[Player::Three],
                ],
            },
            Self {
                bids: [bids[Player::Two], bids[Player::Four]],
                tricks_taken: [
                    tricks_taken[Player::Two],
                    tricks_taken[Player::Four],
                ],
            },
        ]
    }

    /// Gets the change in score caused by this round.
    pub fn get_score(&self) -> Score {
        let tricks_taken = self.tricks_taken[0] + self.tricks_taken[1];
        let tricks_required =
            bid_util::num_team_tricks(self.bids[0], self.bids[1]);

        let failed = (tricks_taken < tricks_required)
            || (bid_util::is_any_nil(self.bids[0])
                && self.tricks_taken[0] != 0)
            || (bid_util::is_any_nil(self.bids[1])
                && self.tricks_taken[1] != 0);
        let value = super::get_bid_value(self.bids[0], self.bids[1]);

        let mut score = Score::default();
        if failed {
            score.sub_tens(value);
        } else {
            score.add_tens(value);
        }
        if tricks_taken > tricks_required {
            score.add_extras(tricks_taken - tricks_required);
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn win() {
        let result = TeamRoundResult {
            bids: [Bid::Take(3), Bid::Take(2)],
            tricks_taken: [1, 4],
        };
        let score = result.get_score();

        assert_eq!(5, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn win_with_extras() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::Take(0)],
            tricks_taken: [3, 2],
        };
        let score = result.get_score();

        assert_eq!(4, score.get_tens());
        assert_eq!(1, score.get_extras());
    }

    #[test]
    fn nil_win() {
        let result = TeamRoundResult {
            bids: [Bid::Nil, Bid::Take(5)],
            tricks_taken: [0, 5],
        };
        let score = result.get_score();

        assert_eq!(15, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn nil_win_with_extras() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::Nil],
            tricks_taken: [6, 0],
        };
        let score = result.get_score();

        assert_eq!(14, score.get_tens());
        assert_eq!(2, score.get_extras());
    }

    #[test]
    fn blind_nil_win() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::BlindNil],
            tricks_taken: [4, 0],
        };
        let score = result.get_score();

        assert_eq!(24, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn ten_for_two_win() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::Take(6)],
            tricks_taken: [6, 5],
        };
        let score = result.get_score();

        assert_eq!(20, score.get_tens());
        assert_eq!(1, score.get_extras());
    }

    #[test]
    fn lose() {
        let result = TeamRoundResult {
            bids: [Bid::Take(3), Bid::Take(2)],
            tricks_taken: [2, 2],
        };
        let score = result.get_score();

        assert_eq!(-5, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn nil_lose_due_to_take() {
        let result = TeamRoundResult {
            bids: [Bid::Nil, Bid::Take(4)],
            tricks_taken: [1, 3],
        };
        let score = result.get_score();

        assert_eq!(-14, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn nil_lose_due_to_not_take() {
        let result = TeamRoundResult {
            bids: [Bid::Nil, Bid::Take(5)],
            tricks_taken: [0, 4],
        };
        let score = result.get_score();

        assert_eq!(-15, score.get_tens());
        assert_eq!(0, score.get_extras());
    }

    #[test]
    fn nil_lose_with_extra() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::Nil],
            tricks_taken: [4, 1],
        };
        let score = result.get_score();

        assert_eq!(-14, score.get_tens());
        assert_eq!(1, score.get_extras());
    }

    #[test]
    fn blind_nil_lose() {
        let result = TeamRoundResult {
            bids: [Bid::Take(4), Bid::BlindNil],
            tricks_taken: [4, 3],
        };
        let score = result.get_score();

        assert_eq!(-24, score.get_tens());
        assert_eq!(3, score.get_extras());
    }

    #[test]
    fn ten_for_two_lose() {
        let result = TeamRoundResult {
            bids: [Bid::Take(7), Bid::Take(3)],
            tricks_taken: [6, 3],
        };
        let score = result.get_score();

        assert_eq!(-20, score.get_tens());
        assert_eq!(0, score.get_extras());
    }
}

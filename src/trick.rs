//! Contains the `Trick` struct and related `Status` enum.

use crate::card::{self, Card, Suite};
use crate::{player, Player};

/// Contains all of the currently played cards and the starting player.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Trick {
    start_player: Player,
    cards: player::Array<Option<Card>>,
}

/// The status of the trick.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    /// The trick is waiting for a player to play a card.
    Waiting(Player),
    /// The trick has ended, a has won.
    Won(Player, Card),
}

impl Trick {
    /// Creates a new trick from the starting player.
    pub fn new(start_player: Player) -> Self {
        Self {
            start_player,
            cards: player::Array::default(),
        }
    }

    /// Gets the status of this trick.
    pub fn get_status(&self) -> Status {
        // see if we are waiting for a card to be played
        for player in self.start_player.iter() {
            println!("{:#?}", player);
            if self.cards[player].is_none() {
                return Status::Waiting(player);
            }
        }
        // find the winner
        let mut winner =
            (self.start_player, self.cards[self.start_player].unwrap());
        for player in self.start_player.iter().skip(1) {
            let card = self.cards[player].unwrap();
            if card.suite == winner.1.suite {
                if card.value > winner.1.value {
                    winner = (player, card);
                }
            } else if card.suite == Suite::Spade {
                winner = (player, card);
            }
        }
        Status::Won(winner.0, winner.1)
    }

    /// Gets the suite that lead this trick.
    /// If no cards have been played returns None.
    pub fn get_suite(&self) -> Option<Suite> {
        self.cards[self.start_player].map(|card| card.suite)
    }

    /// Gets the card played by a player.
    pub fn get_card(&self, player: Player) -> Option<Card> {
        self.cards[player]
    }

    /// Attempts to play a card as a player.
    /// Checks that it is actually this player's turn.
    pub fn play_card(
        &mut self,
        player: Player,
        card: Card,
    ) -> Result<(), String> {
        match self.get_status() {
            Status::Won(_, _) => {
                Err("Can not play a card into a trick that is already won."
                    .to_string())
            }
            Status::Waiting(expected_player) => {
                if expected_player != player {
                    Err("Can not play a card when it is not your turn"
                        .to_string())
                } else {
                    self.cards[player] = Some(card);
                    Ok(())
                }
            }
        }
    }

    /// Takes a player's hand and returns all cards that the
    /// player may play assuming that it is currently their turn.
    pub fn get_playable_cards(
        &self,
        hand: card::Set,
        is_trump_broken: bool,
    ) -> card::Set {
        if let Some(suite) = self.get_suite() {
            let same_suite = hand & card::Set::suite(suite);
            if same_suite.is_empty() {
                // can not follow suite, may play any card
                hand
            } else {
                // must follow suite
                same_suite
            }
        } else {
            // lead player
            let non_spades = hand & !card::Set::suite(Suite::Spade);
            if is_trump_broken || non_spades.is_empty() {
                // can lead any card, including trump cards
                hand
            } else {
                // can only lead with non-trump cards
                non_spades
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Value;

    #[test]
    fn high_card_wins() {
        let mut trick = Trick::new(Player::One);
        trick
            .play_card(Player::One, Card::new(Suite::Heart, Value::Number(5)))
            .unwrap();
        trick
            .play_card(Player::Two, Card::new(Suite::Heart, Value::Number(8)))
            .unwrap();
        trick
            .play_card(Player::Three, Card::new(Suite::Heart, Value::Queen))
            .unwrap();
        trick
            .play_card(Player::Four, Card::new(Suite::Heart, Value::Number(4)))
            .unwrap();

        assert_eq!(
            trick.get_status(),
            Status::Won(Player::Three, Card::new(Suite::Heart, Value::Queen))
        )
    }

    #[test]
    fn off_suite_never_wins() {
        let mut trick = Trick::new(Player::One);
        trick
            .play_card(Player::One, Card::new(Suite::Heart, Value::Number(2)))
            .unwrap();
        trick
            .play_card(
                Player::Two,
                Card::new(Suite::Diamond, Value::Number(10)),
            )
            .unwrap();
        trick
            .play_card(Player::Three, Card::new(Suite::Club, Value::Ace))
            .unwrap();
        trick
            .play_card(Player::Four, Card::new(Suite::Heart, Value::Number(3)))
            .unwrap();

        assert_eq!(
            trick.get_status(),
            Status::Won(
                Player::Four,
                Card::new(Suite::Heart, Value::Number(3))
            )
        );
    }

    #[test]
    fn spades_win() {
        let mut trick = Trick::new(Player::One);
        trick
            .play_card(Player::One, Card::new(Suite::Club, Value::Number(10)))
            .unwrap();
        trick
            .play_card(Player::Two, Card::new(Suite::Club, Value::Number(10)))
            .unwrap();
        trick
            .play_card(Player::Three, Card::new(Suite::Spade, Value::Number(5)))
            .unwrap();
        trick
            .play_card(Player::Four, Card::new(Suite::Spade, Value::Number(3)))
            .unwrap();

        assert_eq!(
            trick.get_status(),
            Status::Won(
                Player::Three,
                Card::new(Suite::Spade, Value::Number(5))
            )
        );
    }

    #[test]
    fn waits_for_player() {
        let mut trick = Trick::new(Player::Three);
        assert_eq!(trick.get_status(), Status::Waiting(Player::Three));

        trick
            .play_card(
                Player::Three,
                Card::new(Suite::Diamond, Value::Number(2)),
            )
            .unwrap();
        assert_eq!(trick.get_status(), Status::Waiting(Player::Four));

        trick
            .play_card(Player::Four, Card::new(Suite::Diamond, Value::Ace))
            .unwrap();
        assert_eq!(trick.get_status(), Status::Waiting(Player::One));

        trick
            .play_card(Player::One, Card::new(Suite::Club, Value::Number(3)))
            .unwrap();
        assert_eq!(trick.get_status(), Status::Waiting(Player::Two));
    }

    #[test]
    fn get_suite() {
        let mut trick = Trick::new(Player::One);
        assert!(trick.get_suite().is_none());

        trick
            .play_card(Player::One, Card::new(Suite::Diamond, Value::Ace))
            .unwrap();
        assert_eq!(trick.get_suite(), Some(Suite::Diamond));

        trick
            .play_card(Player::Two, Card::new(Suite::Spade, Value::Ace))
            .unwrap();
        assert_eq!(trick.get_suite(), Some(Suite::Diamond));
    }

    #[test]
    fn wrong_player_fails() {
        for start_player in Player::One.iter() {
            let mut trick = Trick::new(start_player);
            for wrong_player in start_player.iter().skip(1) {
                assert!(trick
                    .play_card(
                        wrong_player,
                        Card::new(Suite::Spade, Value::Ace)
                    )
                    .is_err());
            }
        }
    }

    #[test]
    fn fifth_play_fails() {
        for start_player in Player::One.iter() {
            let mut trick = Trick::new(start_player);

            // fill up the trick
            for player in start_player.iter() {
                trick
                    .play_card(
                        player,
                        Card::new(
                            Suite::from_index(player.to_index()).unwrap(),
                            Value::Ace,
                        ),
                    )
                    .unwrap();
            }

            // any play should fail
            for player in start_player.iter() {
                assert!(trick
                    .play_card(
                        player,
                        Card::new(Suite::Spade, Value::Number(2))
                    )
                    .is_err());
            }
        }
    }

    #[test]
    fn get_card() {
        let mut trick = Trick::new(Player::One);
        for player in Player::One.iter() {
            assert_eq!(trick.get_card(player), None);
        }

        let card = Card::new(Suite::Spade, Value::Ace);
        trick.play_card(Player::One, card).unwrap();
        assert_eq!(trick.get_card(Player::One), Some(card));
        for player in Player::One.iter().skip(1) {
            assert_eq!(trick.get_card(player), None);
        }
    }

    #[test]
    fn leading_trump() {
        let trick = Trick::new(Player::One);
        let spades: card::Set = [
            Card::new(Suite::Spade, Value::Number(2)),
            Card::new(Suite::Spade, Value::Ace),
        ]
        .iter()
        .collect();
        let non_spades: card::Set = [
            Card::new(Suite::Heart, Value::Number(2)),
            Card::new(Suite::Club, Value::King),
        ]
        .iter()
        .collect();

        // can always play spades if they only have spades
        assert_eq!(spades, trick.get_playable_cards(spades, false));
        assert_eq!(spades, trick.get_playable_cards(spades, true));

        // can not play spades if they have non-spades and trump is not broken
        assert_eq!(
            non_spades,
            trick.get_playable_cards(spades | non_spades, false)
        );

        // can play any card if spades are broken
        assert_eq!(
            spades | non_spades,
            trick.get_playable_cards(spades | non_spades, true)
        );
    }

    #[test]
    fn following_suite() {
        let mut trick = Trick::new(Player::One);
        trick
            .play_card(Player::One, Card::new(Suite::Heart, Value::Ace))
            .unwrap();
        let hearts: card::Set = [
            Card::new(Suite::Heart, Value::Number(2)),
            Card::new(Suite::Heart, Value::King),
        ]
        .iter()
        .collect();
        let non_hearts: card::Set = [
            Card::new(Suite::Club, Value::Ace),
            Card::new(Suite::Diamond, Value::Jack),
            Card::new(Suite::Spade, Value::Number(7)),
        ]
        .iter()
        .collect();

        // if they have hearts must play hearts
        assert_eq!(hearts, trick.get_playable_cards(hearts | non_hearts, true));
        assert_eq!(
            hearts,
            trick.get_playable_cards(hearts | non_hearts, false)
        );

        // if they have no hearts can play anything
        assert_eq!(non_hearts, trick.get_playable_cards(non_hearts, true));
        assert_eq!(non_hearts, trick.get_playable_cards(non_hearts, false));
    }
}

//! Contains a trait for dealing cards to players and a default implementation.

use crate::{card, player, Player};

/// Trait for creating each player's hand.
///
/// Is split off to allow for dependency injection.
pub trait Dealer {
    /// Creates each player's hand.
    fn deal_cards(&mut self) -> player::Array<card::Set>;
}

/// Default implementation of the Dealer Trait.
///
/// Uses card::make_shuffled() to deal cards.
#[derive(Default)]
pub struct ShuffledDealer {}

impl Dealer for ShuffledDealer {
    fn deal_cards(&mut self) -> player::Array<card::Set> {
        let mut hands = player::Array::<card::Set>::default();
        let mut player = Player::One;
        for card in card::make_shuffled().iter() {
            hands[player].insert(*card);
            player = player.next();
        }
        hands
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shuffled() {
        let mut dealer = Box::new(ShuffledDealer::default());
        for _ in 0..10 {
            let hands = dealer.deal_cards();
            for player in Player::One.iter() {
                assert_eq!(13, hands[player].len());
                for other_player in player.iter().skip(1) {
                    let intersection = hands[player] & hands[other_player];
                    assert!(intersection.is_empty());
                }
            }
        }
    }
}

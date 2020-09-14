/// A player's bid.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Bid {
    /// A player must take no tricks.  They decided before they saw their cards.
    BlindNil,
    /// A player must take no tricks.  They decided after they saw their cards.
    Nil,
    /// The player claims that they will take the wrapper number of tricks.
    Take(u8),
}

/// Iterates through every possible bid in an arbitrary order.
pub(crate) struct Generator {
    next: Bid,
}

impl Bid {
    /// Gets the reason why this bid can not be played with another bid.
    pub(crate) fn get_compatibility_error(
        self,
        teammate_bid: Option<Bid>,
    ) -> Option<&'static str> {
        if let Some(teammate_bid_) = teammate_bid {
            if super::bid_util::is_any_nil(teammate_bid_)
                && super::bid_util::is_any_nil(self)
            {
                return Some(
                    "Both players in team can not bid nil or blind nil.",
                );
            }
        }
        if let Bid::Take(tricks_claimed) = self {
            let team_tricks_claimed = tricks_claimed
                + if let Some(Bid::Take(count)) = teammate_bid {
                    count
                } else {
                    0
                };
            if team_tricks_claimed > 13 {
                return Some("Can not bid more than 13 tricks as a team.");
            }
        }
        None
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            next: Bid::BlindNil,
        }
    }
}

impl Iterator for Generator {
    type Item = Bid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == Bid::Take(14) {
            None
        } else {
            let current = self.next;
            self.next = match current {
                Bid::BlindNil => Bid::Nil,
                Bid::Nil => Bid::Take(0),
                Bid::Take(n) => Bid::Take(n + 1),
            };
            Some(current)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator_len() {
        assert_eq!(Generator::default().count(), 16);
    }

    #[test]
    fn generator_yields_blind_nil() {
        assert!(Generator::default().find(|x| *x == Bid::BlindNil).is_some());
    }

    #[test]
    fn generator_yields_nil() {
        assert!(Generator::default().find(|x| *x == Bid::Nil).is_some());
    }

    #[test]
    fn generator_yields_takes() {
        for i in 0..=13 {
            assert!(Generator::default()
                .find(|x| *x == Bid::Take(i))
                .is_some());
        }
    }

    #[test]
    fn generator_does_not_yield_out_of_bounds() {
        assert!(Generator::default()
            .find(|x| if let Bid::Take(amount) = *x {
                if amount > 13 {
                    true
                } else {
                    false
                }
            } else {
                false
            })
            .is_none());
    }

    #[test]
    fn no_compatibility_error_with_nothing() {
        for bid in Generator::default() {
            assert!(bid.get_compatibility_error(None).is_none())
        }
    }

    #[test]
    fn all_nils_incompatible_with_each_other() {
        let nils = [Bid::BlindNil, Bid::Nil];

        for nil1 in nils.iter().copied() {
            for nil2 in nils.iter().copied() {
                assert!(nil1.get_compatibility_error(Some(nil2)).is_some());
            }
        }
    }

    #[test]
    fn bid_sum_at_most_13() {
        // fine
        for i in 0..=13 {
            assert!(Bid::Take(i)
                .get_compatibility_error(Some(Bid::Take(13 - i)))
                .is_none());
        }
        assert!(Bid::Take(13)
            .get_compatibility_error(Some(Bid::Nil))
            .is_none());
        assert!(Bid::Take(13)
            .get_compatibility_error(Some(Bid::BlindNil))
            .is_none());

        // error
        assert!(Bid::Take(14).get_compatibility_error(None).is_some());
        assert!(Bid::Take(0)
            .get_compatibility_error(Some(Bid::Take(14)))
            .is_some());
        for i in 0..=13 {
            assert!(Bid::Take(i)
                .get_compatibility_error(Some(Bid::Take(14 - i)))
                .is_some());
        }
    }

    #[test]
    fn bid_compatibility_commutative() {
        for bid1 in Generator::default() {
            for bid2 in Generator::default() {
                assert_eq!(
                    bid1.get_compatibility_error(Some(bid2)),
                    bid2.get_compatibility_error(Some(bid1))
                );
            }
        }
    }
}

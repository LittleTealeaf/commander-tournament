use core::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign},
};
use std::collections::HashMap;

use crate::{
    Tournament,
    error::TournamentError,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
)]
pub struct MatchPerformance {
    played: usize,
    won: usize,
    lost: usize,
}

impl PartialOrd for MatchPerformance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MatchPerformance {
    fn cmp(&self, other: &Self) -> Ordering {
        // // First, person with the highest win-rate
        let left = self.wins() * other.played();
        let right = other.wins() * self.played();
        let cmp = left.cmp(&right);

        cmp.then_with(|| {
            // Second, person with the lowest lose-rate
            let left = self.losses() * other.played();
            let right = other.losses() * self.played();
            let cmp = left.cmp(&right).reverse();

            cmp.then_with(|| {
                // Third, the person with the highest wins
                self.wins().cmp(&other.wins())
            })
        })
    }
}

impl Add<Self> for MatchPerformance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            played: self.played + rhs.played,
            won: self.won + rhs.won,
            lost: self.lost + rhs.lost,
        }
    }
}

impl AddAssign<Self> for MatchPerformance {
    fn add_assign(&mut self, rhs: Self) {
        self.played += rhs.played;
        self.won += rhs.won;
        self.lost += rhs.lost;
    }
}

impl Sum for MatchPerformance {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl MatchPerformance {
    #[must_use]
    pub const fn played(&self) -> usize {
        self.played
    }

    #[must_use]
    pub const fn wins(&self) -> usize {
        self.won
    }

    #[must_use]
    pub const fn losses(&self) -> usize {
        self.lost
    }

    #[must_use]
    pub const fn draws(&self) -> usize {
        self.played - (self.won + self.lost)
    }

    pub(crate) const fn add_draw(&mut self) {
        self.played += 1;
    }

    pub(crate) const fn add_win(&mut self) {
        self.played += 1;
        self.won += 1;
    }

    pub(crate) const fn add_loss(&mut self) {
        self.played += 1;
        self.lost += 1;
    }
}

impl Tournament {
    fn internal_player_get_player_match_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        self.require_id_registered(id)?;

        let mut records = self
            .get_registered_players()
            .map(|value| (value.id(), (value, MatchPerformance::default())))
            .collect::<HashMap<_, _>>();

        for game in self.get_player_games(id)? {
            let winner = game.winner();
            let losers = game.losers();

            if winner == id {
                for player in losers {
                    records.entry(player).and_modify(|(_, perf)| {
                        if player == id {
                            perf.add_loss();
                        }
                        perf.add_win();
                    });
                }
            } else {
                records.entry(winner).and_modify(|(_, perf)| {
                    perf.add_loss();
                });
                for player in losers {
                    if player == id {
                        continue;
                    }
                    records.entry(player).and_modify(|(_, perf)| {
                        perf.add_draw();
                    });
                }
            }
        }

        Ok(records.into_values())
    }

    pub fn get_player_player_match_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        Ok(self
            .internal_player_get_player_match_performance(id)?
            .filter(move |(player, _)| player.id() != id))
    }

    pub fn get_player_identity_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        let players = self.internal_player_get_player_match_performance(id)?;

        let mut identities = HashMap::from(
            ColorIdentity::IDENTITIES.map(|identity| (identity, MatchPerformance::default())),
        );

        for (player, performance) in players {
            identities
                .entry(*player.info().color_identity())
                .and_modify(|entry| {
                    *entry += performance;
                });
        }

        Ok(identities)
    }

    pub fn get_player_color_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        let players = self.internal_player_get_player_match_performance(id)?;

        let mut colors =
            HashMap::from_iter(MtgColor::COLORS.map(|color| (color, MatchPerformance::default())));

        for (player, performance) in players {
            for color in player.info().colors() {
                colors.entry(color).and_modify(|entry| {
                    *entry += performance;
                });
            }
        }

        Ok(colors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod match_performance {
        use super::*;

        #[test]
        fn highest_wr_accounted_first() {
            let left = MatchPerformance {
                played: 10,
                won: 5,
                lost: 4,
            };

            let right = MatchPerformance {
                played: 10,
                won: 4,
                lost: 4,
            };

            assert_eq!(Ordering::Greater, left.cmp(&right));
        }

        #[test]
        fn highest_loss_rate_accounted_second() {
            let left = MatchPerformance {
                played: 10,
                won: 5,
                lost: 4,
            };

            let right = MatchPerformance {
                played: 10,
                won: 5,
                lost: 3,
            };

            assert_eq!(Ordering::Less, left.cmp(&right));
        }

        #[test]
        fn highest_win_count_accounted_third() {
            let left = MatchPerformance {
                played: 10,
                won: 5,
                lost: 5,
            };
            let right = MatchPerformance {
                played: 20,
                won: 10,
                lost: 10,
            };

            assert_eq!(Ordering::Less, left.cmp(&right));
        }
    }

    #[test]
    fn player_player_does_not_return_self() {
        for tourn in Tournament::test_tournaments() {
            for id in tourn.players().keys() {
                let values = tourn.get_player_player_match_performance(*id).unwrap();
                for (player, _) in values {
                    assert_ne!(*id, player.id());
                }
            }
        }
    }

    #[test]
    fn player_identity_returns_all_identities() {
        for tourn in Tournament::test_tournaments() {
            for player in tourn.players().keys() {
                let values = tourn
                    .get_player_identity_match_performance(*player)
                    .unwrap();
                for identity in ColorIdentity::IDENTITIES {
                    assert!(values.contains_key(&identity));
                }
            }
        }
    }

    #[test]
    fn player_color_returns_all_colors() {
        for tourn in Tournament::test_tournaments() {
            for player in tourn.players().keys() {
                let values = tourn.get_player_color_match_performance(*player).unwrap();
                for identity in MtgColor::COLORS {
                    assert!(values.contains_key(&identity));
                }
            }
        }
    }
}

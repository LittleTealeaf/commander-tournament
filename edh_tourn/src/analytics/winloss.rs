use core::{
    iter::Sum,
    ops::{Add, AddAssign},
};
use std::collections::HashMap;

use itertools::Itertools;

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
    pub fn player_get_player_match_performance(
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
        records.remove(&id);

        Ok(records.into_values())
    }

    pub fn player_get_identity_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        let players = self.player_get_player_match_performance(id)?;
        let identities = players.map(|(player, perf)| (*player.info().color_identity(), perf));
        let aggregated = identities.into_grouping_map().sum();
        Ok(aggregated)
    }

    pub fn player_get_color_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        let players = self.player_get_player_match_performance(id)?;
        let colors = players.flat_map(|(player, perf)| {
            player
                .info()
                .color_identity()
                .into_colors()
                .map(move |color| (color, perf))
        });
        let aggregated = colors.into_grouping_map().sum();
        Ok(aggregated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_does_not_return_self() {
        for tourn in Tournament::test_tournaments() {
            for id in tourn.players().keys() {
                let iter = tourn.player_get_player_match_performance(*id).unwrap();
                for (player, _) in iter {
                    assert_ne!(*id, player.id());
                }
            }
        }
    }
}

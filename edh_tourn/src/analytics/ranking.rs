pub mod games_played;
pub mod neighbors;

use core::{cmp::Ordering, fmt::Display};

use itertools::{Itertools, chain};

use crate::{
    Tournament,
    error::TournamentError,
    player::{RegisteredPlayer, stats::PlayerStats},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RankingMethod {
    LeastPlayed,
    LostWith,
    Nemesis,
    EloNeighbors,
    WRNeighbors,
    ExpectedNeighbors,
    #[default]
    Combined,
}

impl RankingMethod {
    pub const VALUES: [Self; 7] = [
        Self::Combined,
        Self::LeastPlayed,
        Self::Nemesis,
        Self::LostWith,
        Self::EloNeighbors,
        Self::WRNeighbors,
        Self::ExpectedNeighbors,
    ];
}

impl Display for RankingMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::LeastPlayed => "Least Played",
            Self::LostWith => "Lost With",
            Self::Nemesis => "Nemesis",
            Self::EloNeighbors => "Elo Neighbors",
            Self::WRNeighbors => "WR Neighbors",
            Self::ExpectedNeighbors => "Expected Neighbors",
            Self::Combined => "Combined",
        })
    }
}

#[allow(clippy::cast_precision_loss)]
fn to_weight_rank<T>(
    ranking: impl IntoIterator<Item = T>,
    weight: f64,
) -> impl Iterator<Item = (T, f64)> {
    ranking
        .into_iter()
        .enumerate()
        .map(move |(score, val)| (val, (score as u64) as f64 * weight))
}

impl Tournament {
    fn get_elo(&self, id: u32) -> f64 {
        self.get_player_stats(id)
            .map_or(self.config.starting_elo, PlayerStats::elo)
    }

    fn get_wr(&self, id: u32) -> f64 {
        self.get_player_stats(id)
            .and_then(PlayerStats::wr)
            .unwrap_or(0.25)
    }

    pub fn get_player_ranked_combined(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        self.require_id_registered(id)?;

        let games_played_ranked = chain!(
            to_weight_rank(
                self.get_player_ranked_least_played(id)?,
                self.config.match_weight_least_played
            ),
            to_weight_rank(
                self.get_player_ranked_nemesis(id)?,
                self.config.match_weight_nemesis
            ),
            to_weight_rank(
                self.get_player_ranked_lost_with(id)?,
                self.config.match_weight_lost_with
            ),
        )
        .map(|((player, _), score)| (player, score));

        let neighbors_ranked = chain!(
            to_weight_rank(
                self.get_player_ranked_elo_neighbors(id)?,
                self.config.match_weight_elo_neighbor
            ),
            to_weight_rank(
                self.get_player_ranked_wr_neighbors(id)?,
                self.config.match_weight_wr_neighbor
            ),
            to_weight_rank(
                self.get_player_ranked_expected_neighbors(id)?,
                self.config.match_weight_expected_neighbor
            ),
        );

        let combined = chain!(games_played_ranked, neighbors_ranked)
            .map(|(player, score)| (player.id(), score));

        let scores = combined.into_grouping_map().sum();

        let players = scores
            .into_iter()
            .filter_map(|(id, score)| Some((self.get_registered_player(id).ok()?, score)));

        let sorted = players.sorted_by(|(player_a, score_a), (player_b, score_b)| {
            match score_a.total_cmp(score_b) {
                Ordering::Equal => player_a.id().cmp(&player_b.id()),
                cmp => cmp,
            }
        });

        Ok(sorted.map(|(player, _)| player))
    }

    pub fn get_player_ranked(
        &self,
        id: u32,
        method: RankingMethod,
    ) -> Result<Vec<RegisteredPlayer<'_>>, TournamentError> {
        Ok(match method {
            RankingMethod::LeastPlayed => self
                .get_player_ranked_least_played(id)?
                .map(|(player, _)| player)
                .collect(),
            RankingMethod::LostWith => self
                .get_player_ranked_lost_with(id)?
                .map(|(player, _)| player)
                .collect(),
            RankingMethod::Nemesis => self
                .get_player_ranked_nemesis(id)?
                .map(|(player, _)| player)
                .collect(),
            RankingMethod::EloNeighbors => self.get_player_ranked_elo_neighbors(id)?.collect(),
            RankingMethod::WRNeighbors => self.get_player_ranked_wr_neighbors(id)?.collect(),
            RankingMethod::ExpectedNeighbors => {
                self.get_player_ranked_expected_neighbors(id)?.collect()
            }
            RankingMethod::Combined => self.get_player_ranked_combined(id)?.collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_does_not_return_self() {
        for tourn in Tournament::test_tournaments() {
            for id in tourn.players().keys().copied() {
                for player in tourn
                    .get_player_ranked_combined(id)
                    .expect("Expected Player to Exist")
                {
                    assert_ne!(id, player.id(), "Player found in function output");
                }
            }
        }
    }
}

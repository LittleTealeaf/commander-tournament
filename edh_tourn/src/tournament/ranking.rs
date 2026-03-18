pub mod games_played;
pub mod neighbors;

use itertools::{Itertools, chain};

use crate::{
    error::TournamentError, player::RegisteredPlayer, ranking::RankingMethod,
    tournament::Tournament,
};

fn to_weight_rank<T>(
    ranking: impl IntoIterator<Item = T>,
    weight: usize,
) -> impl Iterator<Item = (T, usize)> {
    ranking
        .into_iter()
        .enumerate()
        .map(move |(score, val)| (val, score * weight))
}

impl Tournament {
    fn get_player_games_played_ranked_combined(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, usize)>, TournamentError> {
        Ok(chain!(
            to_weight_rank(
                self.get_player_ranked_least_played(id)?,
                self.ranking_config().least_played
            ),
            to_weight_rank(
                self.get_player_ranked_nemesis(id)?,
                self.ranking_config().nemesis
            ),
            to_weight_rank(
                self.get_player_ranked_lost_with(id)?,
                self.ranking_config().lost_with
            ),
        )
        .map(|((player, _), score)| (player, score)))
    }

    fn get_player_neigbhors_ranked_combined(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, usize)>, TournamentError> {
        Ok(chain!(
            to_weight_rank(
                self.get_player_ranked_elo_neighbors(id)?,
                self.ranking_config().elo_neighbor
            ),
            to_weight_rank(
                self.get_player_ranked_wr_neighbors(id)?,
                self.ranking_config().wr_neighbor
            ),
            to_weight_rank(
                self.get_player_ranked_expected_neighbors(id)?,
                self.ranking_config().expected_neighbor
            ),
        ))
    }

    pub fn get_player_ranked_combined(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        self.require_id_registered(id)?;

        let games_played_ranked = self.get_player_games_played_ranked_combined(id)?;
        let neighbors_ranked = self.get_player_neigbhors_ranked_combined(id)?;
        let combined = chain!(games_played_ranked, neighbors_ranked);

        let grouped = combined.into_grouping_map_by(|(player, _)| player.id());
        let scores = grouped.aggregate(|acc, _, (player, score)| {
            Some((player, acc.map_or(0, |(_, val)| val) + score))
        });

        let players = scores.into_values();
        let sorted = players.sorted_by(|(player_a, score_a), (player_b, score_b)| {
            score_a
                .cmp(score_b)
                .then_with(|| player_a.id().cmp(&player_b.id()))
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

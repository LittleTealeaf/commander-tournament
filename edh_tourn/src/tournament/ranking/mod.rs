pub mod games_played;
pub mod neighbors;

use itertools::{Itertools, chain};

use crate::{
    error::TournamentError,
    player::{PlayerId, RegisteredPlayer},
    ranking::RankingMethod,
    tournament::Tournament,
};

#[derive(Debug, Clone, Copy)]
pub struct Ranking<'a>(&'a Tournament);

impl Tournament {
    #[must_use]
    pub const fn ranking(&self) -> Ranking<'_> {
        Ranking(self)
    }
}

fn to_weight_rank<T>(
    ranking: impl IntoIterator<Item = T>,
    weight: usize,
) -> impl Iterator<Item = (T, usize)> {
    ranking
        .into_iter()
        .enumerate()
        .map(move |(score, val)| (val, score * weight))
}

impl<'a> Ranking<'a> {
    fn games_played(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, usize)> + 'a, TournamentError> {
        let config = self.0.ranking_config();
        Ok(chain!(
            to_weight_rank(self.least_played(id)?, config.least_played),
            to_weight_rank(self.nemesis(id)?, config.nemesis),
            to_weight_rank(self.lost_with(id)?, config.lost_with),
        )
        .map(|((player, _), score)| (player, score)))
    }

    fn neighbors(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, usize)> + 'a, TournamentError> {
        let config = self.0.ranking_config();
        Ok(chain!(
            to_weight_rank(self.elo_neighbors(id)?, config.elo_neighbor),
            to_weight_rank(self.wr_neighbors(id)?, config.wr_neighbor),
            to_weight_rank(self.expected_neighbors(id)?, config.expected_neighbor),
        ))
    }

    pub fn combined(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'a>> + 'a, TournamentError> {
        self.0.require_id_registered(id)?;

        let games_played_ranked = self.games_played(id)?;
        let neighbors_ranked = self.neighbors(id)?;
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

    pub fn ranked(
        self,
        id: PlayerId,
        method: RankingMethod,
    ) -> Result<Vec<RegisteredPlayer<'a>>, TournamentError> {
        Ok(match method {
            RankingMethod::LeastPlayed => {
                self.least_played(id)?.map(|(player, _)| player).collect()
            }
            RankingMethod::LostWith => self.lost_with(id)?.map(|(player, _)| player).collect(),
            RankingMethod::Nemesis => self.nemesis(id)?.map(|(player, _)| player).collect(),
            RankingMethod::EloNeighbors => self.elo_neighbors(id)?.collect(),
            RankingMethod::WRNeighbors => self.wr_neighbors(id)?.collect(),
            RankingMethod::ExpectedNeighbors => self.expected_neighbors(id)?.collect(),
            RankingMethod::Combined => self.combined(id)?.collect(),
        })
    }
}

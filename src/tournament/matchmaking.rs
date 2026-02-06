use itertools::Itertools;

use crate::tournament::{Tournament, TournamentError};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchmakerConfig {
    weight_least_played: f64,
    weight_nemesis: f64,
    weight_neighbor: f64,
    weight_wr_neighbor: f64,
    weight_lost_with: f64,
}

impl Tournament {
    fn rank_least_played(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        if !self.has_registered_player(player) {
            return Err(TournamentError::PlayerNotRegistered(player.to_string()));
        }

        Ok(self
            .games
            .iter()
            .map(|game| game.players.clone())
            .filter(|p| p.contains(&String::from(player)))
            .flatten()
            .filter(|p| p != player)
            .counts()
            .into_iter()
            .sorted_by_key(|(_, count)| *count)
            .map(|(player, _)| player))
    }

    fn rank_nemesis(&self, player: &str) -> Result<impl Iterator<Item = String>, TournamentError> {
        if !self.has_registered_player(player) {
            return Err(TournamentError::PlayerNotRegistered(player.to_string()));
        }
        Ok(self
            .games
            .iter()
            .filter(|game| game.players.contains(&String::from(player)))
            .flat_map(|game| {
                if player == game.winner {
                    game.players.iter().map(|p| (p.clone(), -1)).collect_vec()
                } else {
                    vec![(game.winner.clone(), 1)]
                }
            })
            .into_grouping_map()
            .sum()
            .into_iter()
            .sorted_by_key(|(_, count)| *count)
            .map(|(player, _)| player))
    }
}

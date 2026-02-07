use itertools::{Itertools, chain};

use crate::tournament::{GameMatch, PlayerStats, Tournament, TournamentError};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchmakerConfig {
    weight_least_played: f64,
    weight_nemesis: f64,
    weight_neighbor: f64,
    weight_wr_neighbor: f64,
    weight_lost_with: f64,
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self {
            weight_least_played: 6.0,
            weight_nemesis: 4.0,
            weight_neighbor: 5.0,
            weight_wr_neighbor: 3.0,
            weight_lost_with: 3.0,
        }
    }
}

macro_rules! impl_game_creator {
    ($method_name:ident, $rank_method: ident) => {
        pub fn $method_name(&mut self, player: &str) -> Result<GameMatch, TournamentError> {
            let mut iter = self.$rank_method(player)?;
            let p2 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let p3 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let p4 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            Ok(self.create_game([player.to_string(), p2, p3, p4]))
        }
    };
}

impl Tournament {
    fn is_registered(&self, player: &str) -> Result<(), TournamentError> {
        if !self.has_registered_player(player) {
            return Err(TournamentError::PlayerNotRegistered(player.to_string()));
        }
        Ok(())
    }

    fn get_player_stats<'a>(&'a self, player: &str) -> Result<&'a PlayerStats, TournamentError> {
        self.players
            .get(player)
            .ok_or_else(|| TournamentError::PlayerNotRegistered(player.to_string()))
    }

    pub fn rank_least_played(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        self.is_registered(player)?;

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

    impl_game_creator!(game_least_played, rank_least_played);

    pub fn rank_nemesis(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        self.is_registered(player)?;

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

    impl_game_creator!(game_nemesis, rank_nemesis);

    pub fn rank_wr_neighbors(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        let stats = self.get_player_stats(player)?;

        Ok(self
            .players
            .iter()
            .filter(|&(p, _)| p != player)
            .map(|(p, s)| (p, (stats.wr().unwrap_or(0.0) - s.wr().unwrap_or(0.0)).abs()))
            .sorted_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(p, _)| p.clone()))
    }

    impl_game_creator!(game_wr_neighbors, rank_wr_neighbors);

    pub fn rank_neighbors(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        let stats = self.get_player_stats(player)?;

        Ok(self
            .players
            .iter()
            .filter(|&(p, _)| p != player)
            .map(|(p, s)| (p, (stats.elo() - s.elo()).abs()))
            .sorted_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(p, _)| p.clone()))
    }

    impl_game_creator!(game_neighbors, rank_neighbors);

    pub fn rank_combined(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        fn to_weight(weight: f64) -> impl Fn((usize, String)) -> (String, f64) {
            move |(score, player)| (player, (score as u64) as f64 * weight)
        }

        Ok(chain!(
            self.rank_least_played(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_least_played)),
            self.rank_nemesis(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_nemesis)),
            self.rank_neighbors(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_neighbor)),
            self.rank_wr_neighbors(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_wr_neighbor)),
        )
        .into_grouping_map()
        .sum()
        .into_iter()
        .sorted_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(p, _)| p))
    }

    impl_game_creator!(game_combined, rank_combined);
}

use crate::{Tournament, error::TournamentError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(default = "Default::default")]
pub struct TournamentConfig {
    #[serde(rename = "se", alias = "starting_elo")]
    pub starting_elo: f64,
    #[serde(rename = "gp", alias = "game_points")]
    pub game_points: f64,
    #[serde(rename = "geps", alias = "game_elo_pow_scale")]
    pub game_elo_pow_scale: f64,
    #[serde(rename = "gwps", alias = "game_wr_pow_scale")]
    pub game_wr_pow_scale: f64,
    #[serde(rename = "gew", alias = "game_elo_weight")]
    pub game_elo_weight: f64,
    #[serde(rename = "gww", alias = "game_wr_weight")]
    pub game_wr_weight: f64,
    #[serde(rename = "mwlp", alias = "match_weight_least_played")]
    pub match_weight_least_played: f64,
    #[serde(rename = "mwn", alias = "match_weight_nemesis")]
    pub match_weight_nemesis: f64,
    #[serde(rename = "mwlw", alias = "match_weight_lost_with")]
    pub match_weight_lost_with: f64,
    #[serde(
        rename = "mwln",
        alias = "match_weight_neighbor",
        alias = "mwne",
        alias = "match_weight_elo_neighbor"
    )]
    pub match_weight_elo_neighbor: f64,
    #[serde(rename = "mwwn", alias = "match_weight_wr_neighbor")]
    pub match_weight_wr_neighbor: f64,

    #[serde(rename = "mwen", alias = "match_weight_expected_neighbor")]
    pub match_weight_expected_neighbor: f64,
}

impl Default for TournamentConfig {
    fn default() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            game_elo_pow_scale: 6.0,
            game_wr_pow_scale: 1.0,
            game_elo_weight: 65.0,
            game_wr_weight: 35.0,
            match_weight_least_played: 6.0,
            match_weight_nemesis: 4.0,
            match_weight_elo_neighbor: 5.0,
            match_weight_wr_neighbor: 3.0,
            match_weight_lost_with: 3.0,
            match_weight_expected_neighbor: 4.0,
        }
    }
}

impl Tournament {
    #[must_use]
    pub const fn config(&self) -> &TournamentConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: TournamentConfig) -> Result<(), TournamentError> {
        self.config = config;
        self.reload()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn updating_config_updates_stats() {
        let mut tournament = Tournament::generate_tournament(4, 1).unwrap();
        let id = *tournament.players.keys().next().unwrap();
        let elo_start = tournament.get_player_stats(id).unwrap().elo();
        let mut config = tournament.config().clone();
        config.starting_elo += 1500.0;
        tournament.set_config(config).unwrap();
        let elo_end = tournament.get_player_stats(id).unwrap().elo();
        assert!(elo_start.total_cmp(&elo_end).is_ne());
    }

    #[test]
    fn updating_config_updates_version() {
        let mut tournament = Tournament::generate_tournament(4, 1).unwrap();
        let mut config = tournament.config().clone();
        config.starting_elo += 1500.0;

        let version = tournament.snapshot;
        tournament.set_config(config).unwrap();
        let new_version = tournament.snapshot;
        assert_eq!(version + 1, new_version);
    }
}

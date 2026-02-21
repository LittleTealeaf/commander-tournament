use crate::{Tournament, error::TournamentError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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
    #[serde(rename = "mwne", alias = "match_weight_neighbor")]
    pub match_weight_neighbor: f64,
    #[serde(rename = "mwwn", alias = "match_weight_wr_neighbor")]
    pub match_weight_wr_neighbor: f64,
    #[serde(rename = "mwlw", alias = "match_weight_lost_with")]
    pub match_weight_lost_with: f64,
    #[serde(skip)]
    pub(crate) version: usize,
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
            match_weight_neighbor: 5.0,
            match_weight_wr_neighbor: 3.0,
            match_weight_lost_with: 3.0,
            version: 0,
        }
    }
}

impl Tournament {
    #[must_use]
    pub const fn config(&self) -> &TournamentConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: &TournamentConfig) -> Result<(), TournamentError> {
        self.config = TournamentConfig {
            version: self.config.version + 1,
            ..*config
        };
        self.reload()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Tournament, config::TournamentConfig};

    #[test]
    fn new_config_version_is_0() {
        assert_eq!(0, TournamentConfig::default().version);
    }

    #[test]
    fn updating_config_increases_version() {
        let mut t = Tournament::new();
        t.set_config(&t.config.clone()).unwrap();
        assert_eq!(1, t.config.version);
        t.set_config(&t.config.clone()).unwrap();
        assert_eq!(2, t.config.version);
    }

    #[test]
    fn config_version_based_on_tournament() {
        let mut t = Tournament::new();
        let mut config = t.config().clone();
        config.version = 5;
        t.set_config(&config).unwrap();
        assert_eq!(1, t.config.version);
    }

    #[test]
    fn updating_config_updates_stats() {
        let mut tournament = Tournament::generate_tournament(4, 1).unwrap();
        let id = *tournament.players.keys().next().unwrap();
        let elo_start = tournament.get_player_stats(id).unwrap().elo();
        let mut config = tournament.config().clone();
        config.starting_elo += 1500.0;
        tournament.set_config(&config).unwrap();
        let elo_end = tournament.get_player_stats(id).unwrap().elo();
        assert!(elo_start.total_cmp(&elo_end).is_ne());
    }
}

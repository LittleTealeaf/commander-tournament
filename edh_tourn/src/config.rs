use crate::{Tournament, error::TournamentError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TournamentConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub game_elo_pow_scale: f64,
    pub game_wr_pow_scale: f64,
    pub game_elo_weight: f64,
    pub game_wr_weight: f64,
    pub match_weight_least_played: f64,
    pub match_weight_nemesis: f64,
    pub match_weight_neighbor: f64,
    pub match_weight_wr_neighbor: f64,
    pub match_weight_lost_with: f64,
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
        assert_eq!(0, t.config.version);
        t.set_config(&t.config.clone()).unwrap();
        assert_eq!(1, t.config.version);
    }
}

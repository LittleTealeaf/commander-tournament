use commander_tournament_core::config::{game::GameConfig, matchmaker::MatchmakerConfig};

#[derive(Debug, Clone)]
pub(super) struct GameConfigDraft {
    pub(super) initial_elo: String,
    pub(super) logistic_scale: String,
    pub(super) initial_k: String,
    pub(super) base_k: String,
    pub(super) calibration_games: String,
}

impl From<&GameConfig> for GameConfigDraft {
    fn from(cfg: &GameConfig) -> Self {
        Self {
            initial_elo: cfg.initial_elo().to_string(),
            logistic_scale: cfg.logistic_scale().to_string(),
            initial_k: cfg.initial_k().to_string(),
            base_k: cfg.base_k().to_string(),
            calibration_games: cfg.calibration_games().to_string(),
        }
    }
}

impl GameConfigDraft {
    pub(super) fn parse(&self) -> Result<GameConfig, String> {
        let initial_elo = self
            .initial_elo
            .trim()
            .parse::<f64>()
            .map_err(|_| "Initial Elo must be a valid number.".to_owned())?;
        let logistic_scale = self
            .logistic_scale
            .trim()
            .parse::<f64>()
            .map_err(|_| "Logistic Scale must be a valid number.".to_owned())?;
        let initial_k = self
            .initial_k
            .trim()
            .parse::<f64>()
            .map_err(|_| "Initial K must be a valid number.".to_owned())?;
        let base_k = self
            .base_k
            .trim()
            .parse::<f64>()
            .map_err(|_| "Base K must be a valid number.".to_owned())?;
        let calibration_games = self
            .calibration_games
            .trim()
            .parse::<u32>()
            .map_err(|_| "Calibration Games must be a non-negative integer.".to_owned())?;

        if logistic_scale <= 0.0 {
            return Err("Logistic Scale must be greater than 0.".to_owned());
        }
        if initial_k <= 0.0 || base_k <= 0.0 {
            return Err("K-factors must be greater than 0.".to_owned());
        }

        Ok(GameConfig::new(
            initial_elo,
            logistic_scale,
            initial_k,
            base_k,
            calibration_games,
        ))
    }
}

#[derive(Debug, Clone)]
pub(super) struct MatchmakerConfigDraft {
    pub(super) elo_range: String,
    pub(super) min_pool_size: String,
    pub(super) repeat_penalty: String,
    pub(super) strict_calibrate: bool,
    pub(super) max_precon: String,
    pub(super) max_unranked: String,
}

impl From<&MatchmakerConfig> for MatchmakerConfigDraft {
    fn from(cfg: &MatchmakerConfig) -> Self {
        Self {
            elo_range: cfg.elo_range().to_string(),
            min_pool_size: cfg.min_pool_size().to_string(),
            repeat_penalty: cfg.repeat_penalty().to_string(),
            strict_calibrate: cfg.strict_calibrate(),
            max_precon: cfg.max_precon().to_string(),
            max_unranked: cfg.max_unranked().to_string(),
        }
    }
}

impl MatchmakerConfigDraft {
    pub(super) fn parse(&self) -> Result<MatchmakerConfig, String> {
        let elo_range = self
            .elo_range
            .trim()
            .parse::<f64>()
            .map_err(|_| "Elo Range must be a valid number.".to_owned())?;
        let min_pool_size = self
            .min_pool_size
            .trim()
            .parse::<usize>()
            .map_err(|_| "Min Pool Size must be a non-negative integer.".to_owned())?;
        let repeat_penalty = self
            .repeat_penalty
            .trim()
            .parse::<f64>()
            .map_err(|_| "Repeat Penalty must be a valid number.".to_owned())?;
        let max_precon = self
            .max_precon
            .trim()
            .parse::<usize>()
            .map_err(|_| "Max Precons must be a non-negative integer.".to_owned())?;
        let max_unranked = self
            .max_unranked
            .trim()
            .parse::<usize>()
            .map_err(|_| "Max Unranked must be a non-negative integer.".to_owned())?;

        Ok(MatchmakerConfig::new(
            elo_range,
            min_pool_size,
            repeat_penalty,
            self.strict_calibrate,
            max_precon,
            max_unranked,
        ))
    }
}

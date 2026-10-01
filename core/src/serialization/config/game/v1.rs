use crate::config::game::GameConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_more::Constructor)]
pub struct V1GameConfig {
    #[serde(alias = "starting_elo")]
    initial_elo: f64,
    logistic_scale: f64,
    initial_k: f64,
    base_k: f64,
    calibration_games: u32,
}

impl Default for V1GameConfig {
    fn default() -> Self {
        Self {
            initial_elo: 1500.0,
            logistic_scale: 400.0,
            initial_k: 48.0,
            base_k: 24.0,
            calibration_games: 12,
        }
    }
}

impl From<GameConfig> for V1GameConfig {
    fn from(value: GameConfig) -> Self {
        Self::new(
            value.initial_elo(),
            value.logistic_scale(),
            value.initial_k(),
            value.base_k(),
            value.calibration_games(),
        )
    }
}

impl From<V1GameConfig> for GameConfig {
    fn from(value: V1GameConfig) -> Self {
        Self::new(
            value.initial_elo,
            value.logistic_scale,
            value.initial_k,
            value.base_k,
            value.calibration_games,
        )
    }
}

use crate::serialization::config::{
    game::V1GameConfig,
    v2::{V2TournamentConfig, V2TournamentMatchmakerConfig},
};

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub struct V1TournamentConfig {
    #[serde(default)]
    pub(crate) game: V1TournamentGameConfig,
    #[serde(default)]
    pub(crate) matchmaker: V2TournamentMatchmakerConfig,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct V1TournamentGameConfig {
    #[serde(alias = "starting_elo")]
    initial_elo: f64,
    logistic_scale: f64,
    initial_k: f64,
    base_k: f64,
    calibration_games: u32,
}

impl Default for V1TournamentGameConfig {
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

impl From<V1TournamentConfig> for V2TournamentConfig {
    fn from(value: V1TournamentConfig) -> Self {
        Self::new(
            V1GameConfig::new(
                value.game.initial_elo,
                value.game.logistic_scale,
                value.game.initial_k,
                value.game.base_k,
                value.game.calibration_games,
            )
            .into(),
            value.matchmaker,
        )
    }
}

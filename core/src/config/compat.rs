use crate::config::{TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig};

#[derive(Debug, serde::Deserialize)]
pub struct UnversionedGameConfig {
    #[serde(alias = "starting_elo")]
    initial_elo: f64,
    logistic_scale: f64,
    initial_k: f64,
    base_k: f64,
    calibration_games: u32,
}

impl Default for UnversionedGameConfig {
    fn default() -> Self {
        Self {
            initial_elo: 1500.0,
            logistic_scale: 400.0,
            initial_k: 24.0,
            base_k: 12.0,
            calibration_games: 12,
        }
    }
}

impl From<UnversionedGameConfig> for GameConfig {
    fn from(value: UnversionedGameConfig) -> Self {
        Self::new(
            value.initial_elo,
            value.logistic_scale,
            value.initial_k,
            value.base_k,
            value.calibration_games,
        )
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UnversionedMatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for UnversionedMatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 120.0,
            min_pool_size: 5,
        }
    }
}

impl From<UnversionedMatchmakerConfig> for MatchmakerConfig {
    fn from(value: UnversionedMatchmakerConfig) -> Self {
        Self::new(value.elo_range, value.min_pool_size)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct V1TournamentConfig {
    #[serde(default)]
    game: UnversionedGameConfig,
    #[serde(default)]
    matchmaker: UnversionedMatchmakerConfig,
}

impl From<V1TournamentConfig> for TournamentConfig {
    fn from(value: V1TournamentConfig) -> Self {
        Self::new(value.game.into(), value.matchmaker.into())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct V2TournamentConfig {
    #[serde(default)]
    game: GameConfig,
    #[serde(default)]
    matchmaker: UnversionedMatchmakerConfig,
}

impl From<V2TournamentConfig> for TournamentConfig {
    fn from(value: V2TournamentConfig) -> Self {
        Self::new(value.game, value.matchmaker.into())
    }
}

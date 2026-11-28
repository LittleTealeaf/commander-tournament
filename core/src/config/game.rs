use backwards_compat::backwards_compat;
use serde_inline_default::serde_inline_default;

const DEFAULT: GameConfig = GameConfig {
    initial_elo: 1500.0,
    logistic_scale: 400.0,
    initial_k: 24.0,
    base_k: 12.0,
    calibration_games: 15,
    momentum_alpha: 0.25,
    momentum_k: 18.0,
    momentum_win_streak: 4,
    momentum_loss_streak: 7,
};

#[derive(Debug, Clone, PartialEq)]
#[serde_inline_default]
#[backwards_compat(
    versions(
        _: UnversionedGameConfig => Self,
        2: Self
    )
)]
pub struct GameConfig {
    #[serde(alias = "starting_elo")]
    pub initial_elo: f64,
    pub logistic_scale: f64,
    pub initial_k: f64,
    pub base_k: f64,
    pub calibration_games: u32,
    #[serde_inline_default(DEFAULT.initial_elo)]
    #[serde(rename = "mom_alpha", alias = "momentum_alpha")]
    pub momentum_alpha: f64,
    #[serde_inline_default(DEFAULT.momentum_k)]
    #[serde(rename = "mom_k", alias = "momentum_k")]
    pub momentum_k: f64,
    #[serde_inline_default(DEFAULT.momentum_win_streak)]
    #[serde(rename = "mom_w", alias = "momentum_win_streak")]
    pub momentum_win_streak: i32,
    #[serde_inline_default(DEFAULT.momentum_loss_streak)]
    #[serde(rename = "mom_l", alias = "momentum_loss_streak")]
    pub momentum_loss_streak: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        DEFAULT
    }
}

#[cfg(feature = "dev")]
impl GameConfig {
    #[must_use]
    pub fn random(seed: usize) -> Self {
        use rand::{RngExt, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);

        let base_k = rng.random_range(16.0..32.0);
        let initial_k = rng.random_range(base_k..64.0);

        Self {
            initial_elo: rng.random_range(1000.0..2000.0),
            logistic_scale: rng.random_range(200.0..600.0),
            initial_k,
            base_k,
            calibration_games: rng.random_range(5..30),
            ..Default::default()
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct UnversionedGameConfig {
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
        Self {
            initial_elo: value.initial_elo,
            logistic_scale: value.logistic_scale,
            initial_k: value.initial_k,
            base_k: value.base_k,
            calibration_games: value.calibration_games,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_config_returns_random() {
        for i in 0..100 {
            let _ = GameConfig::random(i);
        }
    }
}

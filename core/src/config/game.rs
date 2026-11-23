use backwards_compat::backwards_compat;

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
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    getset::CopyGetters,
    getset::Setters,
    getset::WithSetters,
    getset::MutGetters,
    derive_more::Constructor,
)]
#[getset(set = "pub", set_with = "pub", get_copy = "pub", get_mut = "pub")]
#[backwards_compat(
    tag = "version",
    versions(
        _: UnversionedGameConfig => Self,
        2: Self
    )
)]
pub struct GameConfig {
    #[serde(alias = "starting_elo")]
    initial_elo: f64,
    logistic_scale: f64,
    initial_k: f64,
    base_k: f64,
    calibration_games: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            initial_elo: 1500.0,
            logistic_scale: 400.0,
            initial_k: 24.0,
            base_k: 12.0,
            calibration_games: 15,
        }
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

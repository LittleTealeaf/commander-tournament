#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    getset::CopyGetters,
    getset::Setters,
    getset::WithSetters,
    getset::MutGetters,
    derive_more::Constructor,
)]
#[getset(set = "pub", set_with = "pub", get_copy = "pub", get_mut = "pub")]
pub struct GameConfig {
    starting_elo: f64,
    game_points: f64,
    game_elo_pow_scale: f64,
    game_wr_pow_scale: f64,
    game_elo_weight: f64,
    game_wr_weight: f64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            game_elo_pow_scale: 6.0,
            game_wr_pow_scale: 1.0,
            game_elo_weight: 65.0,
            game_wr_weight: 35.0,
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

        Self {
            starting_elo: rng.random_range(1000.0..5000.0),
            game_points: rng.random_range(10.0..200.0),
            game_elo_pow_scale: rng.random_range(1.0..5.0),
            game_wr_pow_scale: rng.random_range(1.0..5.0),
            game_elo_weight: rng.random_range(1.0..10.0),
            game_wr_weight: rng.random_range(1.0..10.0),
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

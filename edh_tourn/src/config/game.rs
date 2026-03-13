#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GameConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub game_elo_pow_scale: f64,
    pub game_wr_pow_scale: f64,
    pub game_elo_weight: f64,
    pub game_wr_weight: f64,
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

        let mut seed_bytes = [0u8; 32];
        let seed_data = seed.to_le_bytes();
        seed_bytes
            .get_mut(..seed_data.len())
            .expect("Expected Seed Data to Work")
            .copy_from_slice(&seed_data);

        let mut rng = ChaCha8Rng::from_seed(seed_bytes);

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

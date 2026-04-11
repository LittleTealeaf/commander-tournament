use crate::{config::game::GameConfig, player::stats::PlayerStats};

pub trait Matchable {
    fn wr(&self) -> Option<f64>;
    fn elo(&self) -> f64;
}

impl<T> Matchable for (T, &PlayerStats) {
    fn elo(&self) -> f64 {
        self.1.elo()
    }
    fn wr(&self) -> Option<f64> {
        self.1.wr()
    }
}

pub fn calculate_expected_values<M, const T: usize>(
    config: &GameConfig,
    players: [M; T],
) -> [(M, f64); T]
where
    M: Matchable,
{
    #[derive(Debug)]
    struct TempMatchPlayer<M> {
        player: M,
        scaled_elo: f64,
        scaled_wr: f64,
    }

    #[allow(clippy::cast_precision_loss)]
    let base_chance: f64 = 1.0 / (T as f64);

    let players = players.map(|player| TempMatchPlayer {
        scaled_elo: player.elo().powf(config.game_elo_pow_scale),
        scaled_wr: player
            .wr()
            .unwrap_or(base_chance)
            .powf(config.game_wr_pow_scale),
        player,
    });

    let sum_wr = players.iter().map(|p| p.scaled_wr).sum::<f64>();
    let sum_elo = players.iter().map(|p| p.scaled_elo).sum::<f64>();

    let weight_wr = if sum_wr > 0.0 {
        config.game_wr_weight
    } else {
        0.0
    };

    let weight_elo = if sum_elo > 0.0 {
        config.game_elo_weight
    } else {
        0.0
    };

    let weight_total = weight_wr + weight_elo;

    if weight_total <= 0.0 {
        return players.map(|p| (p.player, base_chance));
    }

    let coef_wr = if weight_wr > 0.0 {
        weight_wr / (weight_total * sum_wr)
    } else {
        0.0
    };

    let coef_elo = if weight_elo > 0.0 {
        weight_elo / (weight_total * sum_elo)
    } else {
        0.0
    };

    players.map(|p| {
        (
            p.player,
            p.scaled_elo.mul_add(coef_elo, p.scaled_wr * coef_wr),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // 1. Mock structure to test the algorithm logic independently of PlayerStats
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MockPlayer {
        id: usize,
        elo: f64,
        wr: Option<f64>,
    }

    impl Matchable for MockPlayer {
        fn elo(&self) -> f64 {
            self.elo
        }
        fn wr(&self) -> Option<f64> {
            self.wr
        }
    }

    fn test_config(elo_pow: f64, wr_pow: f64, elo_weight: f64, wr_weight: f64) -> GameConfig {
        GameConfig {
            game_elo_pow_scale: elo_pow,
            game_wr_pow_scale: wr_pow,
            game_elo_weight: elo_weight,
            game_wr_weight: wr_weight,
            ..Default::default()
        }
    }

    #[test]
    fn calculate_expected_values_normal() {
        let config = test_config(1.0, 1.0, 0.5, 0.5);
        let p1 = MockPlayer {
            id: 1,
            elo: 1500.0,
            wr: Some(0.6),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 1000.0,
            wr: Some(0.4),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // sum_elo = 2500, sum_wr = 1.0
        // weight_total = 1.0 (0.5 + 0.5)
        // coef_wr = 0.5 / (1.0 * 1.0) = 0.5
        // coef_elo = 0.5 / (1.0 * 2500) = 0.0002
        // P1 = (1500 * 0.0002) + (0.6 * 0.5) = 0.3 + 0.3 = 0.6
        // P2 = (1000 * 0.0002) + (0.4 * 0.5) = 0.2 + 0.2 = 0.4

        assert_eq!(result[0].0.id, 1);
        assert_relative_eq!(result[0].1, 0.6, epsilon = 1e-9);

        assert_eq!(result[1].0.id, 2);
        assert_relative_eq!(result[1].1, 0.4, epsilon = 1e-9);
    }

    #[test]
    fn calculate_expected_values_no_wr_fallback() {
        // Player 2 has None for wr, so it should fallback to 0.5 (1/2 base chance)
        let config = test_config(1.0, 1.0, 0.0, 1.0);
        let p1 = MockPlayer {
            id: 1,
            elo: 1000.0,
            wr: Some(0.3),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 1000.0,
            wr: None,
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // sum_wr = 0.3 + 0.5 = 0.8
        // coef_wr = 1.0 / (1.0 * 0.8) = 1.25
        // P1 = 0.3 * 1.25 = 0.375
        // P2 = 0.5 * 1.25 = 0.625
        assert_relative_eq!(result[0].1, 0.375, epsilon = 1e-9);
        assert_relative_eq!(result[1].1, 0.625, epsilon = 1e-9);
    }

    #[test]
    fn calculate_expected_values_zero_weights() {
        let config = test_config(1.0, 1.0, 0.0, 0.0);
        let p1 = MockPlayer {
            id: 1,
            elo: 1500.0,
            wr: Some(0.6),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 1000.0,
            wr: Some(0.4),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // Weight total is 0, should map perfectly to base_chance (0.5)
        assert_relative_eq!(result[0].1, 0.5);
        assert_relative_eq!(result[1].1, 0.5);
    }

    #[test]
    fn calculate_expected_values_zero_sums() {
        let config = test_config(1.0, 1.0, 1.0, 1.0);
        let p1 = MockPlayer {
            id: 1,
            elo: 0.0,
            wr: Some(0.0),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 0.0,
            wr: Some(0.0),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // If sum_wr and sum_elo are 0.0, weight_wr and weight_elo become 0.0.
        // Triggers the `weight_total <= 0.0` early return.
        assert_relative_eq!(result[0].1, 0.5);
        assert_relative_eq!(result[1].1, 0.5);
    }

    #[test]
    fn calculate_expected_values_only_elo() {
        let config = test_config(1.0, 1.0, 1.0, 0.0); // 100% ELO weight
        let p1 = MockPlayer {
            id: 1,
            elo: 1500.0,
            wr: Some(0.8),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 500.0,
            wr: Some(0.2),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // WR should be totally ignored. Shares should be 75% and 25%.
        assert_relative_eq!(result[0].1, 0.75, epsilon = 1e-9);
        assert_relative_eq!(result[1].1, 0.25, epsilon = 1e-9);
    }

    #[test]
    fn calculate_expected_values_only_wr() {
        let config = test_config(1.0, 1.0, 0.0, 1.0); // 100% WR weight
        let p1 = MockPlayer {
            id: 1,
            elo: 2000.0,
            wr: Some(0.8),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 2000.0,
            wr: Some(0.2),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // ELO should be totally ignored. Shares should be 80% and 20%.
        assert_relative_eq!(result[0].1, 0.8, epsilon = 1e-9);
        assert_relative_eq!(result[1].1, 0.2, epsilon = 1e-9);
    }

    #[test]
    fn calculate_expected_values_power_scaling() {
        let config = test_config(2.0, 2.0, 0.5, 0.5); // Square everything
        let p1 = MockPlayer {
            id: 1,
            elo: 2.0,
            wr: Some(0.2),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 4.0,
            wr: Some(0.4),
        };

        let result = calculate_expected_values(&config, [p1, p2]);

        // ELO squared: 4.0 and 16.0 (sum 20) -> shares 0.2 and 0.8
        // WR squared: 0.04 and 0.16 (sum 0.2) -> shares 0.2 and 0.8
        // Expected value combined shares should exactly map out to 0.2 and 0.8
        assert_relative_eq!(result[0].1, 0.2, epsilon = 1e-9);
        assert_relative_eq!(result[1].1, 0.8, epsilon = 1e-9);
    }

    #[test]
    fn calculate_expected_values_three_players() {
        let config = test_config(1.0, 1.0, 0.5, 0.5);
        let p1 = MockPlayer {
            id: 1,
            elo: 1000.0,
            wr: Some(0.5),
        };
        let p2 = MockPlayer {
            id: 2,
            elo: 1000.0,
            wr: Some(0.5),
        };
        let p3 = MockPlayer {
            id: 3,
            elo: 1000.0,
            wr: Some(0.5),
        };

        let result = calculate_expected_values(&config, [p1, p2, p3]);

        // Should perfectly divide 3 ways (0.333...)
        assert_relative_eq!(result[0].1, 1.0 / 3.0, epsilon = 1e-9);
        assert_relative_eq!(result[1].1, 1.0 / 3.0, epsilon = 1e-9);
        assert_relative_eq!(result[2].1, 1.0 / 3.0, epsilon = 1e-9);
    }
}

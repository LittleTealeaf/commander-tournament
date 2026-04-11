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

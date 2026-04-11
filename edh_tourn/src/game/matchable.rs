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

    if sum_wr <= 0.0 {
        if sum_elo <= 0.0 {
            return players.map(|pl| (pl.player, base_chance));
        }

        return players.map(|pl| (pl.player, pl.scaled_elo / sum_elo));
    }

    let weight_total = config.game_wr_weight + config.game_elo_weight;

    if weight_total <= 0.0 {
        return players.map(|pl| (pl.player, base_chance));
    }

    let coef_wr = config.game_wr_weight / (weight_total * sum_wr);
    let coef_elo = config.game_elo_weight / (weight_total * sum_elo);

    players.map(|pl| {
        (
            pl.player,
            pl.scaled_elo.mul_add(coef_elo, pl.scaled_wr * coef_wr),
        )
    })
}

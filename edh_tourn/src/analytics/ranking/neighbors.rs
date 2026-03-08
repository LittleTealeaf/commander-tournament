use core::cmp::Ordering;

use itertools::Itertools;

use crate::{Tournament, error::TournamentError, player::RegisteredPlayer};

#[must_use]
const fn abs_diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

fn ordered_by_proximity(score_a: f64, score_b: f64, target: f64, id_a: u32, id_b: u32) -> Ordering {
    let diff_a = abs_diff(score_a, target);
    let diff_b = abs_diff(score_b, target);

    let cmp = diff_a.total_cmp(&diff_b);

    let Ordering::Equal = cmp else {
        return cmp;
    };

    id_a.cmp(&id_b)
}

impl Tournament {
    pub fn get_player_ranked_elo_neighbors(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        self.ensure_id_registered(id)?;

        let players = self
            .get_registered_players()
            .filter(|player| player.id() != id);

        let elo = self.get_player_or_default_stats(id).elo();

        let sorted = players.sorted_by(|player_a, player_b| {
            ordered_by_proximity(
                player_a.stats().elo(),
                player_b.stats().elo(),
                elo,
                player_a.id(),
                player_b.id(),
            )
        });

        Ok(sorted)
    }

    pub fn get_player_ranked_wr_neighbors(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        self.ensure_id_registered(id)?;

        let players = self
            .get_registered_players()
            .filter(|player| player.id() != id);

        let wr = self.get_player_or_default_stats(id).wr().unwrap_or(0.25);

        let sorted = players.sorted_by(|player_a, player_b| {
            ordered_by_proximity(
                player_a.stats().wr().unwrap_or(0.25),
                player_b.stats().wr().unwrap_or(0.25),
                wr,
                player_a.id(),
                player_b.id(),
            )
        });

        Ok(sorted)
    }

    pub fn get_player_ranked_expected_neighbors(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        // This is going to function by the following: Assuming a 1v1, grab the list of people who
        // have the closest to a 50% expected winrate against the player
        self.ensure_id_registered(id)?;

        let players = self
            .get_registered_players()
            .filter(|player| player.id() != id);

        let stats = self.get_player_or_default_stats(id);
        let wr_t = stats
            .wr()
            .unwrap_or(0.25)
            .powf(self.config.game_wr_pow_scale);
        let elo_t = stats.elo().powf(self.config.game_elo_pow_scale);

        let weight_total = self.config.game_wr_weight + self.config.game_elo_weight;
        let weight_wr = self.config.game_wr_weight / weight_total;
        let weight_elo = self.config.game_elo_weight / weight_total;

        let players_calc = players.map(|player| {
            let wr = player.stats().wr().unwrap_or(0.25);
            let wr_scaled = wr.powf(self.config.game_wr_pow_scale);
            let elo_scaled = player.stats().elo().powf(self.config.game_elo_pow_scale);

            let coef_elo = weight_elo / (elo_scaled + elo_t);

            let expected =
                (weight_wr / (wr_scaled + wr_t)).mul_add(wr_scaled, coef_elo * elo_scaled);

            (player, abs_diff(expected, 0.5))
        });

        let sorted = players_calc
            .sorted_by(|(player_a, score_a), (player_b, score_b)| {
                let cmp = score_a.total_cmp(score_b);

                let Ordering::Equal = cmp else {
                    return cmp;
                };

                player_a.id().cmp(&player_b.id())
            })
            .map(|(player, _)| player);

        Ok(sorted)
    }
}

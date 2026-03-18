
use core::cmp::Ordering;

use itertools::Itertools;

use crate::{error::TournamentError, player::RegisteredPlayer, tournament::Tournament};

#[must_use]
const fn abs_diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

fn ordered_by_proximity(score_a: f64, score_b: f64, target: f64, id_a: u32, id_b: u32) -> Ordering {
    let diff_a = abs_diff(score_a, target);
    let diff_b = abs_diff(score_b, target);
    diff_a.total_cmp(&diff_b).then_with(|| id_a.cmp(&id_b))
}

impl Tournament {
    pub fn get_player_ranked_elo_neighbors(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = RegisteredPlayer<'_>>, TournamentError> {
        self.require_id_registered(id)?;

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
        self.require_id_registered(id)?;

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
        self.require_id_registered(id)?;

        Ok(self
            .get_registered_players()
            .filter(|player| player.id() != id)
            .map(|player| {
                let [_, match_player] = self.create_match_players([id, player.id()]);
                (player, abs_diff(*match_player.expected(), 0.5))
            })
            .sorted_by(|(player_a, score_a), (player_b, score_b)| {
                score_a
                    .total_cmp(score_b)
                    .then_with(|| player_a.id().cmp(&player_b.id()))
            })
            .map(|(player, _)| player))
    }
}

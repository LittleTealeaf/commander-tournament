pub mod least_played;

use core::cmp::Ordering;

use crate::{Tournament, error::TournamentError, player::stats::PlayerStats};

fn with_tie_breaker(cmp: Ordering, tie_breaker: impl Fn() -> Ordering) -> Ordering {
    match cmp {
        Ordering::Equal => tie_breaker(),
        cmp => cmp,
    }
}

impl Tournament {
    fn ensure_id_registered(&self, id: u32) -> Result<(), TournamentError> {
        if !self.is_id_registered(&id) {
            return Err(TournamentError::InvalidPlayerId(id));
        }
        Ok(())
    }

    fn get_elo(&self, id: u32) -> f64 {
        self.get_player_stats(id)
            .map_or(self.config.starting_elo, PlayerStats::elo)
    }

    fn get_wr(&self, id: u32) -> f64 {
        self.get_player_stats(id)
            .and_then(PlayerStats::wr)
            .unwrap_or(0.25)
    }
}

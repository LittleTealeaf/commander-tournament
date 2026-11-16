use std::collections::HashMap;

use crate::{
    error::TournamentError,
    player::{PlayerId, stats::PlayerStats},
    tournament::Tournament,
};

impl Tournament {
    pub fn recalculate_stats(&mut self) -> Result<(), TournamentError> {
        self.default_stats = PlayerStats::new(self.game_config().initial_elo());

        let mut stats = HashMap::new();
        let mut games = Vec::new();

        core::mem::swap(&mut self.stats, &mut stats);
        core::mem::swap(&mut self.games, &mut games);

        (|| {
            for record in &games {
                self.record_entry(record.into())?;
            }
            Ok(())
        })()
        .inspect_err(|_| {
            core::mem::swap(&mut self.games, &mut games);
            core::mem::swap(&mut self.stats, &mut stats);
        })
    }

    #[must_use]
    pub fn get_player_or_default_stats(&self, player: PlayerId) -> &PlayerStats {
        self.get_player_stats(player).unwrap_or(&self.default_stats)
    }

    #[must_use]
    pub const fn default_stats(&self) -> &PlayerStats {
        &self.default_stats
    }

    #[must_use]
    pub fn get_player_stats(&self, player: PlayerId) -> Option<&PlayerStats> {
        self.stats.get(&player)
    }
}

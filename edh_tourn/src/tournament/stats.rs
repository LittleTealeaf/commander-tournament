use crate::{error::TournamentError, player::stats::PlayerStats, tournament::Tournament};

impl Tournament {
    pub fn recalcualte_stats(&mut self) -> Result<(), TournamentError> {
        let version = self.snapshot;
        self.default_stats = PlayerStats::new(self.game_config().starting_elo);
        self.stats.clear();
        let mut games = Vec::new();
        core::mem::swap(&mut self.games, &mut games);
        for record in games {
            self.register_entry(record.into())?;
        }
        self.snapshot = version + 1;
        Ok(())
    }

    #[must_use]
    pub fn get_player_or_default_stats(&self, player: u32) -> &PlayerStats {
        self.get_player_stats(player).unwrap_or(&self.default_stats)
    }

    #[must_use]
    pub const fn default_stats(&self) -> &PlayerStats {
        &self.default_stats
    }

    #[must_use]
    pub fn get_player_stats(&self, player: u32) -> Option<&PlayerStats> {
        self.stats.get(&player)
    }
}

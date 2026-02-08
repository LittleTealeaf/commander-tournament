use crate::{Tournament, error::TournamentError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

impl Tournament {
    pub(crate) fn create_default_stats(&self) -> PlayerStats {
        PlayerStats {
            elo: self.config.starting_elo,
            games: 0,
            wins: 0,
        }
    }

    pub fn get_player_stats(&self, player: &u32) -> Option<&PlayerStats> {
        self.stats.get(player)
    }

    pub fn get_player_stats_cloned(&self, player: &u32) -> Result<PlayerStats, TournamentError> {
        if let Some(stats) = self.get_player_stats(player) {
            return Ok(stats.clone());
        }
        if self.is_id_registered(player) {
            return Ok(self.create_default_stats());
        }

        Err(TournamentError::InvalidPlayerId(*player))
    }
}

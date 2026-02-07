use crate::Tournament;

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
}

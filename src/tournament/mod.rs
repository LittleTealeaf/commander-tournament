mod errors;
mod game;
mod matchmaking;

use std::collections::HashMap;

pub use errors::*;
pub use game::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<GameRecord>,
    score_config: ScoreConfig,
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            score_config: ScoreConfig::new(),
        }
    }

    pub fn has_registered_player(&self, player: &str) -> bool {
        self.players.contains_key(player)
    }

    pub fn players(&self) -> impl Iterator<Item = (&String, &PlayerStats)> {
        self.players.iter()
    }
}

mod errors;
mod game;
mod matchmaking;

use std::collections::HashMap;

pub use errors::*;
pub use game::*;
pub use matchmaking::*;

#[derive(Debug, Clone)]
pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<GameRecord>,
    score_config: ScoreConfig,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            score_config: ScoreConfig::new(),
        }
    }

    pub fn players(&self) -> impl Iterator<Item = (&String, &PlayerStats)> {
        self.players.iter()
    }
}

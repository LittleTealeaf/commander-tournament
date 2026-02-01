mod errors;
mod game;
mod matchmaking;

use std::collections::HashMap;

pub use errors::*;
pub use game::*;
pub use matchmaking::*;

pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<GameRecord>,
    score_config: ScoreConfig,
    auto_register_players: bool,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            score_config: ScoreConfig::new(),
            auto_register_players: true,
        }
    }
}

pub mod config;
pub mod error;
pub mod game;
pub mod info;
pub mod matches;
pub mod stats;
pub mod tsv;
use std::collections::HashMap;

use crate::{
    config::TournamentConfig, error::TournamentError, game::GameRecord, info::PlayerInfo,
    stats::PlayerStats,
};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    config: TournamentConfig,
    stats: HashMap<u32, PlayerStats>,
    players: HashMap<u32, PlayerInfo>,
    #[serde(skip)]
    player_names: HashMap<String, u32>,
    games: Vec<GameRecord>,
}

impl Tournament {
    pub fn get_player_id(&self, name: &String) -> Result<u32, TournamentError> {
        self.player_names
            .get(name)
            .copied()
            .ok_or(TournamentError::PlayerNameNotRegistered(name.to_string()))
    }

    pub fn is_id_registered(&self, id: &u32) -> bool {
        self.players.contains_key(id)
    }

    pub fn register_player(&mut self, name: String) -> Result<u32, TournamentError> {
        if let Some(id) = self.player_names.get(&name) {
            return Err(TournamentError::PlayerAlreadyRegistered(name, *id));
        }

        let id = self.players.keys().max().map(|i| i + 1).unwrap_or(0);

        self.player_names.insert(name.clone(), id);
        self.players.insert(id, PlayerInfo::new(name));

        Ok(id)
    }

    pub fn unregister_player(&mut self, id: u32) -> Result<(), TournamentError> {
        self.players
            .remove(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?;
        self.games.retain(|game| !game.players().contains(&id));
        self.reload()?;
        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), TournamentError> {
        // Update player_names to the player info
        self.player_names = self
            .players
            .iter()
            .map(|(id, info)| (info.name().to_string(), *id))
            .collect();

        self.stats.clear();

        let mut games = Vec::new();
        std::mem::swap(&mut self.games, &mut games);
        for record in games {
            self.register_record(record)?;
        }

        Ok(())
    }
}

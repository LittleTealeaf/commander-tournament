pub mod config;
pub mod error;
pub mod info;
pub mod stats;
use std::collections::HashMap;

use crate::{
    config::TournamentConfig, error::TournamentError, info::PlayerInfo, stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    config: TournamentConfig,
    stats: HashMap<usize, PlayerStats>,
    players: HashMap<usize, PlayerInfo>,
    #[serde(skip)]
    player_names: HashMap<String, usize>,
    #[serde(skip)]
    next_id: usize,
}

impl Tournament {
    fn get_next_id(&mut self) -> usize {
        while self.players.contains_key(&self.next_id) {
            self.next_id += 1;
        }
        self.next_id
    }

    pub fn get_player_id(&self, name: &String) -> Result<usize, TournamentError> {
        self.player_names
            .get(name)
            .copied()
            .ok_or(TournamentError::PlayerNameNotRegistered(name.to_string()))
    }

    pub fn register_player(&mut self, name: String) -> Result<usize, TournamentError> {
        if self.player_names.contains_key(&name) {
            return Err(TournamentError::PlayerAlreadyRegistered(name));
        }

        let id = self.get_next_id();

        self.player_names.insert(name.clone(), id);
        self.players.insert(id, PlayerInfo::new(name));

        Ok(id)
    }

    pub fn config(&self) -> &TournamentConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: TournamentConfig) -> Result<(), TournamentError> {
        self.config = config;
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

        Ok(())
    }
}

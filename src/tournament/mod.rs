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
    stats: HashMap<u32, PlayerStats>,
    players: HashMap<u32, PlayerInfo>,
    #[serde(skip)]
    player_names: HashMap<String, u32>,
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
        if self.player_names.contains_key(&name) {
            return Err(TournamentError::PlayerAlreadyRegistered(name));
        }

        let id = self.players.keys().max().map(|i| i + 1).unwrap_or(0);

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

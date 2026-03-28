mod analytics;
mod config;
#[cfg(feature = "dev")]
mod dev;
mod matches;
mod players;
mod ranking;
mod stats;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    config::TournamentConfig,
    error::TournamentError,
    game::{entry::GameEntry, record::GameRecord},
    player::{PlayerId, info::PlayerInfo, stats::PlayerStats},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(try_from = "crate::serialization::SerializedTournament")]
#[serde(into = "crate::serialization::SerdeTournament")]
pub struct Tournament {
    pub(crate) config: TournamentConfig,
    pub(crate) stats: HashMap<PlayerId, PlayerStats>,
    pub(crate) default_stats: PlayerStats,
    pub(crate) players: HashMap<PlayerId, PlayerInfo>,
    pub(crate) player_names: HashMap<String, PlayerId>,
    pub(crate) games: Vec<GameRecord>,
    pub(crate) snapshot: usize,
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}

impl Tournament {
    #[must_use]
    pub fn new() -> Self {
        let config = TournamentConfig::default();
        Self {
            stats: HashMap::default(),
            default_stats: PlayerStats::new(config.game_config().starting_elo),
            players: HashMap::default(),
            player_names: HashMap::default(),
            games: Vec::new(),
            snapshot: 0,
            config,
        }
    }

    pub fn reload(&mut self) -> Result<(), TournamentError> {
        self.update_player_names();
        self.recalculate_stats()?;
        Ok(())
    }

    pub fn union(mut self, other: &Self) -> Result<Self, TournamentError> {
        self.merge(other)?;
        Ok(self)
    }

    pub fn merge(&mut self, other: &Self) -> Result<(), TournamentError> {
        let id_map = self.merge_players_from_tournament(other)?;
        let snapshot = self.snapshot;

        for game in &other.games {
            self.register_entry(GameEntry::from(game).map_ids(&id_map)?)?;
        }

        self.snapshot = snapshot + 1;

        Ok(())
    }

    pub fn into_fresh(&self) -> Result<Self, TournamentError> {
        let mut tourn = Self {
            config: self.config.clone(),
            default_stats: self.default_stats.clone(),
            snapshot: 0,
            ..Self::new()
        };
        tourn.merge(self)?;
        tourn.snapshot = 0;
        Ok(tourn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_tournament_snapshot_is_0() {
        let tourn = Tournament::new();
        assert_eq!(0, tourn.snapshot);
    }
    #[test]
    fn into_fresh_resets_snapshot() {
        let mut game = Tournament::new();
        game.snapshot = 5;
        let new_game = game.into_fresh().unwrap();
        assert_eq!(0, new_game.snapshot);
    }
}

pub mod compat;
pub mod config;
#[cfg(feature = "dev")]
pub mod dev;
pub mod error;
pub mod game;
pub mod info;
pub mod matches;
pub mod serialization;
pub mod stats;
pub mod tsv;
pub mod utils;

use std::collections::HashMap;

use crate::{
    utils::ordered_map,
    {
        config::TournamentConfig, error::TournamentError, game::GameRecord, info::PlayerInfo,
        stats::PlayerStats,
    },
};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(try_from = "serialization::SerdeTournament")]
pub struct Tournament {
    config: TournamentConfig,
    #[serde(skip)]
    stats: HashMap<u32, PlayerStats>,
    #[serde(serialize_with = "ordered_map")]
    players: HashMap<u32, PlayerInfo>,
    #[serde(skip)]
    player_names: HashMap<String, u32>,
    games: Vec<GameRecord>,
}

impl Tournament {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_player_id(&self, name: &String) -> Result<u32, TournamentError> {
        self.player_names
            .get(name)
            .copied()
            .ok_or_else(|| TournamentError::PlayerNameNotRegistered(name.clone()))
    }

    #[must_use]
    pub fn is_id_registered(&self, id: &u32) -> bool {
        self.players.contains_key(id)
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
            .map(|(id, info)| (info.name().to_owned(), *id))
            .collect();

        self.stats.clear();

        let mut games = Vec::new();
        core::mem::swap(&mut self.games, &mut games);
        for record in games {
            self.register_record(record)?;
        }

        Ok(())
    }

    #[must_use]
    pub const fn players(&self) -> &HashMap<u32, PlayerInfo> {
        &self.players
    }

    /// Moves all of the tournament data, systematically, into a new Tournament object.
    /// This is useful as a way around resetting player ids
    pub fn into_fresh(&self) -> Result<Self, TournamentError> {
        let mut tourn = Self::new();

        // Set Config
        tourn.config = TournamentConfig {
            version: 0,
            ..self.config
        };

        let mut id_map = HashMap::new();

        // Register players
        for (old_id, info) in &self.players {
            let id = tourn.register_player_with_info(info.clone())?;
            id_map.insert(*old_id, id);
        }

        // Register Games
        for game in &self.games {
            let players = (*game.players()).map(|p| id_map.get(&p));
            let ([Some(a), Some(b), Some(c), Some(d)], Some(winner)) =
                (players, id_map.get(&game.winner()))
            else {
                continue;
            };
            let record = GameRecord::new([*a, *b, *c, *d], *winner)?;
            tourn.register_record(record)?;
        }

        Ok(tourn)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::Tournament;

    #[test]
    fn new_has_no_players() {
        let new_tourn = Tournament::new();
        assert_eq!(0, new_tourn.players.len());
    }

    #[test]
    fn unregister_removes_players_games() {
        let sample = Tournament::sample_game();
        for id in sample.players().keys() {
            let mut tourn = sample.clone();
            tourn.unregister_player(*id).unwrap();
            for game in tourn.games() {
                assert!(!game.players().contains(id));
                assert_ne!(game.winner(), *id);
            }
        }
    }

    #[test]
    fn unregister_invalid_id_returns_err() {
        let mut tourn = Tournament::new();
        for i in 0..100 {
            tourn.unregister_player(i).unwrap_err();
        }
    }

    #[test]
    fn reload_maintains_equivilancy() {
        let mut sample = Tournament::sample_game();
        let snapshot = sample.clone();
        sample.reload().unwrap();
        assert_eq!(sample, snapshot);
    }

    #[test]
    fn load_resets_config_version() {
        assert_eq!(Tournament::sample_game().config.version, 0);
    }

    #[test]
    fn into_fresh_works_simple() -> anyhow::Result<()> {
        Tournament::sample_game().into_fresh()?;
        Tournament::generate_tournament(13, 50)?.into_fresh()?;
        Tournament::generate_tournament(13, 0)?.into_fresh()?;
        Tournament::generate_tournament(100, 50)?.into_fresh()?;
        Ok(())
    }

    #[test]
    fn into_fresh_same_players() -> anyhow::Result<()> {
        let game = Tournament::generate_tournament(35, 20)?;
        let new_game = game.into_fresh()?;
        let new_game_players = new_game.players().values().collect::<Vec<_>>();
        for player in game.players().values() {
            assert!(new_game_players.contains(&player));
        }
        assert_eq!(game.players().len(), new_game_players.len());

        Ok(())
    }

    #[test]
    fn into_fresh_resets_config_version() {
        let mut game = Tournament::new();
        game.config.version = 5;
        let new_game = game.into_fresh().unwrap();
        assert_eq!(0, new_game.config.version);
    }

    #[test]
    fn into_fresh_resets_ids() -> anyhow::Result<()> {
        const REMOVE_COUNT: usize = 40;
        let mut game = Tournament::generate_tournament(100, 0)?;
        let mut ids = game.players.keys().copied().sorted().take(40);
        // Just a dummy test that the first one is 0
        assert_eq!(0, ids.next().unwrap());
        game.unregister_player(0)?;

        for id in ids {
            game.unregister_player(id)?;
        }

        assert_eq!(60, game.players.len());
        assert_eq!(99, *game.players.keys().max().unwrap());

        let new_game = game.into_fresh()?;

        assert_eq!(60, new_game.players.len());
        assert_eq!(59, *new_game.players.keys().max().unwrap());

        Ok(())
    }
}

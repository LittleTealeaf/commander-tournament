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

use std::collections::HashMap;

use crate::{
    config::TournamentConfig,
    error::TournamentError,
    game::{GameEntry, GameRecord},
    info::PlayerInfo,
    serialization::{convert_games, ordered_map},
    stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(try_from = "serialization::SerdeTournament")]
pub struct Tournament {
    config: TournamentConfig,
    #[serde(skip)]
    stats: HashMap<u32, PlayerStats>,
    #[serde(skip)]
    default_stats: PlayerStats,
    #[serde(serialize_with = "ordered_map")]
    players: HashMap<u32, PlayerInfo>,
    #[serde(skip)]
    player_names: HashMap<String, u32>,
    #[serde(serialize_with = "convert_games")]
    games: Vec<GameRecord>,
}

impl Default for Tournament {
    fn default() -> Self {
        let config = TournamentConfig::default();
        Self {
            stats: HashMap::default(),
            default_stats: PlayerStats::new(config.starting_elo),
            players: HashMap::default(),
            player_names: HashMap::default(),
            games: Vec::new(),
            config,
        }
    }
}

impl Tournament {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get_player_id(&self, name: &String) -> Option<u32> {
        self.player_names.get(name).copied()
    }

    #[must_use]
    pub fn is_id_registered(&self, id: &u32) -> bool {
        self.players.contains_key(id)
    }

    pub fn unregister_player(&mut self, id: u32) -> Result<(), TournamentError> {
        self.players
            .remove(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?;
        self.games.retain(|game| !game.has_player(id));
        self.reload()?;
        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), TournamentError> {
        self.default_stats = PlayerStats {
            elo: self.config.starting_elo,
            games: 0,
            wins: 0,
        };
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

    /// Merges with another tournament. If decks from either game have the same name, they are
    /// merged. Games are added to the end of the base tournament.
    pub fn merge_tournament(&mut self, other: &Self) -> Result<(), TournamentError> {
        let mut id_map = HashMap::new();

        for (old_id, info) in &other.players {
            id_map.insert(
                *old_id,
                match self.get_player_id(info.name()) {
                    Some(id) => id,
                    None => self.register_player_with_info(info.clone())?,
                },
            );
        }

        for game in &other.games {
            self.register_entry(GameEntry::from(game).map_ids(&id_map)?)?;
        }

        Ok(())
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
            tourn.register_entry(GameEntry::from(game).map_ids(&id_map)?)?;
        }

        Ok(tourn)
    }
}

impl FromIterator<Self> for Tournament {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        let mut base = Self::new();
        for tourn in iter {
            let _ = base.merge_tournament(&tourn);
        }

        base
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::Tournament;

    #[test]
    fn collects_into_tournament() {
        let tourn = Tournament::test_tournaments().collect::<Tournament>();
        assert!(!tourn.games().is_empty());
    }

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
                assert!(!game.has_player(*id));
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
        for game in Tournament::test_tournaments() {
            game.into_fresh()?;
        }
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

    #[test]
    fn into_fresh_same_stats() -> anyhow::Result<()> {
        for game in Tournament::test_tournaments() {
            let new_game = game.into_fresh()?;
            for (id, info) in game.players() {
                let stats = game.get_player_stats(*id);
                let new_id = new_game.get_player_id(info.name()).unwrap();
                let new_stats = new_game.get_player_stats(new_id);
                assert_eq!(stats.is_some(), new_stats.is_some());
                let (Some(stats), Some(new_stats)) = (stats, new_stats) else {
                    continue;
                };

                assert!((stats.elo() - new_stats.elo()).abs() <= 1e-9);
            }
        }

        Ok(())
    }

    #[test]
    fn merge_tournaments_merge_players() {
        let players = ["a", "b", "c", "d"];
        let mut tournament_a = Tournament::new();
        for p in &players {
            tournament_a.register_player(p.to_string()).unwrap();
        }
        let mut tournament_b = Tournament::new();
        for p in &players {
            tournament_b.register_player(p.to_string()).unwrap();
        }

        tournament_a.merge_tournament(&tournament_b).unwrap();

        assert_eq!(4, tournament_a.players.len());
    }
}

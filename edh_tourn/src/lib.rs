pub mod compat;
pub mod config;
pub mod error;
pub mod game;
pub mod info;
pub mod matches;
pub mod stats;
#[cfg(test)]
pub mod testing;
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
#[serde(try_from = "SerdeTournament")]
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

#[derive(serde::Deserialize)]
struct SerdeTournament {
    config: TournamentConfig,
    players: HashMap<u32, PlayerInfo>,
    games: Vec<GameRecord>,
}

impl TryFrom<SerdeTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: SerdeTournament) -> Result<Self, TournamentError> {
        let player_names = value
            .players
            .iter()
            .map(|(id, info)| (info.name().to_owned(), *id))
            .collect();

        let mut tournament = Self {
            config: value.config,
            stats: HashMap::new(),
            players: value.players,
            player_names,
            games: Vec::new(),
        };

        tournament.config.version = 0;

        for game in value.games {
            tournament.register_record(game)?;
        }

        Ok(tournament)
    }
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
        std::mem::swap(&mut self.games, &mut games);
        for record in games {
            self.register_record(record)?;
        }

        Ok(())
    }

    pub fn players(&self) -> &HashMap<u32, PlayerInfo> {
        &self.players
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn unregister_removes_players_games() {
        let sample = Tournament::sample_game();
        for id in sample.players().keys() {
            let mut tourn = sample.clone();
            tourn.unregister_player(*id).unwrap();
            for game in tourn.games() {
                assert!(!game.players().contains(id));
                assert_ne!(game.winner(), *id)
            }
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
    fn ron_serialize_loop() {
        for mut game in [
            Tournament::sample_game(),
            Tournament::generate_tournament(30, 50).unwrap(),
        ] {
            for _ in 0..3 {
                let ser = ron::ser::to_string(&game).unwrap();
                game = ron::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn load_resets_config_version() {
        assert_eq!(Tournament::sample_game().config.version, 0);
    }
}

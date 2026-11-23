mod analytics;
mod config;
mod matches;
mod matchmaker;
mod players;
mod serialize;
mod stats;

use std::collections::HashMap;

use backwards_compat::backwards_compat;

use self::serialize::{
    SerializedTournament,
    compat::{v3::V3Tournament, v4::V4Tournament, v5::V5Tournament},
};
use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::{entry::GameEntry, record::GameRecord},
    player::{PlayerId, info::PlayerInfo, stats::PlayerStats},
    tournament::serialize::compat::{v6::V6Tournament, v7::V7Tournament},
};

#[derive(
    Debug, Clone, PartialEq, getset::Getters, getset::MutGetters, getset::Setters, getset::WithSetters,
)]
#[backwards_compat(
    tag = "version",
    error = TournamentError,
    versions(
        "3": V3Tournament => "5",
        "4": V4Tournament,
        "5": V5Tournament => 7,
        "6": V6Tournament,
        7 | "7": V7Tournament,
        #[fallible]
        #[serialize]
        8: SerializedTournament
    )
)]
pub struct Tournament {
    stats: HashMap<PlayerId, PlayerStats>,
    default_stats: PlayerStats,
    players: HashMap<PlayerId, PlayerInfo>,
    player_names: im::HashMap<String, PlayerId>,
    games: Vec<GameRecord>,
    #[getset(get = "pub")]
    game_config: GameConfig,
    #[getset(get = "pub", set = "pub", set_with = "pub", get_mut = "pub")]
    matchmaker_config: MatchmakerConfig,
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}

impl Tournament {
    #[must_use]
    pub fn new() -> Self {
        let game_config = GameConfig::default();
        Self {
            stats: HashMap::default(),
            default_stats: PlayerStats::new(game_config.initial_elo()),
            players: HashMap::default(),
            player_names: im::HashMap::default(),
            games: Vec::new(),
            game_config,
            matchmaker_config: MatchmakerConfig::default(),
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

        for game in &other.games {
            self.record_entry(GameEntry::from(game).map_ids(&id_map)?)?;
        }

        Ok(())
    }

    pub fn into_fresh(&self) -> Result<Self, TournamentError> {
        let mut tourn = Self {
            game_config: self.game_config.clone(),
            matchmaker_config: self.matchmaker_config.clone(),
            default_stats: self.default_stats.clone(),
            ..Self::new()
        };
        tourn.merge(self)?;
        Ok(tourn)
    }
}

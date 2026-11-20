mod analytics;
mod compat;
mod config;
mod matches;
mod matchmaker;
mod players;
pub(crate) mod serialize;
mod stats;

use std::collections::HashMap;

use backwards_compat::backwards_compat;

use self::{
    compat::{V3Tournament, V4Tournament, V5Tournament},
    serialize::SerializedTournament,
};
use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::{entry::GameEntry, record::GameRecord},
    player::{PlayerId, info::PlayerInfo, stats::PlayerStats},
};

#[derive(
    Debug, Clone, PartialEq, getset::Getters, getset::MutGetters, getset::Setters, getset::WithSetters,
)]
#[backwards_compat(
    tag = "version",
    version = 6,
    error = TournamentError,
    versions(
        3: V3Tournament => 5,
        4: V4Tournament,
        5: V5Tournament,
        #[fallible] 6: SerializedTournament,
    )
)]
pub struct Tournament {
    pub(crate) stats: HashMap<PlayerId, PlayerStats>,
    pub(crate) default_stats: PlayerStats,
    pub(crate) players: HashMap<PlayerId, PlayerInfo>,
    pub(crate) player_names: im::HashMap<String, PlayerId>,
    pub(crate) games: Vec<GameRecord>,
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

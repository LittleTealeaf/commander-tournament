use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::tournament::serialize::compat::v5::{
    V5GameConfig, V5MatchmakerConfig, V5Tournament, V5TournamentConfig,
};
use crate::utils::DeserializableMap;

fn player_info_deserialize<'de, D>(deserializer: D) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
        .into_iter()
        .map(|(id, info)| (PlayerId(id), info))
        .collect())
}

#[derive(Deserialize, Debug)]
pub struct V4Tournament {
    #[serde(alias = "cfg")]
    config: V4TournamentConfig,
    #[serde(deserialize_with = "player_info_deserialize", alias = "pls")]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(alias = "gms")]
    games: Vec<GameEntry>,
}

#[derive(Debug, Deserialize)]
pub(super) struct V4TournamentConfig {
    #[serde(default)]
    game: V5GameConfig,
    #[serde(default)]
    matchmaker: V4MatchmakerConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub(super) struct V4MatchmakerConfig {
    player_least_played: usize,
    player_lost_with: usize,
    player_nemesis: usize,
    elo_neighbor: usize,
    wr_neighbor: usize,
    expected_neighbor: usize,
    #[serde(alias = "exclude_precons")]
    include_precons: bool,
    outlier_include_extremes: bool,
}

impl Default for V4MatchmakerConfig {
    fn default() -> Self {
        Self {
            player_least_played: 4,
            player_nemesis: 3,
            player_lost_with: 2,
            elo_neighbor: 4,
            wr_neighbor: 3,
            expected_neighbor: 3,
            include_precons: true,
            outlier_include_extremes: true,
        }
    }
}

impl From<V4Tournament> for V5Tournament {
    fn from(value: V4Tournament) -> Self {
        Self::new(
            V5TournamentConfig::new(value.config.game, V5MatchmakerConfig::default()),
            value.players,
            value.games,
        )
    }
}

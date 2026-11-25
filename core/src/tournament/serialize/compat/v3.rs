use std::collections::HashMap;

use serde::Deserialize;

use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::tournament::serialize::compat::{
    v5::{V5GameConfig, V5MatchmakerConfig, V5Tournament, V5TournamentConfig},
    v7::player_info_deserialize,
};

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
struct V3RankingConfig {
    least_played: usize,
    nemesis: usize,
    lost_with: usize,
    elo_neighbor: usize,
    wr_neighbor: usize,
    expected_neighbor: usize,
}

impl Default for V3RankingConfig {
    fn default() -> Self {
        Self {
            least_played: 6,
            nemesis: 4,
            lost_with: 5,
            elo_neighbor: 3,
            wr_neighbor: 3,
            expected_neighbor: 4,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct V3TournamentConfig {
    #[serde(default)]
    game: V5GameConfig,
    #[serde(default)]
    ranking: V3RankingConfig,
}

#[derive(Deserialize, Debug)]
pub(in crate::tournament::serialize) struct V3Tournament {
    #[serde(alias = "cfg")]
    config: V3TournamentConfig,
    #[serde(deserialize_with = "player_info_deserialize", alias = "pls")]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(alias = "gms")]
    games: Vec<GameEntry>,
}

impl From<V3Tournament> for V5Tournament {
    fn from(value: V3Tournament) -> Self {
        println!("Converting V3 to V5");
        Self::new(
            V5TournamentConfig::new(value.config.game, V5MatchmakerConfig::default()),
            value.players,
            value.games,
        )
    }
}

use std::collections::HashMap;

use serde::Deserialize;

use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::tournament::compat::V5Tournament;
use crate::tournament::compat::v5::{V5GameConfig, V5MatchmakerConfig, V5TournamentConfig};
use crate::tournament::serialize::player_info_deserialize;

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, Default)]
pub struct V3RankingConfig {
    pub least_played: usize,
    pub nemesis: usize,
    pub lost_with: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
    pub expected_neighbor: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct V3TournamentConfig {
    #[serde(default)]
    game: V5GameConfig,
    #[serde(default)]
    ranking: V3RankingConfig,
}

#[derive(Deserialize, Debug)]
pub struct V3Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: V3TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "crate::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    pub(super) players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    pub(super) games: Vec<GameEntry>,
}

impl From<V3Tournament> for V5Tournament {
    fn from(value: V3Tournament) -> Self {
        Self {
            config: V5TournamentConfig::new(value.config.game, V5MatchmakerConfig::default()),
            players: value.players,
            games: value.games,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../../res/tests/compats/sample-v3.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

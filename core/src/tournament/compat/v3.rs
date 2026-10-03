use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::tournament::compat::V5Tournament;
use crate::tournament::compat::v5::{V5GameConfig, V5MatchmakerConfig, V5TournamentConfig};
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

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct V3RankingConfig {
    pub least_played: usize,
    pub nemesis: usize,
    pub lost_with: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
    pub expected_neighbor: usize,
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

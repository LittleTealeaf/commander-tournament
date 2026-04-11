use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::config::TournamentConfig;
use crate::config::game::GameConfig;
use crate::config::matchmaker::MatchmakerConfig;
use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::serialization::utils::DeserializableMap;
use crate::serialization::v4::V4Tournament;

fn player_info_deserialize<'de, D>(
    deserializer: D,
) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(
        DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
            .into_iter()
            .map(|(id, info)| (PlayerId(id), info))
            .collect(),
    )
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct V3TournamentConfig {
    #[serde(default)]
    pub game: GameConfig,
    #[serde(default)]
    pub ranking: V3RankingConfig,
}

#[derive(Deserialize, Debug, serde::Serialize)]
pub struct V3Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: V3TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "super::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    pub(super) players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    pub(super) games: Vec<GameEntry>,
}

impl From<V3Tournament> for V4Tournament {
    fn from(value: V3Tournament) -> Self {
        Self {
            players: value.players,
            games: value.games,
            config: TournamentConfig {
                game: value.config.game,
                matchmaker: MatchmakerConfig {
                    player_nemesis: value.config.ranking.nemesis,
                    player_lost_with: value.config.ranking.lost_with,
                    player_least_played: value.config.ranking.least_played,
                    elo_neighbor: value.config.ranking.elo_neighbor,
                    wr_neighbor: value.config.ranking.wr_neighbor,
                    expected_neighbor: value.config.ranking.expected_neighbor,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../res/tests/compats/sample-v3.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }

    #[test]
    fn deserialize_untagged() {
        let data = include_str!("../../../res/tests/compats/sample-v3-untagged.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

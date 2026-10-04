use std::collections::HashMap;

use serde::Deserialize;

use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::player::info::PlayerInfo;
use crate::tournament::compat::v5::{V5GameConfig, V5MatchmakerConfig, V5Tournament, V5TournamentConfig};
use crate::tournament::serialize::player_info_deserialize;

#[derive(Deserialize, Debug)]
pub struct V4Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: V4TournamentConfig,
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

#[derive(Debug, Deserialize)]
pub(super) struct V4TournamentConfig {
    #[serde(default)]
    pub game: V5GameConfig,
    #[serde(default)]
    pub matchmaker: V4MatchmakerConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub(super) struct V4MatchmakerConfig {
    pub player_least_played: usize,
    pub player_lost_with: usize,
    pub player_nemesis: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
    pub expected_neighbor: usize,
    #[serde(alias = "exclude_precons")]
    pub include_precons: bool,
    pub outlier_include_extremes: bool,
}

impl From<V4Tournament> for V5Tournament {
    fn from(value: V4Tournament) -> Self {
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
        let data = include_str!("../../../../res/tests/compats/sample-v4.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

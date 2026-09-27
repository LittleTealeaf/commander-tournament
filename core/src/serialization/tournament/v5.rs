use std::collections::HashMap;

use serde::Deserializer;

use crate::{
    config::{TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    serialization::{tournament::v6::V6Tournament, utils::DeserializableMap},
    tournament::Tournament,
};

fn player_info_deserialize<'de, D>(deserializer: D) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
        .into_iter()
        .map(|(id, info)| (PlayerId(id), info))
        .collect())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct V5Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: V5TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "super::super::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    pub(super) players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    pub(super) games: Vec<GameEntry>,
}

impl From<V5Tournament> for V6Tournament {
    fn from(value: V5Tournament) -> Self {
        Self {
            config: TournamentConfig::new(
                GameConfig::default(),
                MatchmakerConfig::new()
                    .with_elo_range(value.config.matchmaker.elo_range)
                    .with_min_pool_size(value.config.matchmaker.min_pool_size),
            ),
            players: value.players,
            games: value.games,
        }
    }
}

impl TryFrom<V5Tournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V5Tournament) -> Result<Self, Self::Error> {
        let mut tournament = Self {
            config: TournamentConfig::new(
                GameConfig::default(),
                MatchmakerConfig::new()
                    .with_elo_range(value.config.matchmaker.elo_range)
                    .with_min_pool_size(value.config.matchmaker.min_pool_size),
            ),
            players: value.players,
            ..Self::default()
        };
        tournament.reload()?;
        for game in value.games {
            tournament.record_entry(game)?;
        }

        tournament.snapshot = 0;

        Ok(tournament)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_more::Constructor)]
pub(super) struct V5TournamentConfig {
    #[serde(default)]
    game: V5GameConfig,
    #[serde(default)]
    matchmaker: V5MatchmakerConfig,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct V5GameConfig {
    starting_elo: f64,
    game_points: f64,
    game_elo_pow_scale: f64,
    game_wr_pow_scale: f64,
    game_elo_weight: f64,
    game_wr_weight: f64,
}

impl Default for V5GameConfig {
    fn default() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            game_elo_pow_scale: 6.0,
            game_wr_pow_scale: 1.0,
            game_elo_weight: 65.0,
            game_wr_weight: 35.0,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct V5MatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for V5MatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 50.0,
            min_pool_size: 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../../res/tests/compats/sample-v5.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

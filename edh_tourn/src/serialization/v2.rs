use std::collections::HashMap;

use crate::{
    config::{
        TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig, ranking::RankingConfig,
    },
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    serialization::{utils::DeserializableMap, v3::V3Tournament},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(default = "Default::default")]
pub(super) struct V2TournamentConfig {
    #[serde(alias = "se")]
    pub starting_elo: f64,
    #[serde(alias = "gp")]
    pub game_points: f64,
    #[serde(alias = "geps")]
    pub game_elo_pow_scale: f64,
    #[serde(alias = "gwps")]
    pub game_wr_pow_scale: f64,
    #[serde(alias = "gew")]
    pub game_elo_weight: f64,
    #[serde(alias = "gww")]
    pub game_wr_weight: f64,
    #[serde(alias = "mwlp")]
    pub match_weight_least_played: f64,
    #[serde(alias = "mwn")]
    pub match_weight_nemesis: f64,
    #[serde(alias = "mwlw")]
    pub match_weight_lost_with: f64,
    #[serde(alias = "mwln", alias = "match_weight_neighbor", alias = "mwne")]
    pub match_weight_elo_neighbor: f64,
    #[serde(alias = "mwwn")]
    pub match_weight_wr_neighbor: f64,
    #[serde(alias = "mwen")]
    pub match_weight_expected_neighbor: f64,
}

impl Default for V2TournamentConfig {
    fn default() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            game_elo_pow_scale: 6.0,
            game_wr_pow_scale: 1.0,
            game_elo_weight: 65.0,
            game_wr_weight: 35.0,
            match_weight_least_played: 6.0,
            match_weight_nemesis: 4.0,
            match_weight_elo_neighbor: 5.0,
            match_weight_wr_neighbor: 3.0,
            match_weight_lost_with: 3.0,
            match_weight_expected_neighbor: 4.0,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct V2GameEntry {
    #[serde(rename = "p", alias = "players")]
    pub(crate) players: [u32; 4],
    #[serde(rename = "w", alias = "winner")]
    pub(crate) winner: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct V2Tournament {
    #[serde(alias = "c")]
    pub(super) config: V2TournamentConfig,
    #[serde(
        alias = "p",
        deserialize_with = "DeserializableMap::deserialize_to_map"
    )]
    pub(super) players: HashMap<u32, PlayerInfo>,
    #[serde(alias = "g")]
    pub(super) games: Vec<V2GameEntry>,
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl From<V2Tournament> for V3Tournament {
    fn from(value: V2Tournament) -> Self {
        let game_config = GameConfig {
            starting_elo: value.config.starting_elo,
            game_points: value.config.game_points,
            game_elo_pow_scale: value.config.game_elo_pow_scale,
            game_wr_pow_scale: value.config.game_wr_pow_scale,
            game_elo_weight: value.config.game_elo_weight,
            game_wr_weight: value.config.game_wr_weight,
        };

        let ranking_config = RankingConfig {
            least_played: value.config.match_weight_least_played.round() as usize,
            nemesis: value.config.match_weight_nemesis.round() as usize,
            lost_with: value.config.match_weight_lost_with.round() as usize,
            elo_neighbor: value.config.match_weight_elo_neighbor.round() as usize,
            wr_neighbor: value.config.match_weight_wr_neighbor.round() as usize,
            expected_neighbor: value.config.match_weight_expected_neighbor.round() as usize,
        };

        Self {
            config: TournamentConfig {
                game: game_config,
                ranking: ranking_config,
                matchmaker: MatchmakerConfig::new(),
            },
            players: value
                .players
                .into_iter()
                .map(|(id, info)| (PlayerId(id), info))
                .collect(),
            games: value
                .games
                .into_iter()
                .filter_map(|game| {
                    GameEntry::new(game.players.map(PlayerId), PlayerId(game.winner)).ok()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../res/tests/compats/sample-v2.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

use core::iter::once;
use std::collections::HashMap;

use itertools::{Itertools, chain};
use serde::Deserialize;

use crate::{
    game::entry::GameEntry,
    player::{
        color::{ColorIdentity, MtgColor},
        info::PlayerInfo,
    },
    serialization::v2::{V2Tournament, V2TournamentConfig},
};

#[derive(Clone, serde::Deserialize, Debug)]
struct CompatGame {
    players: [String; 4],
    winner: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
struct CompatScoreConfig {
    starting_elo: f64,
    game_points: f64,
    elo_pow: f64,
    wr_pow: f64,
    elo_weight: f64,
    wr_weight: f64,
}

#[derive(Clone, serde::Deserialize, Debug)]
#[allow(clippy::struct_field_names)]
struct CompatMatchConfig {
    weight_least_played: f64,
    weight_nemesis: f64,
    weight_neighbor: f64,
    weight_wr_neighbor: f64,
    weight_lost_with: f64,
}

#[allow(dead_code)]
#[derive(Clone, serde::Deserialize, Debug)]
struct CompatPlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

#[derive(Clone, serde::Deserialize, Debug)]
struct CompatPlayerDetails {
    description: Option<String>,
    moxfield_link: Option<String>,
    colors: Vec<MtgColor>,
}

#[derive(Deserialize, Debug)]
pub struct V1Tournament {
    players: HashMap<String, CompatPlayerStats>,
    player_details: HashMap<String, CompatPlayerDetails>,
    games: Vec<CompatGame>,
    score_config: CompatScoreConfig,
    match_config: CompatMatchConfig,
}

impl From<V1Tournament> for V2Tournament {
    fn from(value: V1Tournament) -> Self {
        let config = V2TournamentConfig {
            starting_elo: value.score_config.starting_elo,
            game_points: value.score_config.game_points,
            game_elo_pow_scale: value.score_config.elo_pow,
            game_wr_pow_scale: value.score_config.wr_pow,
            game_elo_weight: value.score_config.elo_weight,
            game_wr_weight: value.score_config.wr_weight,
            match_weight_least_played: value.match_config.weight_least_played,
            match_weight_nemesis: value.match_config.weight_nemesis,
            match_weight_elo_neighbor: value.match_config.weight_neighbor,
            match_weight_wr_neighbor: value.match_config.weight_wr_neighbor,
            match_weight_lost_with: value.match_config.weight_lost_with,
            ..V2TournamentConfig::default()
        };

        let player_names = chain!(
            value.players.keys(),
            value.player_details.keys(),
            value
                .games
                .iter()
                .flat_map(|game| { chain!(&game.players, once(&game.winner)) }),
        )
        .unique()
        .cloned();

        let mut players = HashMap::new();
        for (i, player) in (0_u32..).zip(player_names) {
            let mut info = PlayerInfo::new(player.clone());
            if let Some(details) = value.player_details.get(&player) {
                info.set_color_identity(ColorIdentity::from_iter(details.colors.clone()));
                if let Some(description) = &details.description {
                    info.set_description(description.clone());
                }
                if let Some(moxfield) = &details.moxfield_link {
                    info.set_moxfield_id(moxfield.clone());
                }
            }

            players.insert(i, info);
        }

        let id_map = players
            .iter()
            .map(|(id, info)| (info.name().clone(), *id))
            .collect::<HashMap<_, _>>();

        let games = value
            .games
            .into_iter()
            .filter_map(|game| {
                let [str_a, str_b, str_c, str_d] = game.players;
                let str_w = game.winner;
                // let id_a = id_map.get()
                GameEntry::new(
                    [
                        *id_map.get(&str_a)?,
                        *id_map.get(&str_b)?,
                        *id_map.get(&str_c)?,
                        *id_map.get(&str_d)?,
                    ],
                    *id_map.get(&str_w)?,
                )
                .ok()
            })
            .collect();

        Self {
            config,
            players,
            games,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    pub fn serializes_v1_sample() {
        let data = include_str!("../../../res/tests/compats/sample-v1.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

use core::iter::once;
use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    Tournament,
    config::TournamentConfig,
    error::TournamentError,
    player::{color::MtgColor, info::PlayerInfo},
};

#[derive(Clone, serde::Deserialize)]
struct CompatGame {
    players: [String; 4],
    winner: String,
}

#[derive(Clone, serde::Deserialize)]
struct CompatScoreConfig {
    starting_elo: f64,
    game_points: f64,
    elo_pow: f64,
    wr_pow: f64,
    elo_weight: f64,
    wr_weight: f64,
}

#[derive(Clone, serde::Deserialize)]
#[allow(clippy::struct_field_names)]
struct CompatMatchConfig {
    weight_least_played: f64,
    weight_nemesis: f64,
    weight_neighbor: f64,
    weight_wr_neighbor: f64,
    weight_lost_with: f64,
}

#[allow(dead_code)]
#[derive(Clone, serde::Deserialize)]
struct CompatPlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

#[derive(Clone, serde::Deserialize)]
struct CompatPlayerDetails {
    description: Option<String>,
    moxfield_link: Option<String>,
    colors: Vec<MtgColor>,
}

#[derive(Clone, serde::Deserialize)]
pub struct TournamentCompatV1 {
    players: HashMap<String, CompatPlayerStats>,
    player_details: HashMap<String, CompatPlayerDetails>,
    games: Vec<CompatGame>,
    score_config: CompatScoreConfig,
    match_config: CompatMatchConfig,
}

impl TryFrom<TournamentCompatV1> for Tournament {
    type Error = TournamentError;

    fn try_from(value: TournamentCompatV1) -> Result<Self, Self::Error> {
        let mut tournament = Self::default();
        let players = chain!(
            value
                .games
                .iter()
                .cloned()
                .flat_map(|game| chain!(game.players, once(game.winner))),
            value.players.keys().cloned(),
            value.player_details.keys().cloned(),
        )
        .unique()
        .collect_vec();

        for player in players {
            let mut info = PlayerInfo::new(player.clone());

            if let Some(compat_info) = value.player_details.get(&player) {
                if let Some(description) = &compat_info.description {
                    info.set_description(description.to_owned());
                }
                for color in &compat_info.colors {
                    info.toggle_color(*color);
                }

                if let Some(link) = &compat_info.moxfield_link {
                    info.set_moxfield_id(link.to_owned());
                }

                // if let Some(link) = &compat_info.moxfield_link {
                //     let pattern = "/decks/";
                //     if let Some(index) = link.find(pattern) {
                //         let start_index = pattern.len() + index;
                //         info.set_moxfield_id(
                //             link[start_index..].split('/').next().map(str::to_owned),
                //         );
                //     }
                // }
            }

            tournament.register_player_with_info(info)?;
        }

        let config = TournamentConfig {
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
            ..TournamentConfig::default()
        };

        tournament.set_config(config)?;

        for game in value.games {
            let [player_a, player_b, player_c, player_d] = game.players;
            let winner = game.winner;

            let players = [
                tournament.get_or_register_player(player_a)?,
                tournament.get_or_register_player(player_b)?,
                tournament.get_or_register_player(player_c)?,
                tournament.get_or_register_player(player_d)?,
            ];

            let winner = tournament.get_or_register_player(winner)?;

            tournament.register_record(tournament.create_match(players)?.record(winner)?)?;
        }

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AURELIA: &str = "Aurelia, the Warleader";
    const BRE: &str = "Bre of Clan Stoutarm";
    const TIFA: &str = "Tifa Lockhart";
    const MINWU: &str = "Minwu, White Mage";
    const ROCKSANNE: &str = "Rocksanne, Starfall Savant";

    fn parse_from_compat() -> Tournament {
        let string = include_str!("../../../tests/compat-v1.ron");
        let compat_v1: TournamentCompatV1 = ron::from_str(string).unwrap();
        Tournament::try_from(compat_v1).unwrap()
    }

    #[test]
    fn populates_games() {
        let tourn = parse_from_compat();
        assert!(!tourn.games().is_empty());
    }

    #[test]
    fn test_decks_are_found() {
        let test_decks = [AURELIA, BRE, TIFA, MINWU, ROCKSANNE];

        let tourn = parse_from_compat();
        for deck in test_decks {
            let s = deck.to_owned();
            tourn
                .get_player_id(&s)
                .unwrap_or_else(|| panic!("Could not find deck: {s}"));
        }
    }

    #[test]
    fn info_moxfield_id() {
        let tourn = parse_from_compat();
        let aurelia_id = tourn
            .get_player_id(&String::from(AURELIA))
            .expect("Expected Aurelia to be a player");
        let info = tourn
            .get_player_info(&aurelia_id)
            .expect("Expected Player Info");

        let moxfield = info
            .moxfield_id()
            .expect("Expected Moxfield ID to be filled in");
        assert_eq!("BtCcQ8eWg0uT8n4fFPK3Xg", moxfield);
    }

    #[test]
    fn info_colors() {
        let tourn = parse_from_compat();

        let trials = [
            (ROCKSANNE, vec![MtgColor::Green, MtgColor::Red]),
            (MINWU, vec![MtgColor::White]),
            (TIFA, vec![MtgColor::Green]),
            (BRE, vec![MtgColor::Red, MtgColor::White]),
            (AURELIA, vec![MtgColor::Red, MtgColor::White]),
        ];

        for (deck, colors) in trials {
            let id = tourn.get_player_id(&deck.to_owned()).unwrap();
            let info = tourn.get_player_info(&id).unwrap();
            let identity = info.color_identity();
            for color in colors {
                assert!(identity.has_color(color));
            }
        }
    }

    #[test]
    fn info_description() {
        let tourn = parse_from_compat();
        let id = tourn.get_player_id(&TIFA.to_owned()).unwrap();
        let info = tourn.get_player_info(&id).unwrap();
        assert!(!info.description().is_empty());
    }
}

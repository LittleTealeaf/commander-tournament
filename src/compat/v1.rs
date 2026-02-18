use std::{collections::HashMap, iter::once};

use itertools::{Itertools, chain};

use crate::tourn::{
    Tournament, config::TournamentConfig, error::TournamentError, game::GameRecord, info::MtgColor,
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
            let id = tournament.register_player(player.clone())?;
            if let Some(compat_info) = value.player_details.get(&player) {
                let mut info = tournament.get_player_info(id)?;
                if let Some(description) = &compat_info.description {
                    info.set_description(description.to_owned());
                }
                for color in &compat_info.colors {
                    info.toggle_color(*color);
                }

                if let Some(link) = &compat_info.moxfield_link {
                    let pattern = "/decks/";
                    if let Some(index) = link.find(pattern) {
                        let start_index = pattern.len() + index;
                        info.set_moxfield_id(
                            link[start_index..].split('/').next().map(str::to_owned),
                        );
                    }
                }
                tournament.set_player_info(id, info)?;
            }
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
            match_weight_neighbor: value.match_config.weight_neighbor,
            match_weight_wr_neighbor: value.match_config.weight_wr_neighbor,
            match_weight_lost_with: value.match_config.weight_lost_with,
            version: 0,
        };

        tournament.set_config(config)?;

        for game in value.games {
            let mut ids = [0; 4];
            for (i, player) in game.players.iter().enumerate() {
                let id = tournament.get_player_id(player)?;
                ids[i] = id;
            }
            let winner = tournament.get_player_id(&game.winner)?;

            let record = GameRecord::new(ids, winner)?;
            tournament.register_record(record)?;
        }

        Ok(tournament)
    }
}

use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    Tournament,
    config::TournamentConfig,
    error::TournamentError,
    player::{
        color::{ColorIdentity, MtgColor},
        info::PlayerInfo,
    },
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
pub struct V1SerializedTournament {
    players: HashMap<String, CompatPlayerStats>,
    player_details: HashMap<String, CompatPlayerDetails>,
    games: Vec<CompatGame>,
    score_config: CompatScoreConfig,
    match_config: CompatMatchConfig,
}

impl TryFrom<V1SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V1SerializedTournament) -> Result<Self, Self::Error> {
        let mut tournament = Self::default();

        // Register any player with info
        for (name, details) in value.player_details {
            let mut info = PlayerInfo::new(name);
            if let Some(description) = details.description {
                info.set_description(description);
            }
            info.set_color_identity(ColorIdentity::from_iter(details.colors));

            if let Some(link) = details.moxfield_link {
                info.set_moxfield_id(link);
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
    use crate::Tournament;

    #[test]
    pub fn serializes_v1_sample() {
        let data = include_str!("../../../res/tests/compats/sample-v1.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}

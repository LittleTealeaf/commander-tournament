use std::collections::HashMap;

use crate::{
    Tournament,
    config::TournamentConfig,
    error::TournamentError,
    game::entry::GameEntry,
    player::{info::PlayerInfo, stats::PlayerStats},
};

#[derive(serde::Deserialize, Debug)]
pub struct V2SerializedTournament {
    #[serde(alias = "c")]
    config: TournamentConfig,
    #[serde(alias = "p")]
    players: HashMap<u32, PlayerInfo>,
    #[serde(alias = "g")]
    games: Vec<GameEntry>,
}

impl TryFrom<V2SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V2SerializedTournament) -> Result<Self, TournamentError> {
        let player_names = value
            .players
            .iter()
            .map(|(id, info)| (info.name().to_owned(), *id))
            .collect();

        let mut tournament = Self {
            default_stats: PlayerStats::new(value.config.starting_elo),
            config: value.config,
            stats: HashMap::new(),
            players: value.players,
            player_names,
            games: Vec::new(),
            snapshot: 0,
        };

        for game in value.games {
            tournament.register_entry(game)?;
        }

        tournament.snapshot = 0;

        Ok(tournament)
    }
}

use crate::tournament::{GameMatch, Tournament};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchupType {
    Combined,
    LeastPlayed,
    Nemesis,
    WinrateNeighbors,
    Neighbors,
}

impl fmt::Display for MatchupType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MatchupType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MatchupType::Combined => "Combined",
            MatchupType::LeastPlayed => "Least Played",
            MatchupType::Nemesis => "Rematch",
            MatchupType::WinrateNeighbors => "WR Neighbors",
            MatchupType::Neighbors => "Neighbors",
        }
    }

    pub fn all() -> &'static [MatchupType] {
        &[
            MatchupType::Combined,
            MatchupType::LeastPlayed,
            MatchupType::Nemesis,
            MatchupType::WinrateNeighbors,
            MatchupType::Neighbors,
        ]
    }
}

impl Default for MatchupType {
    fn default() -> Self {
        MatchupType::Combined
    }
}

#[derive(Default)]
pub struct TournamentApp {
    pub tournament: Tournament,
    pub selected_players: [Option<String>; 4],
    pub selected_match: Option<GameMatch>,
    pub selected_winner: Option<String>,
    pub match_player: Option<String>,
    pub change_player_name: Option<(Option<String>, String)>,
    pub show_config: bool,
    pub score_starting_elo: String,
    pub score_game_points: String,
    pub score_elo_pow: String,
    pub score_wr_pow: String,
    pub score_elo_weight: String,
    pub score_wr_weight: String,

    pub match_weight_least_played: String,
    pub match_weight_nemesis: String,
    pub match_weight_neighbor: String,
    pub match_weight_wr_neighbor: String,
    pub match_weight_lost_with: String,
    pub error: Option<String>,
    pub matchup_type: MatchupType,
}

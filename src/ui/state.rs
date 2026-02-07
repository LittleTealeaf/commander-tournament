use crate::tournament::{GameMatch, Tournament};
use std::fmt;
use std::fs::File;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MatchupType {
    #[default]
    Combined,
    LeastPlayed,
    Nemesis,
    WinrateNeighbors,
    Neighbors,
    LossWith,
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
            MatchupType::LossWith => "Loss With",
        }
    }

    pub fn all() -> &'static [MatchupType] {
        &[
            MatchupType::Combined,
            MatchupType::LeastPlayed,
            MatchupType::Nemesis,
            MatchupType::WinrateNeighbors,
            MatchupType::Neighbors,
            MatchupType::LossWith,
        ]
    }
}

#[derive()]
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
    pub show_ingest: bool,
    pub ingest_text: String,
    pub show_export: bool,
    pub export_text: String,
    pub show_games: bool,
    pub selected_game_index: Option<usize>,
}

impl Default for TournamentApp {
    fn default() -> Self {
        // Try to load game.ron if it exists
        if std::path::Path::new("game.ron").exists()
            && let Ok(file) = File::open("game.ron")
            && let Ok(tournament) = ron::de::from_reader(file)
        {
            return TournamentApp {
                tournament,
                selected_players: Default::default(),
                selected_match: None,
                selected_winner: Default::default(),
                match_player: None,
                change_player_name: None,
                show_config: false,
                score_starting_elo: Default::default(),
                score_game_points: Default::default(),
                score_elo_pow: Default::default(),
                score_wr_pow: Default::default(),
                score_elo_weight: Default::default(),
                score_wr_weight: Default::default(),
                match_weight_least_played: Default::default(),
                match_weight_nemesis: Default::default(),
                match_weight_neighbor: Default::default(),
                match_weight_wr_neighbor: Default::default(),
                match_weight_lost_with: Default::default(),
                error: None,
                matchup_type: Default::default(),
                show_ingest: false,
                show_export: false,
                ingest_text: String::new(),
                export_text: String::new(),
                show_games: false,
                selected_game_index: None,
            };
        }

        // Fall back to default empty app
        TournamentApp {
            tournament: Tournament::default(),
            selected_players: Default::default(),
            selected_match: None,
            selected_winner: Default::default(),
            match_player: None,
            change_player_name: None,
            show_config: false,
            score_starting_elo: Default::default(),
            score_game_points: Default::default(),
            score_elo_pow: Default::default(),
            score_wr_pow: Default::default(),
            score_elo_weight: Default::default(),
            score_wr_weight: Default::default(),
            match_weight_least_played: Default::default(),
            match_weight_nemesis: Default::default(),
            match_weight_neighbor: Default::default(),
            match_weight_wr_neighbor: Default::default(),
            match_weight_lost_with: Default::default(),
            error: None,
            matchup_type: MatchupType::default(),
            show_ingest: false,
            show_export: false,
            ingest_text: String::new(),
            export_text: String::new(),
            show_games: false,
            selected_game_index: None,
        }
    }
}

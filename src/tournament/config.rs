#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TournamentConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub game_elo_pow_scale: f64,
    pub game_wr_pow_scale: f64,
    pub game_elo_weight: f64,
    pub game_wr_weight: f64,
    pub match_weight_least_played: f64,
    pub match_weight_nemesis: f64,
    pub match_weight_neighbor: f64,
    pub match_weight_wr_neighbor: f64,
    pub match_weight_lost_with: f64,
}

impl Default for TournamentConfig {
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
            match_weight_neighbor: 5.0,
            match_weight_wr_neighbor: 3.0,
            match_weight_lost_with: 3.0,
        }
    }
}

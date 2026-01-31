use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct PlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

#[derive(Debug, Clone)]
pub struct Game {
    players: [String; 4],
    stats: [PlayerStats; 4],
    expected: [f64; 4],
}

pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<(Game, String)>,
    starting_score: f64,
    game_points: f64,
    score_wr_ratio: f64,
    score_pow: f64,
    wr_pow: f64,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            starting_score: 1500.0,
            game_points: 25.0,
            score_wr_ratio: 0.65,
            score_pow: 2.0,
            wr_pow: 1.0,
        }
    }

    pub fn get_player_stats(&self, player: &str) -> Option<&PlayerStats> {
        self.players.get(player)
    }

    pub fn get_or_create_player_stats(&mut self, player: String) -> &PlayerStats {
        if self.players.contains_key(&player) {
            return self.players.get(&player).unwrap();
        }

        self.players.insert(
            player.clone(),
            PlayerStats {
                elo: self.starting_score,
                games: 0,
                wins: 0,
            },
        );

        self.players.get(&player).unwrap()
    }

    pub fn create_game(&mut self, players: [&str; 4]) -> Game {
        let players = players.map(String::from);
        let stats = players.clone().map(|p| *self.get_or_create_player_stats(p));

        let adj_scores = stats.map(|stat| stat.elo.powf(self.score_pow));
        let score_total = adj_scores.iter().sum::<f64>();
        let score_ratios = adj_scores.map(|sc| sc / score_total);

        let adj_wr = stats.map(|stat| {
            if stat.games == 0 {
                return 0.25;
            }
            let games = f64::from(stat.games);
            let wins = f64::from(stat.wins);
            games / wins
        });
        let wr_total = adj_wr.iter().sum::<f64>();
        let wr_ratio = adj_wr.map(|wr| wr / wr_total);

        Game {
            stats,
            players,
            expected: [
                score_ratios[0] * self.score_wr_ratio + wr_ratio[0] * (1.0 - self.score_wr_ratio),
                score_ratios[1] * self.score_wr_ratio + wr_ratio[1] * (1.0 - self.score_wr_ratio),
                score_ratios[2] * self.score_wr_ratio + wr_ratio[2] * (1.0 - self.score_wr_ratio),
                score_ratios[3] * self.score_wr_ratio + wr_ratio[3] * (1.0 - self.score_wr_ratio),
            ],
        }
    }
}

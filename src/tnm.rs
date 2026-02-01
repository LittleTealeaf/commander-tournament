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

#[derive(Debug, Clone, Copy)]
pub struct ScoreConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub score_pow: f64,
    pub wr_pow: f64,
    pub score_wr_ratio: f64,
}

impl ScoreConfig {
    pub fn new() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            score_wr_ratio: 0.65,
            score_pow: 2.0,
            wr_pow: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<(Game, String)>,
    score_config: ScoreConfig,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            score_config: ScoreConfig::new(),
        }
    }

    pub fn get_score_config_mut(&mut self) -> &mut ScoreConfig {
        &mut self.score_config
    }

    pub fn get_score_config(&self) -> &ScoreConfig {
        &self.score_config
    }

    pub fn set_score_config(&mut self, config: ScoreConfig) {
        self.score_config = config
    }

    pub fn get_player_stats(&self, player: &str) -> Option<&PlayerStats> {
        self.players.get(player)
    }

    pub fn get_or_create_player_stats(&mut self, player: String) -> &PlayerStats {
        if self.players.contains_key(&player) {
            return self.players.get(&player).unwrap();
        }

        self.register_player(player.clone());

        self.players.get(&player).unwrap()
    }

    pub fn register_player(&mut self, player: String) -> bool {
        if self.players.contains_key(&player) {
            return false;
        }
        self.players.insert(
            player,
            PlayerStats {
                elo: self.score_config.starting_elo,
                games: 0,
                wins: 0,
            },
        );
        true
    }

    pub fn create_game(&mut self, players: [&str; 4]) -> Game {
        let players = players.map(String::from);
        let stats = players.clone().map(|p| *self.get_or_create_player_stats(p));

        let adj_scores = stats.map(|stat| stat.elo.powf(self.score_config.score_pow));
        let score_total = adj_scores.iter().sum::<f64>();
        let score_ratios = adj_scores.map(|sc| sc / score_total);

        let adj_wr = stats.map(|stat| {
            if stat.games == 0 {
                return 0.25;
            }
            let games = f64::from(stat.games);
            let wins = f64::from(stat.wins);
            (wins / games).powf(self.score_config.wr_pow)
        });
        let wr_total = adj_wr.iter().sum::<f64>();
        let wr_ratio = adj_wr.map(|wr| wr / wr_total);

        Game {
            stats,
            players,
            expected: [
                score_ratios[0] * self.score_config.score_wr_ratio
                    + wr_ratio[0] * (1.0 - self.score_config.score_wr_ratio),
                score_ratios[1] * self.score_config.score_wr_ratio
                    + wr_ratio[1] * (1.0 - self.score_config.score_wr_ratio),
                score_ratios[2] * self.score_config.score_wr_ratio
                    + wr_ratio[2] * (1.0 - self.score_config.score_wr_ratio),
                score_ratios[3] * self.score_config.score_wr_ratio
                    + wr_ratio[3] * (1.0 - self.score_config.score_wr_ratio),
            ],
        }
    }

    pub fn reload_games(&mut self) {
        let games = self.games.clone();
        let players = self.players.keys().clone().cloned().collect::<Vec<_>>();
        self.games.clear();
        self.players.clear();
        for player in players {
            self.register_player(player.clone());
        }
        for (game, winner) in games {
            let _ = self.submit_game(game, winner);
        }
    }

    pub fn submit_game(&mut self, game: Game, winner: String) -> Result<(), GameError> {
        if !game.players.contains(&winner) {
            return Err(GameError::PlayerNotTracked);
        }

        for i in 0..4 {
            let player = &game.players[i];
            let stats = self
                .players
                .get_mut(player)
                .ok_or(GameError::PlayerNotInTournament)?;
            let expected = game.expected[i];
            stats.games += 1;
            if player.eq(&winner) {
                stats.wins += 1;
                stats.elo += self.score_config.game_points * (1.0 - expected) / 0.75
            } else {
                stats.elo -= self.score_config.game_points * (expected / 0.75);
            }
        }
        Ok(())
    }
}

pub enum GameError {
    PlayerNotTracked,
    PlayerNotInTournament,
}

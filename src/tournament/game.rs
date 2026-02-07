use crate::tournament::{Tournament, TournamentError};

const PLAYER_COUNT: usize = 4;
const BASE_EXPECTED: f64 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScoreConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub elo_pow: f64,
    pub wr_pow: f64,
    pub elo_weight: f64,
    pub wr_weight: f64,
}

impl ScoreConfig {
    pub fn new() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            elo_pow: 6.0,
            wr_pow: 1.0,
            elo_weight: 65.0,
            wr_weight: 100.0,
        }
    }

    pub fn new_player_stats(&self) -> PlayerStats {
        PlayerStats {
            elo: self.starting_elo,
            games: 0,
            wins: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

impl PlayerStats {
    pub fn elo(&self) -> f64 {
        self.elo
    }

    pub fn games(&self) -> u32 {
        self.games
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }

    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| (self.wins as f64) / (self.games as f64))
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GamePlayer {
    name: String,
    stats: PlayerStats,
    expected: f64,
}

impl GamePlayer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stats(&self) -> &PlayerStats {
        &self.stats
    }

    pub fn expected(&self) -> f64 {
        self.expected
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameMatch(pub [GamePlayer; PLAYER_COUNT]);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameRecord {
    pub players: [String; 4],
    pub winner: String,
}

impl Tournament {
    pub fn get_score_config(&self) -> &ScoreConfig {
        &self.score_config
    }

    pub fn set_score_config(&mut self, config: ScoreConfig) -> Result<(), TournamentError> {
        self.score_config = config;
        self.reload()?;
        Ok(())
    }

    pub fn get_match_config(&self) -> &crate::tournament::matchmaking::MatchmakerConfig {
        &self.match_config
    }

    pub fn set_match_config(&mut self, config: crate::tournament::matchmaking::MatchmakerConfig) -> Result<(), TournamentError> {
        self.match_config = config;
        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), TournamentError> {
        let games = self.games.clone();
        self.games.clear();
        let players = self.players.keys().cloned().collect::<Vec<_>>();
        self.players.clear();
        for player in players {
            self.register_player(player);
        }

        for game in games {
            self.process_game_record(game)?;
        }

        Ok(())
    }

    pub fn rename_player(&mut self, from: String, to: String) -> Result<(), TournamentError> {
        if !self.has_registered_player(&from) {
            return Err(TournamentError::PlayerNotRegistered(from));
        }
        if self.has_registered_player(&to) {
            return Err(TournamentError::PlayerAlreadyRegistered(to));
        }

        let replace_name = |name: String| if name.eq(&from) { to.clone() } else { name };

        let games = self
            .games
            .iter()
            .map(|game| GameRecord {
                players: game.players.clone().map(replace_name),
                winner: replace_name(game.winner.clone()),
            })
            .collect::<Vec<_>>();

        let players = self
            .players
            .keys()
            .cloned()
            .map(replace_name)
            .collect::<Vec<_>>();

        self.players.clear();
        self.games.clear();

        for player in players {
            self.register_player(player);
        }

        for game in games {
            self.process_game_record(game)?;
        }

        Ok(())
    }

    pub fn remove_player(&mut self, player: String) -> Result<(), TournamentError> {
        if !self.players.contains_key(&player) {
            return Err(TournamentError::PlayerNotRegistered(player));
        }

        let games = self
            .games
            .iter()
            .filter(|game| !game.players.contains(&player))
            .cloned()
            .collect::<Vec<_>>();

        let players = self
            .players
            .keys()
            .filter(|p| !p.eq(&&player))
            .cloned()
            .collect::<Vec<_>>();

        self.players.clear();
        self.games.clear();

        for player in players {
            self.register_player(player);
        }

        for game in games {
            self.process_game_record(game)?;
        }

        Ok(())
    }

    pub fn register_player(&mut self, player: String) {
        self.players
            .insert(player, self.score_config.new_player_stats());
    }

    pub fn create_game<T>(&self, players: [T; PLAYER_COUNT]) -> GameMatch
    where
        T: Into<String>,
    {
        struct PlayerReference {
            name: String,
            stats: PlayerStats,
            scaled_elo: f64,
            scaled_wr: f64,
        }

        let mut players = players.map(|name| PlayerReference {
            name: name.into(),
            stats: self.score_config.new_player_stats(),
            scaled_elo: 0.0,
            scaled_wr: 0.0,
        });

        for player in players.iter_mut() {
            if let Some(stats) = self.players.get(&player.name) {
                player.stats = *stats;
            }

            player.scaled_elo = player.stats.elo.powf(self.score_config.elo_pow);

            let wr = if player.stats.games == 0 {
                BASE_EXPECTED
            } else {
                (player.stats.wins as f64) / (player.stats.games as f64)
            };

            player.scaled_wr = wr.powf(self.score_config.wr_pow);
        }

        let sum_scaled_elo = players.iter().map(|p| p.scaled_elo).sum::<f64>();
        let sum_scaled_wr = players.iter().map(|p| p.scaled_wr).sum::<f64>();

        let weight_total = self.score_config.wr_weight + self.score_config.elo_weight;
        let weight_wr = self.score_config.wr_weight / weight_total;
        let weight_elo = self.score_config.elo_weight / weight_total;

        let coef_wr = weight_wr / sum_scaled_wr;
        let coef_elo = weight_elo / sum_scaled_elo;

        GameMatch(players.map(|player| GamePlayer {
            name: player.name,
            stats: player.stats,
            expected: (coef_wr * player.scaled_wr) + (coef_elo * player.scaled_elo),
        }))
    }

    pub fn submit_game(
        &mut self,
        game: GameMatch,
        winner: impl Into<String>,
    ) -> Result<(), TournamentError> {
        let winner = winner.into();
        let GameMatch(mut players) = game;

        let new_stats = self.score_config.new_player_stats();

        let stats_match = players.iter_mut().all(|player| {
            let stats = self.players.get(&player.name).unwrap_or(&new_stats);
            stats.eq(&player.stats)
        });

        if !stats_match {
            let GameMatch(new_players) = self.create_game(players.map(|p| p.name));
            players = new_players;
        }

        self.process_game(GameMatch(players), winner)
    }

    fn process_game_record(&mut self, record: GameRecord) -> Result<(), TournamentError> {
        self.process_game(self.create_game(record.players), record.winner)
    }

    /// Requires that the GameMatch must be either created immediately, or is accurate.
    fn process_game(&mut self, game: GameMatch, winner: String) -> Result<(), TournamentError> {
        let GameMatch(mut players) = game;

        // Check if winner is one of the players
        let winner_is_player = players.iter().any(|p| p.name.eq(&winner));
        if !winner_is_player {
            return Err(TournamentError::WinnerNotInMatch(winner));
        }

        for player in players.iter_mut() {
            player.stats.games += 1;
            if player.name.eq(&winner) {
                player.stats.wins += 1;
                player.stats.elo += self.score_config.game_points * (1.0 - player.expected) / 0.75;
            } else {
                player.stats.elo -= self.score_config.game_points * (player.expected / 0.75);
            }

            self.players.insert(player.name.clone(), player.stats);
        }

        self.games.push(GameRecord {
            players: players.map(|p| p.name),
            winner,
        });

        Ok(())
    }
}

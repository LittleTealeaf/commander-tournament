use crate::tournament::{Tournament, TournamentError};

const PLAYER_COUNT: usize = 4;
const BASE_EXPECTED: f64 = 0.25;

/// Configuration for the scoring system used in the tournament.
///
/// This struct defines how player ratings and expected win probabilities are calculated.
/// It combines Elo rating and winrate into a unified expected performance metric.
///
/// # Fields
///
/// - `starting_elo`: Initial Elo rating for new players (typically 1500)
/// - `game_points`: Base points awarded/lost per game (typically 25)
/// - `elo_pow`: Exponent for Elo scaling (higher = more nonlinear)
/// - `wr_pow`: Exponent for winrate scaling
/// - `elo_weight`: Weight factor for Elo in expected calculation
/// - `wr_weight`: Weight factor for winrate in expected calculation
///
/// The expected win probability is calculated by normalizing weighted Elo and winrate components.
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
    /// Creates a new default scoring configuration.
    ///
    /// # Default Values
    ///
    /// - Starting Elo: 1500.0
    /// - Game Points: 25.0
    /// - Elo Power: 6.0
    /// - WR Power: 1.0
    /// - Elo Weight: 65.0
    /// - WR Weight: 100.0
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

    /// Creates a new player with default stats based on this configuration.
    pub fn new_player_stats(&self) -> PlayerStats {
        PlayerStats {
            elo: self.starting_elo,
            games: 0,
            wins: 0,
        }
    }
}

impl Default for ScoreConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a player in the tournament.
///
/// Tracks cumulative performance metrics:
/// - Elo rating (affects expected outcome in games)
/// - Total games played
/// - Total games won
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
}

impl PlayerStats {
    /// Returns the player's current Elo rating.
    pub fn elo(&self) -> f64 {
        self.elo
    }

    /// Returns the total number of games played by this player.
    pub fn games(&self) -> u32 {
        self.games
    }

    /// Returns the total number of games won by this player.
    pub fn wins(&self) -> u32 {
        self.wins
    }

    /// Calculates the player's win rate as a decimal (0.0 to 1.0).
    ///
    /// Returns `None` if the player has not played any games yet.
    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| (self.wins as f64) / (self.games as f64))
    }
}

/// Represents a player in a specific game with their expected performance.
///
/// Captures a snapshot of a player's stats at the time a game is created,
/// along with the calculated expected win probability for that match.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GamePlayer {
    name: String,
    stats: PlayerStats,
    expected: f64,
}

impl GamePlayer {
    /// Returns the player's name.
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the player's stats at the time this game was created.
    pub fn stats(&self) -> &PlayerStats {
        &self.stats
    }

    /// Returns the expected win probability for this player in this match.
    ///
    /// This is a value between 0.0 (expected to lose) and 1.0 (expected to win).
    pub fn expected(&self) -> f64 {
        self.expected
    }
}

/// A match between exactly 4 players with their expected performance metrics.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameMatch(pub [GamePlayer; PLAYER_COUNT]);

/// A record of a completed game with the players and the winner.
///
/// This is used to store game history and enable replay of games
/// when score configuration changes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameRecord {
    pub players: [String; 4],
    pub winner: String,
}

impl Tournament {
    /// Returns the current scoring configuration.
    pub fn get_score_config(&self) -> &ScoreConfig {
        &self.score_config
    }

    /// Updates the scoring configuration and recalculates all player ratings.
    ///
    /// This will replay all games with the new configuration to ensure consistency.
    /// This is an expensive operation and should not be called frequently.
    ///
    /// # Returns
    ///
    /// Returns an error if the configuration change causes issues during replay.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut config = tournament.get_score_config().clone();
    /// config.game_points = 15.0;
    /// tournament.set_score_config(config)?;
    /// ```
    pub fn set_score_config(&mut self, config: ScoreConfig) -> Result<(), TournamentError> {
        self.score_config = config;
        self.reload()?;
        Ok(())
    }

    /// Returns the current matchmaking configuration.
    pub fn get_match_config(&self) -> &crate::tournament::matchmaking::MatchmakerConfig {
        &self.match_config
    }

    /// Updates the matchmaking configuration.
    ///
    /// This does not affect existing games or player ratings, only future matchmaking.
    pub fn set_match_config(&mut self, config: crate::tournament::matchmaking::MatchmakerConfig) -> Result<(), TournamentError> {
        self.match_config = config;
        Ok(())
    }

    /// Recalculates all player ratings from the game history.
    ///
    /// This is useful after changing the scoring configuration or if you need to
    /// ensure consistency. This is an expensive operation that replays all games.
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the total number of games. Only call when necessary.
    ///
    /// # Errors
    ///
    /// Returns an error if a game record is invalid (e.g., winner not in match).
    pub fn reload(&mut self) -> Result<(), TournamentError> {
        let games = std::mem::take(&mut self.games);
        let player_names: Vec<_> = self.players.keys().cloned().collect();
        self.players.clear();
        for player in player_names {
            self.register_player(player);
        }

        for game in games {
            self.process_game_record(game)?;
        }

        Ok(())
    }

    /// Renames a player, updating all game records and statistics.
    ///
    /// This operation updates:
    /// - The player name in all game records
    /// - The player entry in the player stats map
    /// - References in all historical games
    ///
    /// # Arguments
    ///
    /// * `from` - The current player name
    /// * `to` - The new player name
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `from` player does not exist
    /// - `to` player already exists
    ///
    /// # Performance
    ///
    /// This is O(n) where n is the total number of games.
    pub fn rename_player(&mut self, from: String, to: String) -> Result<(), TournamentError> {
        if !self.has_registered_player(&from) {
            return Err(TournamentError::PlayerNotRegistered(from));
        }
        if self.has_registered_player(&to) {
            return Err(TournamentError::PlayerAlreadyRegistered(to));
        }

        // Get the stats for the player we're renaming before clearing
        let from_stats = self.players.get(&from).copied();

        // Update games in place
        for game in self.games.iter_mut() {
            for player in game.players.iter_mut() {
                if player == &from {
                    *player = to.clone();
                }
            }
            if game.winner == from {
                game.winner = to.clone();
            }
        }

        // Update player map
        self.players.remove(&from);
        if let Some(stats) = from_stats {
            self.players.insert(to, stats);
        }

        Ok(())
    }

    /// Removes a player and all their games from the tournament.
    ///
    /// This removes:
    /// - The player from the stats map
    /// - All games where this player participated
    ///
    /// # Arguments
    ///
    /// * `player` - The name of the player to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the player does not exist.
    ///
    /// # Performance
    ///
    /// This is O(n) where n is the total number of games.
    pub fn remove_player(&mut self, player: String) -> Result<(), TournamentError> {
        if !self.players.contains_key(&player) {
            return Err(TournamentError::PlayerNotRegistered(player));
        }

        // Remove games that contain this player
        self.games.retain(|game| !game.players.contains(&player));

        // Remove player from the map
        self.players.remove(&player);

        Ok(())
    }

    /// Registers a new player in the tournament with default statistics.
    ///
    /// The player starts with the default Elo rating from the current [`ScoreConfig`].
    ///
    /// # Arguments
    ///
    /// * `player` - The name of the player to register
    ///
    /// # Note
    ///
    /// This overwrites any existing player with the same name. Use with caution in production code.
    pub fn register_player(&mut self, player: String) {
        self.players
            .insert(player, self.score_config.new_player_stats());
    }

    /// Creates a match for 4 players with calculated expected win probabilities.
    ///
    /// # Arguments
    ///
    /// Takes 4 player names and looks up their current statistics to calculate
    /// expected performance metrics for a prospective match.
    ///
    /// # Expected Probability
    ///
    /// The expected win probability for each player is calculated as:
    /// - Both Elo and winrate are exponentially scaled according to configuration
    /// - The scaled values are weighted (elo_weight vs wr_weight)
    /// - Each player's scaled value is divided by the sum to get a 0-1 probability
    ///
    /// # Returns
    ///
    /// A [`GameMatch`] with the 4 players and their expected stats.
    ///
    /// # Performance
    ///
    /// This is O(1) - constant time lookups and calculations.
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

        // Convert input to strings and look up stats in one pass
        let mut player_refs = players.map(|name| {
            let name_str = name.into();
            let stats = self
                .players
                .get(&name_str)
                .copied()
                .unwrap_or_else(|| self.score_config.new_player_stats());
            PlayerReference {
                name: name_str,
                stats,
                scaled_elo: 0.0,
                scaled_wr: 0.0,
            }
        });

        // Pre-calculate sums in one pass
        let (sum_scaled_elo, sum_scaled_wr) = {
            let mut sum_elo = 0.0;
            let mut sum_wr = 0.0;

            for player in player_refs.iter_mut() {
                player.scaled_elo = player.stats.elo.powf(self.score_config.elo_pow);

                let wr = if player.stats.games == 0 {
                    BASE_EXPECTED
                } else {
                    (player.stats.wins as f64) / (player.stats.games as f64)
                };

                player.scaled_wr = wr.powf(self.score_config.wr_pow);

                sum_elo += player.scaled_elo;
                sum_wr += player.scaled_wr;
            }
            (sum_elo, sum_wr)
        };

        let weight_total = self.score_config.wr_weight + self.score_config.elo_weight;
        let weight_wr = self.score_config.wr_weight / weight_total;
        let weight_elo = self.score_config.elo_weight / weight_total;

        let coef_wr = weight_wr / sum_scaled_wr;
        let coef_elo = weight_elo / sum_scaled_elo;

        GameMatch(player_refs.map(|player| GamePlayer {
            name: player.name,
            stats: player.stats,
            expected: (coef_wr * player.scaled_wr) + (coef_elo * player.scaled_elo),
        }))
    }

    /// Submits a game result to the tournament.
    ///
    /// This records the game in history and updates all players' statistics.
    /// Elo ratings are adjusted based on the outcome and expected performance.
    ///
    /// # Arguments
    ///
    /// * `game` - The match to submit (created by `create_game`)
    /// * `winner` - The name of the winning player
    ///
    /// # Elo Calculation
    ///
    /// The Elo adjustment formula is:
    /// - Winner: `elo += game_points * (1.0 - expected) / 0.75`
    /// - Loser: `elo -= game_points * expected / 0.75`
    ///
    /// The division by 0.75 adjusts from a 4-player game back to a standard 2-player equivalent.
    ///
    /// # Validation
    ///
    /// If player statistics have changed since the game was created (unlikely in normal usage),
    /// the game will be re-evaluated with current stats to ensure consistency.
    ///
    /// # Errors
    ///
    /// Returns an error if the winner is not one of the players in the match.
    ///
    /// # Performance
    ///
    /// This is O(1) for the lookup and update operations.
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

    /// Internally processes a game and updates all player statistics.
    ///
    /// # Internal Implementation Details
    ///
    /// - Increments game count for all players
    /// - Increments win count for the winner
    /// - Adjusts Elo ratings based on expected performance
    /// - Stores the game record in history
    ///
    /// # Errors
    ///
    /// Returns an error if the winner is not in the match.
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

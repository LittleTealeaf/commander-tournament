mod errors;
mod game;
mod matchmaking;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use errors::*;
pub use game::*;
pub use matchmaking::MatchmakerConfig;

/// Manages a tournament with player statistics and game records.
///
/// `Tournament` maintains a collection of players with their statistics (Elo rating, win/loss records)
/// and a history of games. It uses configurable scoring and matchmaking strategies to create
/// competitive matches between players.
///
/// # Scoring System
///
/// The tournament uses an Elo-based rating system with two components:
/// - **Elo Rating**: Traditional Elo rating that changes based on game outcomes
/// - **Winrate**: Historical win/loss ratio that influences expected performance
///
/// Both factors are weighted according to [`ScoreConfig`] and combined to calculate
/// expected win probability for each player in a match.
///
/// # Matchmaking
///
/// The tournament provides multiple matchmaking strategies through [`MatchmakerConfig`]:
/// - **Combined**: Weighted combination of all strategies
/// - **Least Played**: Prioritize opponents played least frequently
/// - **Nemesis**: Find opponents with best win records against you
/// - **WR Neighbors**: Match with players of similar winrate
/// - **Neighbors**: Match with players of similar Elo rating
/// - **Loss With**: Find opponents you struggle against together
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    players: HashMap<String, PlayerStats>,
    games: Vec<GameRecord>,
    score_config: ScoreConfig,
    match_config: MatchmakerConfig,
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}

impl Tournament {
    /// Creates a new empty tournament with default configuration.
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: Vec::new(),
            score_config: ScoreConfig::new(),
            match_config: MatchmakerConfig::default(),
        }
    }

    /// Checks if a player is registered in the tournament.
    pub fn has_registered_player(&self, player: &str) -> bool {
        self.players.contains_key(player)
    }

    /// Returns a reference to all registered players and their statistics.
    pub fn players(&self) -> &HashMap<String, PlayerStats> {
        &self.players
    }

    /// Returns a reference to the game history.
    pub fn games(&self) -> &Vec<GameRecord> {
        &self.games
    }

    /// Change the winner for a game at the provided index and reload stats.
    pub fn set_game_winner(&mut self, index: usize, winner: String) -> Result<(), TournamentError> {
        if index >= self.games.len() {
            return Err(TournamentError::GameNotFound(index));
        }
        self.games[index].winner = winner;
        self.reload()?;
        Ok(())
    }

    /// Delete a game at index and reload stats.
    pub fn delete_game(&mut self, index: usize) -> Result<(), TournamentError> {
        if index >= self.games.len() {
            return Err(TournamentError::GameNotFound(index));
        }
        self.games.remove(index);
        self.reload()?;
        Ok(())
    }
}

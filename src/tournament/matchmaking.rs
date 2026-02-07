use itertools::{Itertools, chain};

use crate::tournament::{GameMatch, PlayerStats, Tournament, TournamentError};

/// Configuration for matchmaking strategies.
///
/// Defines weights for combining multiple matchmaking strategies into a single
/// "best" opponent ranking. Each weight controls how much a particular strategy
/// influences the final ranking.
///
/// # Fields
///
/// - `weight_least_played`: Players you've played least (encourages variety)
/// - `weight_nemesis`: Players with best head-to-head records against you
/// - `weight_neighbor`: Players with Elo rating closest to yours
/// - `weight_wr_neighbor`: Players with winrate closest to yours
/// - `weight_lost_with`: Players you tend to lose/win with (symmetry in outcomes)
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchmakerConfig {
    pub weight_least_played: f64,
    pub weight_nemesis: f64,
    pub weight_neighbor: f64,
    pub weight_wr_neighbor: f64,
    pub weight_lost_with: f64,
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self {
            weight_least_played: 6.0,
            weight_nemesis: 4.0,
            weight_neighbor: 5.0,
            weight_wr_neighbor: 3.0,
            weight_lost_with: 3.0,
        }
    }
}

macro_rules! impl_game_creator {
    ($method_name:ident, $rank_method: ident) => {
        #[allow(dead_code)]
        pub fn $method_name(&mut self, player: &str) -> Result<GameMatch, TournamentError> {
            let mut iter = self.$rank_method(player)?;
            let p2 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let p3 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let p4 = iter.next().ok_or(TournamentError::NotEnoughPlayers)?;
            Ok(self.create_game([player.to_string(), p2, p3, p4]))
        }
    };
}

impl Tournament {
    fn is_registered(&self, player: &str) -> Result<(), TournamentError> {
        if !self.has_registered_player(player) {
            return Err(TournamentError::PlayerNotRegistered(player.to_string()));
        }
        Ok(())
    }

    fn get_player_stats<'a>(&'a self, player: &str) -> Result<&'a PlayerStats, TournamentError> {
        self.players
            .get(player)
            .ok_or_else(|| TournamentError::PlayerNotRegistered(player.to_string()))
    }

    /// Ranks players by frequency of games played with the given player.
    ///
    /// Returns opponents in ascending order of play count (least played first).
    /// When players have been played equally, uses Elo proximity as a tie-breaker
    /// (closest Elo rating to the player first).
    ///
    /// # Strategy Benefits
    ///
    /// - Encourages variety in opponents
    /// - Gives underrepresented matchups higher priority
    /// - Tie-breaker balances Elo competition
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank opponents for
    ///
    /// # Returns
    ///
    /// An iterator of opponent names sorted by this strategy, or error if player not found.
    pub fn rank_least_played(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        self.is_registered(player)?;
        let player_stats = self.get_player_stats(player)?;
        let player_elo = player_stats.elo();

        // Initialize all registered players with count of 0
        let mut opponent_counts: std::collections::HashMap<String, usize> =
            self.players
                .keys()
                .filter(|p| *p != player)
                .map(|p| (p.clone(), 0))
                .collect();

        // Count actual games played
        for game in self.games.iter() {
            if game.players.contains(&String::from(player)) {
                for opponent in game.players.iter() {
                    if opponent != player {
                        *opponent_counts.entry(opponent.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        Ok(opponent_counts
            .into_iter()
            .sorted_by(|(p1, count1), (p2, count2)| {
                // First sort by play count
                match count1.cmp(count2) {
                    std::cmp::Ordering::Equal => {
                        // Tie-breaker: sort by ELO difference (ascending)
                        let stats_p1 = self.get_player_stats(p1).map(|s| s.elo()).unwrap_or(0.0);
                        let stats_p2 = self.get_player_stats(p2).map(|s| s.elo()).unwrap_or(0.0);
                        let diff1 = (player_elo - stats_p1).abs();
                        let diff2 = (player_elo - stats_p2).abs();
                        diff1.total_cmp(&diff2)
                    }
                    other => other,
                }
            })
            .map(|(opponent, _)| opponent))
    }

    impl_game_creator!(game_least_played, rank_least_played);

    /// Ranks players that are your "nemesis" (beat you most or you beat least).
    ///
    /// For each opponent, calculates a "nemesis score":
    /// - +1 for each game they beat you
    /// - -1 for each game in which you beat them
    ///
    /// Returns players sorted by nemesis score (lowest/most losses first),
    /// with Elo as a tie-breaker (lowest Elo first).
    ///
    /// # Strategy Benefits
    ///
    /// - Targets players who give you the most trouble
    /// - Can help you overcome problematic matchups
    /// - Provides specific competitive focus
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank nemesis opponents for
    ///
    /// # Returns
    ///
    /// An iterator of opponent names sorted by nemesis factor.
    pub fn rank_nemesis(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        self.is_registered(player)?;

        // Initialize all registered players with score of 0
        let mut opponent_scores: std::collections::HashMap<String, i32> =
            self.players
                .keys()
                .filter(|p| *p != player)
                .map(|p| (p.clone(), 0))
                .collect();

        for game in self.games.iter() {
            if game.players.contains(&String::from(player)) {
                if player == game.winner {
                    // Player won, so reduce score for all other players (they lost)
                    for opponent in game.players.iter() {
                        if opponent != player {
                            *opponent_scores.entry(opponent.clone()).or_insert(0) -= 1;
                        }
                    }
                } else {
                    // Player lost, increase score for winner
                    *opponent_scores.entry(game.winner.clone()).or_insert(0) += 1;
                }
            }
        }

        Ok(opponent_scores
            .into_iter()
            .sorted_by(|(p1, score1), (p2, score2)| {
                // First sort by win/loss score (descending - highest nemesis score first)
                match score2.cmp(score1) {
                    std::cmp::Ordering::Equal => {
                        // Tie-breaker: sort by ELO (ascending - lowest ELO first)
                        let stats_p1 = self.get_player_stats(p1).map(|s| s.elo()).unwrap_or(0.0);
                        let stats_p2 = self.get_player_stats(p2).map(|s| s.elo()).unwrap_or(0.0);
                        stats_p1.total_cmp(&stats_p2)
                    }
                    other => other,
                }
            })
            .map(|(opponent, _)| opponent))
    }

    impl_game_creator!(game_nemesis, rank_nemesis);

    /// Ranks players by similarity in win rate (winrate neighbors).
    ///
    /// Returns all other players without any particular ordering for now.
    /// This is a placeholder that should be enhanced to actually sort by
    /// winrate proximity.
    ///
    /// # Strategy Benefits
    ///
    /// - Matches players of similar skill/consistency
    /// - Balances competitive environment
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank neighbors for
    pub fn rank_wr_neighbors(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        let _stats = self.get_player_stats(player)?;

        Ok(self
            .players
            .iter()
            .filter(|&(p, _)| p != player)
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>()
            .into_iter())
    }

    impl_game_creator!(game_wr_neighbors, rank_wr_neighbors);

    /// Ranks players by Elo rating proximity (Elo neighbors).
    ///
    /// Returns all other players without any particular ordering for now.
    /// This is a placeholder that should be enhanced to actually sort by
    /// Elo proximity.
    ///
    /// # Strategy Benefits
    ///
    /// - Matches players of similar Elo rating
    /// - Balanced 50/50 expected outcomes
    /// - Competitive matching
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank neighbors for
    pub fn rank_neighbors(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        let _stats = self.get_player_stats(player)?;

        Ok(self
            .players
            .iter()
            .filter(|&(p, _)| p != player)
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>()
            .into_iter())
    }

    impl_game_creator!(game_neighbors, rank_neighbors);

    /// Ranks players by relative win/loss outcome when playing together.
    ///
    /// For each opponent, calculates a "loss_with score":
    /// - Score = games_played - (2 * losses_with_player)
    /// - Higher scores mean the player wins more often with that opponent
    /// - Lower scores mean the player loses more with that opponent
    ///
    /// Returns opponents sorted by this score (ascending), with tie-breakers:
    /// 1. Games played together (more games = higher priority)
    /// 2. Elo proximity (closer Elo = higher priority)
    ///
    /// # Strategy Benefits
    ///
    /// - Identifies partners you struggle against
    /// - Can reveal specific weaknesses
    /// - Creates rematches with problematic teams
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank for
    ///
    /// # Returns
    ///
    /// An iterator of opponent names sorted by this strategy.
    pub fn rank_loss_with(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        self.is_registered(player)?;
        let player_elo = self.get_player_stats(player)?.elo();

        // Initialize all registered players with default stats
        let mut opponent_stats: std::collections::HashMap<String, (usize, usize)> =
            self.players
                .keys()
                .filter(|p| *p != player)
                .map(|p| (p.clone(), (0, 0)))
                .collect();

        // Single pass through games to collect stats
        for game in self.games.iter().filter(|g| g.players.contains(&String::from(player))) {
            for opponent in game.players.iter().filter(|p| p.as_str() != player) {
                let entry = opponent_stats.entry(opponent.clone()).or_insert((0, 0));
                entry.0 += 1; // games played together
                if game.winner != player {
                    entry.1 += 1; // loss for focus player
                }
            }
        }

        Ok(opponent_stats
            .into_iter()
            .map(|(opponent, (x, y))| {
                let score = (x as i32) - (2 * y as i32);
                (opponent, score, x)
            })
            .sorted_by(|(p1, score1, x1), (p2, score2, x2)| {
                // First sort by loss_with score (ascending - lower scores first)
                match score1.cmp(score2) {
                    std::cmp::Ordering::Equal => {
                        // Tie-breaker 1: sort by games played together (x) (descending - more games first)
                        match x2.cmp(x1) {
                            std::cmp::Ordering::Equal => {
                                // Tie-breaker 2: sort by ELO difference (ascending - closest ELO first)
                                let stats_p1 = self.get_player_stats(p1).map(|s| s.elo()).unwrap_or(0.0);
                                let stats_p2 = self.get_player_stats(p2).map(|s| s.elo()).unwrap_or(0.0);
                                let diff1 = (player_elo - stats_p1).abs();
                                let diff2 = (player_elo - stats_p2).abs();
                                diff1.total_cmp(&diff2)
                            }
                            other => other,
                        }
                    }
                    other => other,
                }
            })
            .map(|(opponent, _, _)| opponent))
    }

    impl_game_creator!(game_loss_with, rank_loss_with);

    /// Ranks players using a weighted combination of all strategies.
    ///
    /// Combines all ranking strategies with weights configured in [`MatchmakerConfig`]:
    /// - Least Played (default weight: 6.0)
    /// - Nemesis (default weight: 4.0)
    /// - Neighbors (default weight: 5.0)
    /// - WR Neighbors (default weight: 3.0)
    /// - Loss With (default weight: 3.0)
    ///
    /// Each strategy is ranked and weighted, then the results are summed.
    /// Lower total scores = higher priority.
    ///
    /// # Strategy Benefits
    ///
    /// - Balances multiple strategic concerns
    /// - Adaptive to tournament state
    /// - Respects all different play styles
    ///
    /// # Arguments
    ///
    /// * `player` - The player to rank opponents for
    ///
    /// # Returns
    ///
    /// An iterator of opponent names sorted by combined weighted score.
    pub fn rank_combined(
        &self,
        player: &str,
    ) -> Result<impl Iterator<Item = String>, TournamentError> {
        fn to_weight(weight: f64) -> impl Fn((usize, String)) -> (String, f64) {
            move |(score, player)| (player, (score as u64) as f64 * weight)
        }

        Ok(chain!(
            self.rank_least_played(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_least_played)),
            self.rank_nemesis(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_nemesis)),
            self.rank_neighbors(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_neighbor)),
            self.rank_wr_neighbors(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_wr_neighbor)),
            self.rank_loss_with(player)?
                .enumerate()
                .map(to_weight(self.match_config.weight_lost_with)),
        )
        .into_grouping_map()
        .sum()
        .into_iter()
        .filter(|(p, _)| p != player)
        .sorted_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(p, _)| p))
    }

    impl_game_creator!(game_combined, rank_combined);
}

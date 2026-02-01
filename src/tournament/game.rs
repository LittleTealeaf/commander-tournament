use crate::tournament::{Tournament, TournamentError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScoreConfig {
    pub starting_elo: f64,
    pub game_points: f64,
    pub elo_pow: f64,
    pub wr_pow: f64,
    pub elo_weight: f64,
    pub wr_wright: f64,
}

impl ScoreConfig {
    pub fn new() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            elo_pow: 6.0,
            wr_pow: 1.0,
            elo_weight: 65.0,
            wr_wright: 100.0,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerStats {
    pub elo: f64,
    pub games: u32,
    pub wins: u32,
}

impl PlayerStats {
    pub fn new(starting_elo: f64) -> Self {
        Self {
            elo: starting_elo,
            games: 0,
            wins: 0,
        }
    }
}

pub struct GamePlayer {
    name: String,
    stats: PlayerStats,
    expected: f64,
}

pub struct GameMatch([GamePlayer; 4]);

pub struct GameRecord {
    players: Vec<String>,
    winner: String,
}

impl Tournament {
    pub fn register_player(&mut self, player: String) {
        self.players
            .insert(player, self.score_config.new_player_stats());
    }

    pub fn create_game(&mut self, players: [String; 4]) -> Result<GameMatch, TournamentError> {
        // if players.len() < 2 {
        //     return Err(TournamentError::NotEnoughPlayers(players.len()));
        // }

        // let count = players.len();
        // let count_f64 = count as f64;
        // let base_expected = 1.0 / count_f64;

        // struct PlayerInstance {
        //     player: GamePlayer,
        //     wr_scaled: f64,
        //     elo_scaled: f64,
        // }

        // // Grabs the player stats, creating if configured to do so.
        // let mut game_players = players
        //     .iter()
        //     .map(|player| {
        //         self.players
        //             .get(player)
        //             .cloned()
        //             .or_else(|| {
        //                 self.auto_register_players.then(|| {
        //                     let new_stats = self.score_config.new_player_stats();
        //                     self.players.insert(player.clone(), new_stats);
        //                     new_stats
        //                 })
        //             })
        //             .map(|stats| {
        //                 let wr = if stats.games == 0

        //                 PlayerInstance {
        //                     player: GamePlayer {
        //                     name: player.clone(),
        //                     stats,
        //                     expected: base_expected,
        //                     }
        //                 }
        //             })
        //             .ok_or(TournamentError::InvalidPlayer(player.clone()))
        //     })
        //     .collect::<Result<Vec<_>, _>>()?;



        todo!()
    }
}

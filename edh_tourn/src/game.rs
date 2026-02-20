use crate::{
    Tournament,
    error::{TournResult, TournamentError},
    stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Copy, Hash)]
pub struct GameRecord {
    #[serde(rename = "p")]
    players: [u32; 4],
    #[serde(rename = "w")]
    winner: u32,
}

impl GameRecord {
    pub fn new(players: [u32; 4], winner: u32) -> Result<Self, TournamentError> {
        if !players.contains(&winner) {
            return Err(TournamentError::WinnerNotInMatch(winner));
        }

        Ok(Self { players, winner })
    }

    #[must_use]
    pub const fn players(&self) -> &[u32; 4] {
        &self.players
    }

    #[must_use]
    pub const fn winner(&self) -> u32 {
        self.winner
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MatchPlayer {
    id: u32,
    stats: PlayerStats,
    expected: f64,
    elo_win: f64,
    elo_loss: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Matchup {
    players: [MatchPlayer; 4],
    config_version: usize,
}

impl Matchup {
    #[must_use]
    pub fn get_ids(&self) -> [u32; 4] {
        self.players.clone().map(|player| player.id)
    }

    pub fn create_record(&self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self.get_ids(), winner)
    }
}

impl Tournament {
    pub fn create_match(&self, ids: [u32; 4]) -> Result<Matchup, TournamentError> {
        struct TempMatchPlayer<'a> {
            id: u32,
            stats: &'a PlayerStats,
            scaled_elo: f64,
            scaled_wr: f64,
        }

        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        let default_stats = self.create_default_stats();

        let players = ids.map(|id| {
            let stats = self.get_player_stats(id).unwrap_or(&default_stats);
            TempMatchPlayer {
                scaled_wr: stats
                    .wr()
                    .unwrap_or(0.25)
                    .powf(self.config.game_wr_pow_scale),
                scaled_elo: stats.elo().powf(self.config.game_elo_pow_scale),
                stats,
                id,
            }
        });

        let sum_elo = players.iter().map(|player| player.scaled_elo).sum::<f64>();
        let sum_wr = players.iter().map(|player| player.scaled_wr).sum::<f64>();

        let weight_total = self.config.game_wr_weight + self.config.game_elo_weight;
        let weight_wr = self.config.game_wr_weight / weight_total;
        let weight_elo = self.config.game_elo_weight / weight_total;

        let coef_wr = weight_wr / sum_wr;
        let coef_elo = weight_elo / sum_elo;

        let match_players = players.map(|player| {
            let expected = coef_wr.mul_add(player.scaled_wr, coef_elo * player.scaled_elo);
            let elo_win = self.config.game_points * (1.0 - expected) / 0.75;
            let elo_loss = self.config.game_points * expected / 0.75;

            MatchPlayer {
                id: player.id,
                stats: player.stats.clone(),
                expected,
                elo_win,
                elo_loss,
            }
        });

        Ok(Matchup {
            players: match_players,
            config_version: self.config().version,
        })
    }

    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.config_version == self.config.version {
            return Ok(matchup);
        }
        self.create_match(matchup.players.map(|player| player.id))
    }

    pub fn register_record(&mut self, record: GameRecord) -> Result<(), TournamentError> {
        self.register_match(self.create_match(record.players)?, record.winner)
    }

    pub fn register_match(&mut self, matchup: Matchup, winner: u32) -> Result<(), TournamentError> {
        let matchup = self.update_match(matchup)?;
        let record = matchup.create_record(winner)?;

        let default_stats = self.create_default_stats();

        for player in matchup.players {
            let stats = self
                .stats
                .entry(player.id)
                .or_insert_with(|| default_stats.clone());

            stats.games += 1;

            if player.id == winner {
                stats.wins += 1;
                stats.elo += player.elo_win;
            } else {
                stats.elo -= player.elo_loss;
            }
        }

        self.games.push(record);

        Ok(())
    }

    #[must_use]
    pub const fn games(&self) -> &Vec<GameRecord> {
        &self.games
    }

    pub fn delete_game(&mut self, gid: usize) -> TournResult<()> {
        if gid >= self.games.len() {
            return Err(TournamentError::GameNotFound(gid));
        }
        self.games.remove(gid);
        self.reload()?;
        Ok(())
    }
}

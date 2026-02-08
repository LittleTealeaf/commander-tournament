use crate::{Tournament, error::TournamentError, stats::PlayerStats};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    pub fn players(&self) -> &[u32; 4] {
        &self.players
    }

    pub fn winner(&self) -> u32 {
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
    pub fn get_ids(&self) -> [u32; 4] {
        let mut ids = [0; 4];
        (0..4).for_each(|i| ids[i] = self.players[i].id);
        ids
    }

    pub fn create_record(&self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self.get_ids(), winner)
    }
}

impl Tournament {
    pub fn create_match(&self, ids: [u32; 4]) -> Result<Matchup, TournamentError> {
        let default_stats = self.create_default_stats();
        let mut stats = [&default_stats; 4];

        let mut scaled_elo = [0.0; 4];
        let mut scaled_wr = [0.0; 4];

        for i in 0..4 {
            let id = ids[i];
            if !self.is_id_registered(&id) {
                return Err(TournamentError::InvalidPlayerId(id));
            }
            if let Some(ps) = self.get_player_stats(&id) {
                stats[i] = ps;
            }

            scaled_elo[i] = stats[i].elo.powf(self.config.game_elo_pow_scale);
            scaled_wr[i] = stats[i]
                .wr()
                .unwrap_or(0.25)
                .powf(self.config.game_wr_pow_scale);
        }

        let sum_elo = scaled_elo.iter().sum::<f64>();
        let sum_wr = scaled_wr.iter().sum::<f64>();

        let mut match_players: [MatchPlayer; 4] = Default::default();

        let weight_total = self.config.game_wr_weight + self.config.game_elo_weight;
        let weight_wr = self.config.game_wr_weight / weight_total;
        let weight_elo = self.config.game_elo_weight / weight_total;

        let coef_wr = weight_wr / sum_wr;
        let coef_elo = weight_elo / sum_elo;

        for i in 0..4 {
            let id = ids[i];
            let p = &mut match_players[i];
            p.id = id;
            p.stats = stats[i].clone();
            p.expected = (coef_wr * scaled_wr[i]) + (coef_elo * scaled_elo[i]);
            p.elo_win = self.config.game_points * (1.0 - p.expected) / 0.75;
            p.elo_loss = self.config.game_points * (p.expected) / 0.75;
        }

        Ok(Matchup {
            players: match_players,
            config_version: self.config.version,
        })
    }

    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.config_version == self.config.version {
            return Ok(matchup);
        }
        let mut ids = [0; 4];
        (0..4).for_each(|i| {
            ids[i] = matchup.players[i].id;
        });
        self.create_match(ids)
    }

    pub fn register_record(&mut self, record: GameRecord) -> Result<(), TournamentError> {
        self.register_match(self.create_match(record.players)?, record.winner)
    }

    pub fn register_match(&mut self, matchup: Matchup, winner: u32) -> Result<(), TournamentError> {
        let matchup = self.update_match(matchup)?;
        let record = matchup.create_record(winner)?;
        let default_stats = self.create_default_stats();

        for i in 0..4 {
            let id = record.players[i];
            let stats = self
                .stats
                .entry(id)
                .or_insert_with(|| default_stats.clone());

            stats.games += 1;
            if id == record.winner {
                stats.wins += 1;
                stats.elo += matchup.players[i].elo_win;
            } else {
                stats.elo += matchup.players[i].elo_loss;
            }
        }

        self.games.push(record);

        Ok(())
    }
}

use std::collections::HashMap;

use crate::{
    Tournament,
    error::{TournResult, TournamentError},
    stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Copy)]
pub struct GameRecord {
    #[serde(rename = "p")]
    players: [u32; 4],
    #[serde(rename = "w")]
    winner: u32,
    #[serde(skip)]
    change_elo: Option<[f64; 4]>,
}

impl GameRecord {
    pub fn new(players: [u32; 4], winner: u32) -> Result<Self, TournamentError> {
        Self::with_scores(players, winner, None)
    }

    fn with_scores(
        players: [u32; 4],
        winner: u32,
        change_elo: Option<[f64; 4]>,
    ) -> Result<Self, TournamentError> {
        if !players.contains(&winner) {
            return Err(TournamentError::WinnerNotInMatch(winner));
        }

        Ok(Self {
            players,
            winner,
            change_elo,
        })
    }

    #[must_use]
    pub const fn players(&self) -> &[u32; 4] {
        &self.players
    }

    #[must_use]
    pub const fn winner(&self) -> u32 {
        self.winner
    }

    pub fn map_ids(&self, ids: &HashMap<u32, u32>) -> Result<Self, TournamentError> {
        let [a, b, c, d] = self.players;
        let a = ids.get(&a).ok_or(TournamentError::InvalidPlayerId(a))?;
        let b = ids.get(&b).ok_or(TournamentError::InvalidPlayerId(b))?;
        let c = ids.get(&c).ok_or(TournamentError::InvalidPlayerId(c))?;
        let d = ids.get(&d).ok_or(TournamentError::InvalidPlayerId(d))?;
        let winner = ids
            .get(&self.winner)
            .ok_or(TournamentError::InvalidPlayerId(self.winner))?;

        Self::new([*a, *b, *c, *d], *winner)
    }

    #[must_use]
    pub const fn elo_changes(&self) -> &Option<[f64; 4]> {
        &self.change_elo
    }

    pub fn get_player_elo_change(&self, id: &u32) -> Result<f64, TournamentError> {
        if !self.players.contains(id) {
            return Err(TournamentError::InvalidPlayerId(*id));
        }
        let [a, b, c, d] = &self.players;
        let Some([ea, eb, ec, ed]) = &self.change_elo else {
            return Err(TournamentError::RecordNoEloData);
        };
        let mut chg_elo = 0.0;
        if a == id {
            chg_elo += ea;
        }
        if b == id {
            chg_elo += eb;
        }
        if c == id {
            chg_elo += ec;
        }
        if d == id {
            chg_elo += ed;
        }
        Ok(chg_elo)
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
        let elo_changes = self.players.clone().map(|player| {
            if player.id == winner {
                player.elo_win
            } else {
                player.elo_loss
            }
        });
        GameRecord::with_scores(self.get_ids(), winner, Some(elo_changes))
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

        let mut winner_tracked = false;

        for player in matchup.players {
            let stats = self
                .stats
                .entry(player.id)
                .or_insert_with(|| default_stats.clone());

            stats.games += 1;

            if !winner_tracked && player.id == winner {
                stats.wins += 1;
                stats.elo += player.elo_win;
                winner_tracked = true;
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

    pub fn get_player_games(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = &GameRecord>, TournamentError> {
        if !self.is_id_registered(&id) {
            return Err(TournamentError::InvalidPlayerId(id));
        }

        Ok(self
            .games()
            .iter()
            .filter(move |game| game.players().contains(&id)))
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

#[cfg(test)]
mod tests {
    #![allow(clippy::indexing_slicing)]

    use itertools::Itertools;

    use crate::{Tournament, game::GameRecord};

    #[test]
    fn game_record_without_scores() {
        let record = GameRecord::new([1, 2, 3, 4], 3).unwrap();
        record.get_player_elo_change(&3).unwrap_err();
        assert!(record.elo_changes().is_none());
    }

    #[test]
    fn registered_matches_have_scores() {
        for tournament in Tournament::test_tournaments() {
            for game in tournament.games() {
                assert!(game.elo_changes().is_some());
                let players = *game.players();
                for p in players {
                    game.get_player_elo_change(&p).unwrap();
                }
            }
        }

    }

    #[test]
    fn game_record_winner_must_be_player() {
        GameRecord::new([0, 1, 2, 3], 0).unwrap();
        GameRecord::new([0, 1, 2, 3], 1).unwrap();
        GameRecord::new([0, 1, 2, 3], 2).unwrap();
        GameRecord::new([0, 1, 2, 3], 3).unwrap();
        GameRecord::new([0, 1, 2, 3], 4).unwrap_err();
    }

    #[test]
    fn matchup_record_winner_must_be_player() {
        let tournament = Tournament::generate_tournament(5, 0).unwrap();
        let ids = tournament.players().keys().copied().collect_vec();
        assert_eq!(5, ids.len());
        let matchup = tournament
            .create_match([ids[0], ids[1], ids[2], ids[3]])
            .unwrap();
        matchup.create_record(ids[0]).unwrap();
        matchup.create_record(ids[1]).unwrap();
        matchup.create_record(ids[2]).unwrap();
        matchup.create_record(ids[3]).unwrap();
        matchup.create_record(ids[4]).unwrap_err();
    }

    #[test]
    fn winner_gains_points() -> anyhow::Result<()> {
        for i in 0..4 {
            let mut tourn = Tournament::generate_tournament(4, 0)?;
            let ids = tourn.players().keys().copied().collect_vec();
            let mut match_ids = [0; 4];
            match_ids.copy_from_slice(&ids);
            let matchup = tourn.create_match(match_ids)?;
            let starting_elo = matchup.players[i].stats.elo();
            tourn.register_match(matchup, match_ids[i])?;
            let elo = tourn.stats[&match_ids[i]].elo();
            assert!(elo.total_cmp(&starting_elo).is_gt());
        }
        Ok(())
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn losers_lose_points() -> anyhow::Result<()> {
        for winner_i in 0..4 {
            let tourn = Tournament::generate_tournament(4, 0)?;
            let ids = tourn.players().keys().copied().collect_vec();
            let winner_id = ids[winner_i];
            let mut match_ids = [0; 4];
            match_ids.copy_from_slice(&ids);
            let matchup = tourn.create_match(match_ids)?;
            for loser_i in 0..4 {
                let mut tourn = tourn.clone();
                let matchup = matchup.clone();
                if winner_i == loser_i {
                    continue;
                }
                let loser_id = ids[loser_i];
                let starting_elo = matchup.players[loser_i].stats.elo();
                tourn.register_match(matchup, winner_id)?;
                let elo = tourn.stats[&loser_id].elo();
                assert!(elo.total_cmp(&starting_elo).is_le());
            }
        }

        Ok(())
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn winner_only_counted_once() -> anyhow::Result<()> {
        let mut tourn = Tournament::new();
        let id = tourn.register_player(String::from("sample"))?;
        let matchup = tourn.create_match([id, id, id, id])?;
        let starting_elo = matchup.players[0].stats.elo();
        tourn.register_match(matchup, id)?;
        let elo = tourn.stats[&id].elo();
        assert!(
            (starting_elo - elo).abs() <= 1e-10,
            "Elos do not match: {starting_elo} to {elo}"
        );

        Ok(())
    }
}

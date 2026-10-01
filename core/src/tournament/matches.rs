use core::array;

use crate::{
    error::TournamentError,
    game::{entry::GameEntry, match_player::MatchPlayer, matchup::Matchup, record::GameRecord},
    player::{PlayerId, stats::PlayerStats},
    tournament::Tournament,
};

impl Tournament {
    pub fn update_record(&self, record: GameRecord) -> Result<GameRecord, TournamentError> {
        if record.matchup().snapshot() == self.snapshot {
            return Ok(record);
        }
        let (matchup, winner) = record.decompose();
        self.create_match(matchup.ids())?.record(winner)
    }

    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.snapshot() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.ids())
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss, reason = "Generic Type to f64")]
    #[allow(clippy::indexing_slicing, reason = "Static T-constant range")]
    pub(crate) fn create_match_players<const T: usize>(&self, players: [PlayerId; T]) -> [MatchPlayer; T] {
        let config = self.config.game();

        let stats: [&PlayerStats; T] = array::from_fn(|i| self.get_player_or_default_stats(players[i]));

        let k: [f64; T] = {
            let calibration = config.calibration_games();
            array::from_fn(|i| {
                let games = stats[i].games();
                if calibration == 0 || games >= calibration {
                    return config.base_k();
                }

                let progress = f64::from(games) / f64::from(calibration);
                progress.mul_add(-(config.initial_k() - config.base_k()), config.initial_k())
            })
        };

        let max_elo = stats
            .iter()
            .map(|stats| stats.elo())
            .fold(f64::NEG_INFINITY, f64::max);

        let gamma: [f64; T] =
            array::from_fn(|i| 10.0_f64.powf((stats[i].elo() - max_elo) / config.logistic_scale()));

        let total_gamma: f64 = gamma.iter().sum();

        let expected: [f64; T] = if total_gamma > 0.0 {
            array::from_fn(|i| gamma[i] / total_gamma)
        } else {
            [1.0 / T as f64; T]
        };

        let elo_loss: [f64; T] = array::from_fn(|i| expected[i] * k[i]);

        let total_loss: f64 = elo_loss.iter().sum();

        array::from_fn(|i| {
            MatchPlayer::new(
                players[i],
                stats[i].clone(),
                expected[i],
                total_loss - elo_loss[i],
                elo_loss[i],
            )
        })
    }

    pub fn create_match(&self, ids: [PlayerId; 4]) -> Result<Matchup, TournamentError> {
        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        Ok(Matchup::new(self.create_match_players(ids), self.snapshot))
    }

    pub fn record_entry(&mut self, entry: GameEntry) -> Result<(), TournamentError> {
        let matchup = self.create_match(*entry.players())?;
        let record = matchup.record(entry.winner())?;
        self.insert_game_record(record);
        self.snapshot += 1;
        Ok(())
    }

    pub fn record_game(&mut self, record: GameRecord) -> Result<(), TournamentError> {
        self.insert_game_record(self.update_record(record)?);
        self.snapshot += 1;
        Ok(())
    }

    pub(super) fn insert_game_record(&mut self, record: GameRecord) {
        let mut winner_tracked = false;

        for player in record.matchup().players() {
            let stats = self
                .stats
                .entry(player.id())
                .or_insert_with(|| self.default_stats.clone());

            if !winner_tracked && player.id() == record.winner() {
                stats.add_win(*player.elo_win());
                winner_tracked = true;
            } else {
                stats.add_loss(*player.elo_loss());
            }
        }

        self.games.push(record);
    }

    #[must_use]
    pub const fn games(&self) -> &Vec<GameRecord> {
        &self.games
    }

    pub fn get_player_games(
        &self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = &GameRecord>, TournamentError> {
        if !self.is_id_registered(&id) {
            return Err(TournamentError::InvalidPlayerId(id));
        }

        Ok(self.games().iter().filter(move |game| game.has_player(id)))
    }

    pub fn delete_game(&mut self, gid: usize) -> Result<(), TournamentError> {
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
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn expected_adds_up_to_1() {
        #[allow(clippy::needless_pass_by_value, reason = "testing")]
        fn assert_sums_up_to_one<const T: usize>(players: [MatchPlayer; T]) {
            assert_relative_eq!(1.0, players.iter().map(|p| { p.expected() }).sum::<f64>());
        }
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();

        assert_sums_up_to_one(t.create_match_players([id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id, id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id, id, id, id]));
    }
}

use crate::{
    error::TournamentError,
    game::{entry::GameEntry, match_player::MatchPlayer, matchup::Matchup, record::GameRecord},
    player::{PlayerId, stats::PlayerStats},
    tournament::Tournament,
};

impl Tournament {
    pub fn update_record(&self, record: GameRecord) -> Result<GameRecord, TournamentError> {
        if record.matchup.snapshot() == self.snapshot {
            return Ok(record);
        }
        let GameRecord { matchup, winner } = record;
        self.create_match(matchup.ids().collect())?.record(winner)
    }

    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.snapshot() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.ids().collect())
    }

    pub fn create_match(&self, ids: Vec<PlayerId>) -> Result<Matchup, TournamentError> {
        if ids.len() < 2 {
            return Err(TournamentError::NotEnoughPlayers);
        }
        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        Ok(Matchup::new(self.create_match_players(ids), self.snapshot))
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss, reason = "usize to f64")]
    pub(crate) fn create_match_players<I>(&self, players: I) -> Vec<MatchPlayer>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        struct Player<'a> {
            id: PlayerId,
            stats: &'a PlayerStats,
            gamma: f64,
            k: f64,
        }

        let config = self.config.game();

        let mut max_elo = f64::MIN;

        let mut players = players
            .into_iter()
            .map(|id| {
                let stats = self.get_player_or_default_stats(id);
                if max_elo < stats.elo() {
                    max_elo = stats.elo();
                }

                let k = {
                    if config.calibration_games() == 0 || stats.games() >= config.calibration_games() {
                        config.base_k()
                    } else {
                        let progress = f64::from(stats.games()) / f64::from(config.calibration_games());
                        progress.mul_add(-(config.initial_k() - config.base_k()), config.initial_k())
                    }
                };

                Player {
                    id,
                    stats,
                    gamma: 0.0,
                    k,
                }
            })
            .collect::<Vec<_>>();

        let count = players.len();

        let mut total_gamma = 0.0;

        for player in &mut players {
            player.gamma = 10.0_f64.powf((player.stats.elo() - max_elo) / config.logistic_scale());
            total_gamma += player.gamma;
        }

        players
            .into_iter()
            .map(|player| {
                let expected = {
                    if total_gamma > 0.0 {
                        player.gamma / total_gamma
                    } else {
                        1.0 / (count as f64)
                    }
                };

                let elo_loss = player.k * expected;
                let elo_win = player.k - elo_loss;

                MatchPlayer::new(player.id, player.stats.clone(), expected, elo_win, elo_loss)
            })
            .collect()
    }

    pub fn record_entry(&mut self, entry: GameEntry) -> Result<(), TournamentError> {
        let GameEntry { players, winner } = entry;
        let matchup = self.create_match(players)?;
        let record = matchup.record(winner)?;
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

        for player in record.matchup.players() {
            let stats = self
                .stats
                .entry(player.id())
                .or_insert_with(|| self.default_stats.clone());

            if !winner_tracked && player.id() == record.winner {
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
        fn assert_sums_up_to_one(players: Vec<MatchPlayer>) {
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

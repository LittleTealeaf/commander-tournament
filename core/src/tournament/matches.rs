use crate::{
    config::game::GameConfig,
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
    pub(crate) fn create_match_players<const T: usize>(&self, players: [PlayerId; T]) -> [MatchPlayer; T] {
        let game_config = self.config.game();

        // 1. Stats + K
        let players = players.map(|player| {
            let stats = self.get_player_or_default_stats(player);
            let k = calculate_k(game_config, stats);
            (player, stats, k)
        });

        // 2. Max Elo
        let max_elo = players
            .iter()
            .map(|(_, stats, _)| stats.elo())
            .fold(f64::NEG_INFINITY, f64::max);

        // 3. Gamma
        let players = players.map(|(player, stats, k)| {
            let gamma = 10.0_f64.powf((stats.elo() - max_elo) / game_config.logistic_scale());
            (player, stats, k, gamma)
        });

        let total_gamma: f64 = players.iter().map(|(_, _, _, g)| *g).sum();

        // 4. Expected + Loss
        let players = players.map(|(player, stats, k, gamma)| {
            let expected = if total_gamma > 0.0 {
                gamma / total_gamma
            } else {
                1.0 / T as f64
            };
            let elo_lost = k * expected;
            (player, stats, expected, elo_lost)
        });

        let total_lost: f64 = players.iter().map(|(_, _, _, lost)| *lost).sum();

        // 5. Final MatchPlayer structs (moves `stats` directly without `.clone()`)
        players.map(|(player, stats, expected, lost)| {
            MatchPlayer::new(player, stats.clone(), expected, total_lost - lost, lost)
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

fn calculate_k(config: &GameConfig, stats: &PlayerStats) -> f64 {
    if config.calibration_games() == 0 || stats.games() >= config.calibration_games() {
        return config.base_k();
    }

    let progress = f64::from(stats.games()) / f64::from(config.calibration_games());
    progress.mul_add(-(config.initial_k() - config.base_k()), config.initial_k())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn expected_adds_up_to_1() {
        #[allow(clippy::needless_pass_by_value, reason = "testS")]
        fn assert_sums_up_to_one<const T: usize>(players: [MatchPlayer; T]) {
            assert_relative_eq!(1.0, players.iter().map(|p| { p.expected() }).sum::<f64>());
        }
        let t = Tournament::generate_tournament(1, 0).unwrap();
        let id = *t.players().keys().next().unwrap();

        assert_sums_up_to_one(t.create_match_players([id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id, id]));
        assert_sums_up_to_one(t.create_match_players([id, id, id, id, id]));
    }
}

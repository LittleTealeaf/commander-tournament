use crate::{
    error::TournamentError,
    game::{entry::GameEntry, match_player::MatchPlayer, matchup::Matchup, record::GameRecord},
    player::stats::PlayerStats,
    tournament::Tournament,
};

impl Tournament {
    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.snapshot() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.ids())
    }

    #[must_use]
    pub(crate) fn create_match_players<const T: usize>(
        &self,
        players: [u32; T],
    ) -> [MatchPlayer; T] {
        #[derive(Debug)]
        struct TempMatchPlayer<'a> {
            id: u32,
            stats: &'a PlayerStats,
            scaled_elo: f64,
            scaled_wr: f64,
        }

        #[allow(clippy::cast_precision_loss)]
        let base_chance = 1.0 / (T as f64);

        let config = self.game_config();

        let id_stats = players.map(|id| {
            let stats = self.get_player_or_default_stats(id);
            TempMatchPlayer {
                scaled_wr: stats
                    .wr()
                    .unwrap_or(base_chance)
                    .powf(config.game_wr_pow_scale),
                scaled_elo: stats.elo().powf(config.game_elo_pow_scale),
                stats,
                id,
            }
        });

        let sum_wr = id_stats.iter().map(|p| p.scaled_wr).sum::<f64>();
        let sum_elo = id_stats.iter().map(|p| p.scaled_elo).sum::<f64>();

        let wr_weight = if sum_wr > 0.0 {
            config.game_wr_weight
        } else {
            0.0
        };
        let weight_total = wr_weight + config.game_elo_weight;
        let coef_wr = if sum_wr > 0.0 {
            config.game_wr_weight / (weight_total * sum_wr)
        } else {
            0.0
        };
        let coef_elo = config.game_elo_weight / (weight_total * sum_elo);
        let base_loss = 1.0 - base_chance;

        id_stats.map(|player| {
            let game_points = config.game_points;
            let expected = coef_wr.mul_add(player.scaled_wr, coef_elo * player.scaled_elo);
            let elo_win = game_points * (1.0 - expected) / base_loss;
            let elo_loss = game_points * expected / base_loss;
            MatchPlayer::new(player.id, player.stats.clone(), expected, elo_win, elo_loss)
        })
    }

    pub fn create_match(&self, ids: [u32; 4]) -> Result<Matchup, TournamentError> {
        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        Ok(Matchup::new(self.create_match_players(ids), self.snapshot))
    }
    pub fn register_entry(&mut self, entry: GameEntry) -> Result<(), TournamentError> {
        let matchup = self.create_match(*entry.players())?;
        let record = matchup.record(entry.winner())?;
        self.insert_game_record(record);
        self.snapshot += 1;
        Ok(())
    }

    pub fn register_record(&mut self, record: GameRecord) -> Result<(), TournamentError> {
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
        id: u32,
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
        #[allow(clippy::needless_pass_by_value)]
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

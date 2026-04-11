use crate::{
    error::TournamentError,
    game::{
        entry::GameEntry, match_player::MatchPlayer, matchable::calculate_expected_values,
        matchup::Matchup, record::GameRecord,
    },
    player::PlayerId,
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
    pub(crate) fn create_match_players<const T: usize>(
        &self,
        players: [PlayerId; T],
    ) -> [MatchPlayer; T] {
        #[allow(clippy::cast_precision_loss)]
        let base_chance = 1.0 / (T as f64);

        let config = self.game_config();

        let players = players.map(|player| (player, self.get_player_or_default_stats(player)));
        let expected = calculate_expected_values(config, players);

        let base_loss = 1.0 - base_chance;

        expected.map(|((id, stats), expected)| {
            MatchPlayer::new(
                id,
                stats.clone(),
                expected,
                config.game_points * (1.0 - expected) / base_loss,
                config.game_points * expected / base_loss,
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

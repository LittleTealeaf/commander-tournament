pub mod entry;
pub mod match_player;
pub mod matchup;
pub mod record;

use crate::game::entry::GameEntry;
use crate::game::record::GameRecord;
use crate::{
    Tournament,
    error::{TournResult, TournamentError},
};

impl Tournament {
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

    use crate::Tournament;

    #[test]
    fn winner_gains_points() -> anyhow::Result<()> {
        for i in 0..4 {
            let mut tourn = Tournament::generate_tournament(4, 0)?;
            let ids = tourn.players().keys().copied().collect_vec();
            let mut match_ids = [0; 4];
            match_ids.copy_from_slice(&ids);
            let matchup = tourn.create_match(match_ids)?;
            let starting_elo = matchup.players()[i].stats().elo();
            tourn.register_record(matchup.record(match_ids[i])?)?;
            let elo = tourn.stats[&match_ids[i]].elo();
            assert!(
                elo.total_cmp(&starting_elo).is_gt(),
                "Elo {elo} should be greater than starting elo {starting_elo}"
            );
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
                let starting_elo = matchup.players()[loser_i].stats().elo();
                tourn.register_record(matchup.record(winner_id)?)?;
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
        let starting_elo = matchup.players()[0].stats().elo();
        tourn.register_record(matchup.record(id)?)?;
        let elo = tourn.stats[&id].elo();
        assert!(
            (starting_elo - elo).abs() <= 1e-10,
            "Elos do not match: {starting_elo} to {elo}"
        );

        Ok(())
    }
}

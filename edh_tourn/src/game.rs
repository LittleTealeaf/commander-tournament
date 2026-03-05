pub mod entry;
pub mod match_player;
pub mod matchup;
pub mod record;

use crate::game::entry::GameEntry;
use crate::game::match_player::MatchPlayer;
use crate::game::matchup::Matchup;
use crate::game::record::GameRecord;
use crate::stats::PlayerStats;
use crate::{
    Tournament,
    error::{TournResult, TournamentError},
};

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

        let players = ids.map(|id| {
            let stats = self.get_player_or_default_stats(id);
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

            MatchPlayer::new(player.id, player.stats.clone(), expected, elo_win, elo_loss)
        });

        Ok(Matchup::new(match_players, self.snapshot))
    }

    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.version() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.players().clone().map(|player| player.id()))
    }

    pub fn register_entry(&mut self, entry: GameEntry) -> Result<(), TournamentError> {
        self.register_record(
            self.create_match(*entry.players())?
                .record(entry.winner())?,
        )?;
        Ok(())
    }

    pub fn register_record(&mut self, record: GameRecord) -> Result<(), TournamentError> {
        self.inner_register_record(record)?;
        self.snapshot += 1;
        Ok(())
    }

    pub(super) fn inner_register_record(
        &mut self,
        record: GameRecord,
    ) -> Result<(), TournamentError> {
        let (matchup, winner) = record.into();
        let record = self.update_match(matchup)?.record(winner)?;

        let mut winner_tracked = false;

        for player in record.matchup().players() {
            let stats = self
                .stats
                .entry(player.id())
                .or_insert_with(|| self.default_stats.clone());

            stats.games += 1;

            if !winner_tracked && player.id() == record.winner() {
                stats.wins += 1;
                stats.elo += player.elo_win();
                winner_tracked = true;
            } else {
                stats.elo -= player.elo_loss();
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

    use crate::{Tournament, game::GameEntry};

    #[test]
    fn game_entry_record_winner_must_be_player() {
        GameEntry::new([0, 1, 2, 3], 0).unwrap();
        GameEntry::new([0, 1, 2, 3], 1).unwrap();
        GameEntry::new([0, 1, 2, 3], 2).unwrap();
        GameEntry::new([0, 1, 2, 3], 3).unwrap();
        GameEntry::new([0, 1, 2, 3], 4).unwrap_err();
    }

    #[test]
    fn matchup_record_winner_must_be_player() {
        let tournament = Tournament::generate_tournament(5, 0).unwrap();
        let ids = tournament.players().keys().copied().collect_vec();
        assert_eq!(5, ids.len());
        let matchup = tournament
            .create_match([ids[0], ids[1], ids[2], ids[3]])
            .unwrap();
        matchup.clone().record(ids[0]).unwrap();
        matchup.clone().record(ids[1]).unwrap();
        matchup.clone().record(ids[2]).unwrap();
        matchup.clone().record(ids[3]).unwrap();
        matchup.record(ids[4]).unwrap_err();
    }

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

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
    pub fn create_match_players<const T: usize>(&self, players: [u32; T]) -> [MatchPlayer; T] {
        struct TempMatchPlayer<'a> {
            id: u32,
            stats: &'a PlayerStats,
            scaled_elo: f64,
            scaled_wr: f64,
        }

        #[allow(clippy::cast_precision_loss)]
        let base_chance = 1.0 / (T as f64);

        // let base_chance = (0..T).map(|_| 1.0).sum::<f64>().powi(-1);
        let config = self.game_config();

        let id_stats = players.map(|id| {
            let stats = self.get_player_or_default_stats(id);
            TempMatchPlayer {
                scaled_wr: stats
                    .wr()
                    .unwrap_or(base_chance)
                    .powf(self.game_config().game_wr_pow_scale),
                scaled_elo: stats.elo().powf(self.game_config().game_elo_pow_scale),
                stats,
                id,
            }
        });

        let sum_wr: f64 = id_stats.iter().map(|p| p.scaled_wr).sum();
        let sum_elo: f64 = id_stats.iter().map(|p| p.scaled_elo).sum();

        let weight_total = config.game_wr_weight + config.game_elo_weight;
        let coef_wr = config.game_wr_weight / (weight_total * sum_wr);
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

// #[cfg(test)]
// mod tests {
//     #![allow(clippy::indexing_slicing)]
//
//     use itertools::Itertools;
//
//     use crate::Tournament;
//
//     #[test]
//     fn winner_gains_points() -> anyhow::Result<()> {
//         for i in 0..4 {
//             let mut tourn = Tournament::generate_tournament(4, 0)?;
//             let ids = tourn.players().keys().copied().collect_vec();
//             let mut match_ids = [0; 4];
//             match_ids.copy_from_slice(&ids);
//             let matchup = tourn.create_match(match_ids)?;
//             let starting_elo = matchup.players()[i].stats().elo();
//             tourn.register_record(matchup.record(match_ids[i])?)?;
//             let elo = tourn.stats[&match_ids[i]].elo();
//             assert!(
//                 elo.total_cmp(&starting_elo).is_gt(),
//                 "Elo {elo} should be greater than starting elo {starting_elo}"
//             );
//         }
//         Ok(())
//     }
//
//     #[test]
//     #[allow(clippy::needless_range_loop)]
//     fn losers_lose_points() -> anyhow::Result<()> {
//         for winner_i in 0..4 {
//             let tourn = Tournament::generate_tournament(4, 0)?;
//             let ids = tourn.players().keys().copied().collect_vec();
//             let winner_id = ids[winner_i];
//             let mut match_ids = [0; 4];
//             match_ids.copy_from_slice(&ids);
//             let matchup = tourn.create_match(match_ids)?;
//             for loser_i in 0..4 {
//                 let mut tourn = tourn.clone();
//                 let matchup = matchup.clone();
//                 if winner_i == loser_i {
//                     continue;
//                 }
//                 let loser_id = ids[loser_i];
//                 let starting_elo = matchup.players()[loser_i].stats().elo();
//                 tourn.register_record(matchup.record(winner_id)?)?;
//                 let elo = tourn.stats[&loser_id].elo();
//                 assert!(elo.total_cmp(&starting_elo).is_le());
//             }
//         }
//
//         Ok(())
//     }
//
//     #[test]
//     #[allow(clippy::needless_range_loop)]
//     fn winner_only_counted_once() -> anyhow::Result<()> {
//         let mut tourn = Tournament::new();
//         let id = tourn.register_player(String::from("sample"))?;
//         let matchup = tourn.create_match([id, id, id, id])?;
//         let starting_elo = matchup.players()[0].stats().elo();
//         tourn.register_record(matchup.record(id)?)?;
//         let elo = tourn.stats[&id].elo();
//         assert!(
//             (starting_elo - elo).abs() <= 1e-10,
//             "Elos do not match: {starting_elo} to {elo}"
//         );
//
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//
//     use super::*;
//
//     #[test]
//     fn create_match_invalid_ids() {
//         let tourn = Tournament::new();
//         tourn.create_match([1, 2, 3, 4]).unwrap_err();
//     }
//
//     #[test]
//     fn mirror_matchup_equal_expected() {
//         let mut tourn = Tournament::new();
//         let id = tourn.register_player("A".to_owned()).unwrap();
//         let mu = tourn.create_match([id, id, id, id]).unwrap();
//         for p in mu.players() {
//             assert_relative_eq!(0.25, *p.expected());
//         }
//     }
//
//     #[test]
//     fn record_winner_must_be_player() {
//         let tournament = Tournament::generate_tournament(5, 0).unwrap();
//         let mut ids = tournament.players().keys().copied();
//         let player_a = ids.next().unwrap();
//         let player_b = ids.next().unwrap();
//         let player_c = ids.next().unwrap();
//         let player_d = ids.next().unwrap();
//         let player_e = ids.next().unwrap();
//
//         let mu = tournament
//             .create_match([player_a, player_b, player_c, player_d])
//             .unwrap();
//         mu.clone().record(player_a).unwrap();
//         mu.clone().record(player_b).unwrap();
//         mu.clone().record(player_c).unwrap();
//         mu.clone().record(player_d).unwrap();
//         mu.clone().record(player_e).unwrap_err();
//         mu.record(u32::MAX).unwrap_err();
//     }
// }

#[cfg(test)]
extern crate approx;

pub mod analytics;
pub mod config;
pub mod error;
pub mod game;
pub mod player;
mod serialization;
pub mod tournament;
pub mod tsv;


// impl Tournament {
//     #[must_use]
//     pub fn new() -> Self {
//         Self::default()
//     }
//
//     #[must_use]
//     pub fn get_player_id(&self, name: &String) -> Option<u32> {
//         self.player_names.get(name).copied()
//     }
//
//     #[must_use]
//     pub fn is_id_registered(&self, id: &u32) -> bool {
//         self.players.contains_key(id)
//     }
//
//     pub fn unregister_player(&mut self, id: u32) -> Result<(), TournamentError> {
//         self.players
//             .remove(&id)
//             .ok_or(TournamentError::InvalidPlayerId(id))?;
//         self.games.retain(|game| !game.has_player(id));
//         self.reload()?;
//         Ok(())
//     }
//
//
//
//
//     #[must_use]
//     pub const fn players(&self) -> &HashMap<u32, PlayerInfo> {
//         &self.players
//     }
//
//     /// Merges with another tournament. If decks from either game have the same name, they are
//     /// merged. Games are added to the end of the base tournament.
//
//     /// Moves all of the tournament data, systematically, into a new Tournament object.
//     /// This is useful as a way around resetting player ids
//     pub fn into_fresh(&self) -> Result<Self, TournamentError> {
//         let mut tourn = Self {
//             config: self.config.clone(),
//             default_stats: self.default_stats.clone(),
//             snapshot: 0,
//             ..Self::new()
//         };
//         tourn.merge(self)?;
//         tourn.snapshot = 0;
//         Ok(tourn)
//     }
//
//     fn require_id_registered(&self, id: u32) -> Result<(), TournamentError> {
//         if !self.is_id_registered(&id) {
//             return Err(TournamentError::InvalidPlayerId(id));
//         }
//         Ok(())
//     }
// }
//
// impl FromIterator<Self> for Tournament {
//     fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
//         let mut base = Self::new();
//         for tourn in iter {
//             let _ = base.merge(&tourn);
//         }
//
//         base
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;
//
//     use crate::Tournament;
//
//     #[test]
//     fn new_tournament_snapshot_is_0() {
//         let tourn = Tournament::new();
//         assert_eq!(0, tourn.snapshot);
//     }
//
//     #[test]
//     fn collects_into_tournament() {
//         let tourn = Tournament::test_tournaments().collect::<Tournament>();
//         assert!(!tourn.games().is_empty());
//     }
//
//     #[test]
//     fn new_has_no_players() {
//         let new_tourn = Tournament::new();
//         assert_eq!(0, new_tourn.players.len());
//     }
//
//     #[test]
//     fn unregister_removes_players_games() {
//         let sample = Tournament::sample_game();
//         for id in sample.players().keys() {
//             let mut tourn = sample.clone();
//             tourn.unregister_player(*id).unwrap();
//             for game in tourn.games() {
//                 assert!(!game.has_player(*id));
//                 assert_ne!(game.winner(), *id);
//             }
//         }
//     }
//
//     #[test]
//     fn unregister_invalid_id_returns_err() {
//         let mut tourn = Tournament::new();
//         for i in 0..100 {
//             tourn.unregister_player(i).unwrap_err();
//         }
//     }
//
//     #[test]
//     fn into_fresh_works_simple() -> anyhow::Result<()> {
//         for game in Tournament::test_tournaments() {
//             game.into_fresh()?;
//         }
//         Ok(())
//     }
//
//     #[test]
//     fn into_fresh_same_players() -> anyhow::Result<()> {
//         let game = Tournament::generate_tournament(35, 20)?;
//         let new_game = game.into_fresh()?;
//         let new_game_players = new_game.players().values().collect::<Vec<_>>();
//         for player in game.players().values() {
//             assert!(new_game_players.contains(&player));
//         }
//         assert_eq!(game.players().len(), new_game_players.len());
//
//         Ok(())
//     }
//
//     #[test]
//     fn into_fresh_resets_snapshot() {
//         let mut game = Tournament::new();
//         game.snapshot = 5;
//         let new_game = game.into_fresh().unwrap();
//         assert_eq!(0, new_game.snapshot);
//     }
//
//     #[test]
//     fn into_fresh_resets_ids() -> anyhow::Result<()> {
//         const REMOVE_COUNT: usize = 40;
//         let mut game = Tournament::generate_tournament(100, 0)?;
//         let mut ids = game.players.keys().copied().sorted().take(40);
//         // Just a dummy test that the first one is 0
//         assert_eq!(0, ids.next().unwrap());
//         game.unregister_player(0)?;
//
//         for id in ids {
//             game.unregister_player(id)?;
//         }
//
//         assert_eq!(60, game.players.len());
//         assert_eq!(99, *game.players.keys().max().unwrap());
//
//         let new_game = game.into_fresh()?;
//
//         assert_eq!(60, new_game.players.len());
//         assert_eq!(59, *new_game.players.keys().max().unwrap());
//
//         Ok(())
//     }
//
//     #[test]
//     fn into_fresh_same_stats() -> anyhow::Result<()> {
//         for game in Tournament::test_tournaments() {
//             let new_game = game.into_fresh()?;
//             for (id, info) in game.players() {
//                 let stats = game.get_player_stats(*id);
//                 let new_id = new_game.get_player_id(info.name()).unwrap();
//                 let new_stats = new_game.get_player_stats(new_id);
//                 assert_eq!(stats.is_some(), new_stats.is_some());
//                 let (Some(stats), Some(new_stats)) = (stats, new_stats) else {
//                     continue;
//                 };
//
//                 assert!(
//                     (stats.elo() - new_stats.elo()).abs() <= 1e-9,
//                     "Elo Difference, from {} to {}",
//                     stats.elo(),
//                     new_stats.elo()
//                 );
//             }
//         }
//
//         Ok(())
//     }
//
//     #[test]
//     fn merge_tournaments_merge_players() {
//         let players = ["a", "b", "c", "d"];
//         let mut tournament_a = Tournament::new();
//         for p in &players {
//             tournament_a.register_player(p.to_string()).unwrap();
//         }
//         let mut tournament_b = Tournament::new();
//         for p in &players {
//             tournament_b.register_player(p.to_string()).unwrap();
//         }
//
//         tournament_a.merge(&tournament_b).unwrap();
//
//         assert_eq!(4, tournament_a.players.len());
//     }

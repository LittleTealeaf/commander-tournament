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

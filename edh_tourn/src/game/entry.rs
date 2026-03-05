use std::collections::HashMap;

use crate::{Tournament, error::TournamentError, game::record::GameRecord};

/// Stores only the player IDs and the winner ID. Primarily used for serialization or conversions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Copy, Eq)]
pub struct GameEntry {
    #[serde(rename = "p", alias = "players")]
    players: [u32; 4],
    #[serde(rename = "w", alias = "winner")]
    winner: u32,
}

impl GameEntry {
    pub fn new(players: [u32; 4], winner: u32) -> Result<Self, TournamentError> {
        if !players.contains(&winner) {
            return Err(TournamentError::WinnerNotInMatch(winner));
        }
        Ok(Self::new_unchecked(players, winner))
    }

    #[must_use]
    pub(crate) const fn new_unchecked(players: [u32; 4], winner: u32) -> Self {
        Self { players, winner }
    }

    #[must_use]
    pub const fn players(&self) -> &[u32; 4] {
        &self.players
    }

    #[must_use]
    pub const fn winner(&self) -> u32 {
        self.winner
    }

    pub fn map_ids(&self, ids: &HashMap<u32, u32>) -> Result<Self, TournamentError> {
        let [a, b, c, d] = self.players;
        let a = ids.get(&a).ok_or(TournamentError::InvalidPlayerId(a))?;
        let b = ids.get(&b).ok_or(TournamentError::InvalidPlayerId(b))?;
        let c = ids.get(&c).ok_or(TournamentError::InvalidPlayerId(c))?;
        let d = ids.get(&d).ok_or(TournamentError::InvalidPlayerId(d))?;
        let winner = ids
            .get(&self.winner)
            .ok_or(TournamentError::InvalidPlayerId(self.winner))?;

        Self::new([*a, *b, *c, *d], *winner)
    }
}

impl Tournament {
    pub fn create_entry_record(&self, entry: GameEntry) -> Result<GameRecord, TournamentError> {
        self.create_match(entry.players)?.record(entry.winner)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn winner_must_be_player() {
        GameEntry::new([0, 1, 2, 3], 0).unwrap();
        GameEntry::new([0, 1, 2, 3], 1).unwrap();
        GameEntry::new([0, 1, 2, 3], 2).unwrap();
        GameEntry::new([0, 1, 2, 3], 3).unwrap();
        GameEntry::new([0, 1, 2, 3], 4).unwrap_err();
    }

    #[test]
    fn maps_to_correct_ids() {
        let starting = [1, 2, 3, 4];
        let ending = [5, 6, 7, 8];
        let map = [(1, 5), (2, 6), (3, 7), (4, 8)]
            .into_iter()
            .collect::<HashMap<_, _>>();

        let entry = GameEntry::new(starting, 1).unwrap();
        let mapped_entry = entry.map_ids(&map).unwrap();
        assert_eq!(ending, mapped_entry.players);
        assert_eq!(5, mapped_entry.winner);
    }

    #[test]
    fn map_fails_invalid_id() {
        let entry = GameEntry::new([1, 2, 3, 4], 1).unwrap();
        entry.map_ids(&HashMap::new()).unwrap_err();
    }
}

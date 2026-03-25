use std::collections::HashMap;

use crate::{error::TournamentError, game::record::GameRecord};

/// Stores only the player IDs and the winner ID. Primarily used for serialization or conversions
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Copy, Eq, Hash,
)]
pub struct GameEntry {
    #[serde(rename = "p", alias = "players")]
    players: [u32; 4],
    #[serde(rename = "w", alias = "winner")]
    winner: u32,
}

impl GameEntry {
    pub const fn new(players: [u32; 4], winner: u32) -> Result<Self, TournamentError> {
        let [a, b, c, d] = players;
        if a != winner && b != winner && c != winner && d != winner {
            return Err(TournamentError::PlayerNotInMatch(winner));
        }

        Ok(Self::new_unchecked([a, b, c, d], winner))
    }

    #[must_use]
    const fn new_unchecked(players: [u32; 4], winner: u32) -> Self {
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

impl From<GameRecord> for GameEntry {
    fn from(value: GameRecord) -> Self {
        Self::new_unchecked(value.ids(), value.winner())
    }
}

impl From<&GameRecord> for GameEntry {
    fn from(value: &GameRecord) -> Self {
        Self::new_unchecked(value.ids(), value.winner())
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::tournament::Tournament;

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

    #[test]
    fn from_game_record() {
        let tournament = Tournament::generate_tournament(10, 10).unwrap();
        let ids = tournament
            .players()
            .keys()
            .copied()
            .take(4)
            .collect_array()
            .unwrap();
        let record = tournament
            .create_match(ids)
            .unwrap()
            .record(ids[0])
            .unwrap();
        let entry = GameEntry::from(record);
        assert_eq!(ids, entry.players);
        assert_eq!(ids[0], entry.winner);
    }

    #[test]
    fn from_game_record_ref() {
        let tournament = Tournament::generate_tournament(10, 10).unwrap();
        let ids = tournament
            .players()
            .keys()
            .copied()
            .take(4)
            .collect_array()
            .unwrap();
        let record = tournament
            .create_match(ids)
            .unwrap()
            .record(ids[0])
            .unwrap();
        let entry = GameEntry::from(&record);
        assert_eq!(ids, entry.players);
        assert_eq!(ids[0], entry.winner);
    }
}

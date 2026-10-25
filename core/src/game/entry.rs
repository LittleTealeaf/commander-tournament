use std::collections::HashMap;

use crate::{error::TournamentError, game::record::GameRecord, player::PlayerId};

/// Stores only the player IDs and the winner ID. Primarily used for serialization or conversions
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Default,
    PartialEq,
    Eq,
    Hash,
    getset::Getters,
    getset::CopyGetters,
)]
pub struct GameEntry {
    #[serde(rename = "p", alias = "players")]
    #[getset(get = "pub")]
    players: Vec<PlayerId>,
    #[serde(rename = "w", alias = "winner")]
    #[getset(get_copy = "pub")]
    winner: PlayerId,
}

impl GameEntry {
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn new(players: Vec<PlayerId>, winner: PlayerId) -> Result<Self, TournamentError> {
        if !players.contains(&winner) {
            return Err(TournamentError::PlayerNotInMatch(winner));
        }

        Ok(Self { players, winner })
    }

    pub fn map_ids(&self, ids: &HashMap<PlayerId, PlayerId>) -> Result<Self, TournamentError> {
        Self::new(
            self.players
                .iter()
                .map(|id| ids.get(id).copied().ok_or(TournamentError::InvalidPlayerId(*id)))
                .collect::<Result<Vec<PlayerId>, TournamentError>>()?,
            self.winner,
        )
    }
}

impl From<GameEntry> for (Vec<PlayerId>, PlayerId) {
    fn from(value: GameEntry) -> Self {
        (value.players, value.winner)
    }
}

impl From<GameRecord> for GameEntry {
    fn from(value: GameRecord) -> Self {
        Self {
            players: value.players().iter().map(|mp| mp.id()).collect(),
            winner: value.winner(),
        }
    }
}

impl From<&GameRecord> for GameEntry {
    fn from(value: &GameRecord) -> Self {
        Self {
            players: value.players().iter().map(|mp| mp.id()).collect(),
            winner: value.winner(),
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::tournament::Tournament;

    use super::*;

    #[test]
    fn winner_must_be_player() {
        GameEntry::new(
            vec![PlayerId(0), PlayerId(1), PlayerId(2), PlayerId(3)],
            PlayerId(0),
        )
        .unwrap();
        GameEntry::new(
            vec![PlayerId(0), PlayerId(1), PlayerId(2), PlayerId(3)],
            PlayerId(1),
        )
        .unwrap();
        GameEntry::new(
            vec![PlayerId(0), PlayerId(1), PlayerId(2), PlayerId(3)],
            PlayerId(2),
        )
        .unwrap();
        GameEntry::new(
            vec![PlayerId(0), PlayerId(1), PlayerId(2), PlayerId(3)],
            PlayerId(3),
        )
        .unwrap();
        GameEntry::new(
            vec![PlayerId(0), PlayerId(1), PlayerId(2), PlayerId(3)],
            PlayerId(4),
        )
        .unwrap_err();
    }

    #[test]
    fn maps_to_correct_ids() {
        let starting = [1, 2, 3, 4].map(PlayerId).to_vec();
        let ending = [5, 6, 7, 8].map(PlayerId).to_vec();
        let map = [(1, 5), (2, 6), (3, 7), (4, 8)]
            .into_iter()
            .map(|(a, b)| (PlayerId(a), PlayerId(b)))
            .collect::<HashMap<_, _>>();

        let entry = GameEntry::new(starting, PlayerId(1)).unwrap();
        let mapped_entry = entry.map_ids(&map).unwrap();
        assert_eq!(ending, mapped_entry.players);
        assert_eq!(PlayerId(5), mapped_entry.winner);
    }

    #[test]
    fn map_fails_invalid_id() {
        let entry = GameEntry::new([1, 2, 3, 4].map(PlayerId).to_vec(), PlayerId(1)).unwrap();
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
        let record = tournament.create_match(ids).unwrap().record(ids[0]).unwrap();
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
        let record = tournament.create_match(ids).unwrap().record(ids[0]).unwrap();
        let entry = GameEntry::from(&record);
        assert_eq!(ids, entry.players);
        assert_eq!(ids[0], entry.winner);
    }
}

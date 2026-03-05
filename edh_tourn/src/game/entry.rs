use std::collections::HashMap;

use crate::{error::TournamentError};

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
        Ok(Self { players, winner })
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

use crate::{
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Matchup {
    players: [MatchPlayer; 4],
    version: usize,
}

impl Matchup {
    #[must_use]
    pub(crate) const  fn new(players: [MatchPlayer; 4], version: usize) -> Self {
        Self { players, version }
    }

    #[must_use]
    pub const fn version(&self) -> usize {
        self.version
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        &self.players
    }

    #[must_use]
    pub fn get_ids(&self) -> [u32; 4] {
        self.players.clone().map(|player| player.id())
    }

    pub fn record(self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

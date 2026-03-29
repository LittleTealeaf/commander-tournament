use crate::{
    error::TournamentError,
    game::{POD_SIZE, match_player::MatchPlayer, record::GameRecord},
    player::PlayerId,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Matchup {
    players: [MatchPlayer; POD_SIZE],
    snapshot: usize,
}

impl Matchup {
    pub(crate) const fn new(players: [MatchPlayer; POD_SIZE], snapshot: usize) -> Self {
        Self { players, snapshot }
    }

    #[must_use]
    pub(crate) const fn snapshot(&self) -> usize {
        self.snapshot
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; POD_SIZE] {
        &self.players
    }

    #[must_use]
    pub fn has_player(&self, id: PlayerId) -> bool {
        self.get_player(id).is_some()
    }

    #[must_use]
    pub fn get_player(&self, id: PlayerId) -> Option<&MatchPlayer> {
        self.players.iter().find(|p| p.id() == id)
    }

    #[must_use]
    pub fn ids(&self) -> [PlayerId; 4] {
        self.players.clone().map(|p| p.id())
    }

    pub fn record(self, winner: PlayerId) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

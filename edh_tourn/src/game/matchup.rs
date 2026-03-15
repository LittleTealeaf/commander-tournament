use crate::{
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Matchup {
    players: [MatchPlayer; 4],
    snapshot: usize,
}

impl Matchup {
    #[must_use]
    pub(crate) const fn new(players: [MatchPlayer; 4], snapshot: usize) -> Self {
        Self { players, snapshot }
    }

    #[must_use]
    pub const fn snapshot(&self) -> usize {
        self.snapshot
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        &self.players
    }

    #[must_use]
    pub const fn has_player(&self, id: u32) -> bool {
        self.get_player(id).is_some()
    }

    #[must_use]
    pub const fn get_player(&self, id: u32) -> Option<&MatchPlayer> {
        let [a, b, c, d] = &self.players();
        if a.id() == id {
            Some(a)
        } else if b.id() == id {
            Some(b)
        } else if c.id() == id {
            Some(c)
        } else if d.id() == id {
            Some(d)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn ids(&self) -> [u32; 4] {
        let [player_a, player_b, player_c, player_d] = &self.players;
        [player_a.id(), player_b.id(), player_c.id(), player_d.id()]
    }

    pub const fn record(self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

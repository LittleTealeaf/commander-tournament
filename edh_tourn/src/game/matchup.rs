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

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    use super::*;

    fn create_matchup() -> Matchup {
        let t = Tournament::generate_tournament(10, 10).unwrap();
        let player = t.players().keys().next().copied().unwrap();
        t.matchmaker().create_match(player).unwrap()
    }

    #[test]
    fn test_has_player() {
        let mut t = Tournament::new();
        let id_a = t.register_debug_player().unwrap();
        let id_b = t.register_debug_player().unwrap();
        let id_c = t.register_debug_player().unwrap();
        let id_d = t.register_debug_player().unwrap();
        let id_not_included = t.register_debug_player().unwrap();
        let m = t.create_match([id_a, id_b, id_c, id_d]).unwrap();

        assert!(m.has_player(id_a));
        assert!(m.has_player(id_b));
        assert!(m.has_player(id_c));
        assert!(m.has_player(id_d));
        assert!(!m.has_player(id_not_included));
    }

    #[test]
    fn test_ids() {
        let mut t = Tournament::new();
        let id_a = t.register_debug_player().unwrap();
        let id_b = t.register_debug_player().unwrap();
        let id_c = t.register_debug_player().unwrap();
        let id_d = t.register_debug_player().unwrap();
        let m = t.create_match([id_a, id_b, id_c, id_d]).unwrap();
        let ids = m.ids();
        assert_eq!(ids, [id_a, id_b, id_c, id_d]);
    }
}

use crate::{
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
    player::PlayerId,
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, getset::CopyGetters, getset::Getters,
)]
pub struct Matchup {
    #[get = "pub"]
    players: Vec<MatchPlayer>,
    #[get_copy = "pub"]
    snapshot: usize,
}

impl Matchup {
    pub(crate) const fn new(players: Vec<MatchPlayer>, snapshot: usize) -> Self {
        Self { players, snapshot }
    }

    #[must_use]
    pub fn has_player(&self, id: PlayerId) -> bool {
        self.get_player(id).is_some()
    }

    #[must_use]
    pub fn get_player(&self, id: PlayerId) -> Option<&MatchPlayer> {
        self.players.iter().find(|p| p.id() == id)
    }

    pub fn ids(&self) -> impl Iterator<Item = PlayerId> {
        self.players.iter().map(MatchPlayer::id)
    }

    pub fn record(self, winner: PlayerId) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn test_has_player() {
        let mut t = Tournament::new();
        let id_a = t.register_debug_player().unwrap();
        let id_b = t.register_debug_player().unwrap();
        let id_c = t.register_debug_player().unwrap();
        let id_d = t.register_debug_player().unwrap();
        let id_not_included = t.register_debug_player().unwrap();
        let m = t.create_match(vec![id_a, id_b, id_c, id_d]).unwrap();

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
        let m = t.create_match(vec![id_a, id_b, id_c, id_d]).unwrap();
        let ids: Vec<_> = m.ids().collect();
        assert_eq!(ids, vec![id_a, id_b, id_c, id_d]);
    }

    #[test]
    fn test_variable_player_counts() {
        let mut t = Tournament::new();
        let id_a = t.register_debug_player().unwrap();
        let id_b = t.register_debug_player().unwrap();
        let id_c = t.register_debug_player().unwrap();

        let m2 = t.create_match(vec![id_a, id_b]).unwrap();
        assert_eq!(m2.players().len(), 2);
        assert!(m2.has_player(id_a));
        assert!(m2.has_player(id_b));
        assert!(!m2.has_player(id_c));

        let m3 = t.create_match(vec![id_a, id_b, id_c]).unwrap();
        assert_eq!(m3.players().len(), 3);
        assert!(m3.has_player(id_a));
        assert!(m3.has_player(id_b));
        assert!(m3.has_player(id_c));
    }
}

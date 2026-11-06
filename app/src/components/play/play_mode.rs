use commander_tournament_core::{
    game::{POD_SIZE, matchup::Matchup, next_mode::NextPlayerMode},
    player::PlayerId,
    tournament::Tournament,
};

#[derive(Debug, Clone)]
pub enum PlayMode {
    Player(PlayerId),
    Next(NextPlayerMode),
    Custom(Vec<Option<PlayerId>>),
}

impl PlayMode {
    #[must_use]
    pub fn next() -> Self {
        Self::Next(NextPlayerMode::default())
    }

    #[must_use]
    pub fn custom() -> Self {
        Self::Custom(vec![None; POD_SIZE])
    }
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(id) => tournament.matchmaker().create_match(*id).ok(),
            Self::Custom(players) => {
                let ids: Vec<PlayerId> = players.iter().copied().collect::<Option<_>>()?;
                tournament.create_match(ids).ok()
            }
            Self::Next(mode) => tournament.next_game(*mode).ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_default_has_pod_size_nones() {
        let mode = PlayMode::custom();
        if let PlayMode::Custom(players) = mode {
            assert_eq!(players.len(), POD_SIZE);
            assert!(players.iter().all(core::option::Option::is_none));
        } else {
            panic!("Expected PlayMode::Custom");
        }
    }

    #[test]
    fn custom_with_none_returns_none() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();

        let mode_all_none = PlayMode::custom();
        assert!(mode_all_none.create_matchup(&t).is_none());

        let mode_partial_none = PlayMode::Custom(vec![Some(p1), Some(p2), None]);
        assert!(mode_partial_none.create_matchup(&t).is_none());
    }

    #[test]
    fn custom_with_less_than_two_players_returns_none() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();

        let mode_zero = PlayMode::Custom(vec![]);
        assert!(mode_zero.create_matchup(&t).is_none());

        let mode_one = PlayMode::Custom(vec![Some(p1)]);
        assert!(mode_one.create_matchup(&t).is_none());
    }

    #[test]
    fn custom_with_two_three_four_players() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();
        let p3 = t.register_player("P3").unwrap();
        let p4 = t.register_player("P4").unwrap();

        // 2 players
        let m2 = PlayMode::Custom(vec![Some(p1), Some(p2)]).create_matchup(&t).unwrap();
        assert_eq!(m2.players().len(), 2);
        assert!(m2.has_player(p1));
        assert!(m2.has_player(p2));

        // 3 players
        let m3 = PlayMode::Custom(vec![Some(p1), Some(p2), Some(p3)]).create_matchup(&t).unwrap();
        assert_eq!(m3.players().len(), 3);
        assert!(m3.has_player(p1));
        assert!(m3.has_player(p2));
        assert!(m3.has_player(p3));

        // 4 players
        let m4 = PlayMode::Custom(vec![Some(p1), Some(p2), Some(p3), Some(p4)]).create_matchup(&t).unwrap();
        assert_eq!(m4.players().len(), 4);
        assert!(m4.has_player(p1));
        assert!(m4.has_player(p2));
        assert!(m4.has_player(p3));
        assert!(m4.has_player(p4));
    }

    #[test]
    fn custom_with_unregistered_player_returns_none() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();
        t.unregister_player(p1).unwrap();
        let mode = PlayMode::Custom(vec![Some(p1), Some(p2)]);
        assert!(mode.create_matchup(&t).is_none());
    }
}

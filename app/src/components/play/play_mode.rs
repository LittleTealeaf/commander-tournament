use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

use crate::components::play::PlayNextMode;

#[derive(Debug, Clone)]
pub enum PlayMode {
    Player(PlayerId),
    Next(PlayNextMode),
    Custom([Option<PlayerId>; POD_SIZE]),
}

impl PlayMode {
    #[must_use]
    pub fn next() -> Self {
        Self::Next(PlayNextMode::default())
    }

    #[must_use]
    pub const fn custom() -> Self {
        Self::Custom([None; POD_SIZE])
    }
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(id) => tournament.matchmaker().create_match(*id).ok(),
            Self::Custom(players) => {
                let [a, b, c, d] = *players;
                tournament.create_match([a?, b?, c?, d?]).ok()
            }
            Self::Next(mode) => {
                let id = mode.get_player(tournament)?.id();
                tournament.matchmaker().create_match(id).ok()
            }
        }
    }
}

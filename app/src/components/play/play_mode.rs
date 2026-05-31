use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

use crate::components::play::PlayNextMode;

#[derive(Debug, Clone)]
pub enum PlayMode {
    Player(Option<PlayerId>),
    Next(PlayNextMode),
    Custom([Option<PlayerId>; POD_SIZE]),
}

impl PlayMode {
    #[must_use]
    pub const fn get_type(&self) -> PlayModeType {
        match self {
            Self::Player(..) => PlayModeType::Player,
            Self::Custom(..) => PlayModeType::Custom,
            Self::Next(..) => PlayModeType::Next,
        }
    }

    #[must_use]
    pub const fn player(id: PlayerId) -> Self {
        Self::Player(Some(id))
    }

    #[must_use]
    pub fn next() -> Self {
        Self::Next(PlayNextMode::default())
    }

    #[must_use]
    pub const fn custom() -> Self {
        Self::Custom([None; POD_SIZE])
    }
}

#[derive(Debug, Clone, Default, derive_more::Display, PartialEq, Eq)]
pub enum PlayModeType {
    #[default]
    Next,
    Player,
    Custom,
}

impl PlayModeType {
    pub const VALUES: [Self; 3] = [Self::Next, Self::Player, Self::Custom];
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(player_id) => player_id.and_then(|id| tournament.matchmaker().create_match(id).ok()),
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

impl From<PlayModeType> for PlayMode {
    fn from(value: PlayModeType) -> Self {
        match value {
            PlayModeType::Next => Self::Next(PlayNextMode::default()),
            PlayModeType::Player => Self::Player(None),
            PlayModeType::Custom => Self::Custom([const { None }; POD_SIZE]),
        }
    }
}

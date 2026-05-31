use edh_tourn::{game::POD_SIZE, player::PlayerId};

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
            Self::Player { .. } => PlayModeType::Player,
            Self::Custom { .. } => PlayModeType::Custom,
            Self::Next { .. } => PlayModeType::Next,
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

impl From<PlayModeType> for PlayMode {
    fn from(value: PlayModeType) -> Self {
        match value {
            PlayModeType::Next => Self::Next(PlayNextMode::default()),
            PlayModeType::Player => Self::Player(None),
            PlayModeType::Custom => Self::Custom([const { None }; POD_SIZE]),
        }
    }
}

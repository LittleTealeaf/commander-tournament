use edh_tourn::{game::POD_SIZE, player::PlayerId};

use crate::components::play::PlayNextMode;

#[derive(Debug, Clone)]
pub enum PlayMode {
    Player(Option<PlayerId>),
    Custom { players: [Option<PlayerId>; POD_SIZE] },
    Next { mode: PlayNextMode },
}

impl PlayMode {
    #[must_use]
    pub const fn get_type(&self) -> PlayModeType {
        match self {
            Self::Player(_) => PlayModeType::Player,
            Self::Custom { .. } => PlayModeType::Custom,
            Self::Next { .. } => PlayModeType::Next,
        }
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
            PlayModeType::Next => Self::Next {
                mode: PlayNextMode::default(),
            },
            PlayModeType::Player => Self::Player(None),
            PlayModeType::Custom => Self::Custom {
                players: [const { None }; POD_SIZE],
            },
        }
    }
}

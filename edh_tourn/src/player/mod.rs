use core::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::player::{info::PlayerInfo, stats::PlayerStats};

pub mod color;
pub mod info;
pub mod stats;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd, Default,
)]
#[serde(transparent)]
pub struct PlayerId(pub(crate) u32);

impl Display for PlayerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RegisteredPlayer<'a> {
    id: PlayerId,
    info: &'a PlayerInfo,
    stats: &'a PlayerStats,
}

impl<'a> RegisteredPlayer<'a> {
    #[must_use]
    pub(crate) const fn new(id: PlayerId, info: &'a PlayerInfo, stats: &'a PlayerStats) -> Self {
        Self { id, info, stats }
    }
}

impl RegisteredPlayer<'_> {
    #[must_use]
    pub const fn id(&self) -> PlayerId {
        self.id
    }

    #[must_use]
    pub const fn info(&self) -> &PlayerInfo {
        self.info
    }

    #[must_use]
    pub const fn stats(&self) -> &PlayerStats {
        self.stats
    }
}

impl Display for RegisteredPlayer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.info.name())
    }
}

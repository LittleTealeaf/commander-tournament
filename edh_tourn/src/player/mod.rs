use core::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    game::matchable::Matchable,
    player::{info::PlayerInfo, stats::PlayerStats},
};

pub mod color;
pub mod info;
pub mod stats;
pub mod filter;

/**
 * Identifies a unique player. The specific implementation may vary.
 */
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    Default,
    derive_more::Display,
)]
#[serde(transparent)]
pub struct PlayerId(pub(crate) u32);

/**
 * Represents a reference to a registered player.
 */
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
    /// Returns the direct [`PlayerId`] identifier of the registered player.
    #[must_use]
    pub const fn id(&self) -> PlayerId {
        self.id
    }

    /// Returns a reference to the [`PlayerInfo`].
    #[must_use]
    pub const fn info(&self) -> &PlayerInfo {
        self.info
    }

    /**
     * Returns a reference to the current [`PlayerStats`].
     * If the player has no modified stats, this will be
     * a reference to the default stats.
     */
    #[must_use]
    pub const fn stats(&self) -> &PlayerStats {
        self.stats
    }
}

impl Matchable for RegisteredPlayer<'_> {
    fn wr(&self) -> Option<f64> {
        self.stats().wr()
    }

    fn elo(&self) -> f64 {
        self.stats().elo()
    }
}

impl Display for RegisteredPlayer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.info.display_name())
    }
}

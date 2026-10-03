use core::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::player::{info::PlayerInfo, stats::PlayerStats};

pub mod color;
pub mod info;
pub mod stats;

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
#[derive(Clone, Copy, PartialEq, Debug, getset::CopyGetters, derive_more::Constructor)]
pub struct RegisteredPlayer<'a> {
    #[getset(get_copy = "pub")]
    id: PlayerId,
    #[getset(get_copy = "pub")]
    info: &'a PlayerInfo,
    #[getset(get_copy = "pub")]
    stats: &'a PlayerStats,
}

impl Display for RegisteredPlayer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.info.display_name())
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    use super::*;

    #[test]
    fn display() {
        let info = PlayerInfo::new("test".to_owned());
        let mut t = Tournament::new();
        let id = t.register_player(info).unwrap();
        let rg_pl = t.get_registered_player(id).unwrap();
        assert_eq!("test", format!("{rg_pl}"));
    }

    #[test]
    fn display_precon() {
        let mut info = PlayerInfo::new("test".to_owned());
        info.set_is_precon(true);
        let expected = info.display_name();
        let mut t = Tournament::new();
        let id = t.register_player(info).unwrap();
        let rg_pl = t.get_registered_player(id).unwrap();
        assert_eq!(expected, format!("{rg_pl}"));
    }
}

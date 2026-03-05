use core::fmt::Display;

use crate::{
    Tournament,
    error::TournamentError,
    player::{info::PlayerInfo, stats::PlayerStats},
};

pub mod info;
pub mod color;
pub mod stats;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RegisteredPlayer<'a> {
    id: u32,
    info: &'a PlayerInfo,
    stats: &'a PlayerStats,
}

impl RegisteredPlayer<'_> {
    #[must_use]
    pub const fn id(&self) -> u32 {
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

impl Tournament {
    pub fn get_registered_player(&self, id: u32) -> Result<RegisteredPlayer<'_>, TournamentError> {
        let info = self
            .get_player_info(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?;
        let stats = self.get_player_or_default_stats(id);
        Ok(RegisteredPlayer { id, info, stats })
    }

    pub fn get_registered_players(&self) -> impl Iterator<Item = RegisteredPlayer<'_>> {
        self.players.iter().map(|(id, info)| RegisteredPlayer {
            id: *id,
            info,
            stats: self.get_player_or_default_stats(*id),
        })
    }
}

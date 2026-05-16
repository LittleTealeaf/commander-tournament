//! Gives the ability to filter players based on some criteria

use crate::player::{
    PlayerId, RegisteredPlayer,
    color::{ColorIdentity, MtgColor},
};

/// Each option includes a bool indicating whether to include (`true`) or exclude (`false`)
#[derive(Debug, Clone)]
pub enum PlayerFilter {
    Players(Vec<PlayerId>, bool),
    Identity(ColorIdentity, bool),
    Identities(Vec<ColorIdentity>, bool),
    Precons(bool),
}

impl PlayerFilter {
    pub fn colors<I>(colors: I, include: bool) -> Self
    where
        I: IntoIterator<Item = MtgColor>,
    {
        Self::Identity(colors.into_iter().collect(), include)
    }

    pub fn apply<'a, I>(&self, iter: I) -> impl Iterator<Item = RegisteredPlayer<'a>>
    where
        I: IntoIterator<Item = RegisteredPlayer<'a>>,
    {
        iter.into_iter().filter(|player| match self {
            Self::Players(player_ids, inc) => *inc == player_ids.contains(&player.id()),
            Self::Identity(color_identity, inc) => {
                *inc == player.info().color_identity().contains(color_identity)
            }
            Self::Identities(items, inc) => *inc == items.contains(&player.info().color_identity()),
            Self::Precons(inc) => *inc == player.info().is_precon(),
        })
    }
}

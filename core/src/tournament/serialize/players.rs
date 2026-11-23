use std::collections::HashMap;

use backwards_compat::backwards_compat;
use itertools::Itertools;

use crate::player::{PlayerId, info::PlayerInfo};

#[derive(Debug)]
#[backwards_compat(
    tag="v",
    content = "p",
    versions(
        1: Self
    )
)]
pub(super) struct SerializedPlayerInfo(Vec<(PlayerId, PlayerInfo)>);

impl FromIterator<(PlayerId, PlayerInfo)> for SerializedPlayerInfo {
    fn from_iter<T: IntoIterator<Item = (PlayerId, PlayerInfo)>>(iter: T) -> Self {
        Self(iter.into_iter().sorted_by_key(|(id, _)| *id).collect())
    }
}

impl SerializedPlayerInfo {
    pub(super) fn value(self) -> HashMap<PlayerId, PlayerInfo> {
        HashMap::from_iter(self.0)
    }
}

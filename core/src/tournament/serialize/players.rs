use std::collections::HashMap;

use backwards_compat::backwards_compat;
use itertools::Itertools;

use crate::player::{PlayerId, info::PlayerInfo};

#[derive(Debug)]
#[backwards_compat(
    tag="v",
    versions(
        1: Self
    )
)]
pub struct SerializedPlayerInfo {
    #[serde(rename = "p")]
    players: Vec<(PlayerId, PlayerInfo)>,
}

impl FromIterator<(PlayerId, PlayerInfo)> for SerializedPlayerInfo {
    fn from_iter<T: IntoIterator<Item = (PlayerId, PlayerInfo)>>(iter: T) -> Self {
        Self {
            players: iter.into_iter().sorted_by_key(|(id, _)| *id).collect(),
        }
    }
}

impl SerializedPlayerInfo {
    pub fn value(self) -> HashMap<PlayerId, PlayerInfo> {
        HashMap::from_iter(self.players)
    }
}

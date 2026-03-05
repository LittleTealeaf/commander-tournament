use std::collections::HashMap;

use crate::player::color::MtgColor;



#[derive(Clone, serde::Deserialize)]
struct CompatTournament {
    #[serde(default)]
    players: HashMap<u32, CompatPlayerInfo>,
    #[serde(default)]
    games: Vec<CompatGame>
}

#[derive(Clone, serde::Deserialize)]
struct CompatConfig {

}

#[derive(Clone, serde::Deserialize)]
struct CompatPlayerInfo {
    name: String,
    #[serde(default)]
    colors: Vec<MtgColor>
}

#[derive(Clone, serde::Deserialize)]
struct CompatGame {
    p: [u32; 4],
    w: u32
}



pub mod game;

pub mod v1;
pub mod v2;

use crate::{
    config::TournamentConfig,
    serialization::config::{v1::V1TournamentConfig, v2::V2TournamentConfig},
};

use backwards_compat::backwards_compat;

backwards_compat! {
    #[tag="v", version=2]
    compat TournamentConfig {
        1: V1TournamentConfig,
        2: V2TournamentConfig
    }
}

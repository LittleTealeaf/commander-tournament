pub mod game;
pub mod matchmaker;

pub mod v1;
pub mod v2;
pub mod v3;

use crate::{
    config::TournamentConfig, serialization::config::{v1::V1TournamentConfig, v2::V2TournamentConfig, v3::V3TournamentConfig},
};

use backwards_compat::backwards_compat;

backwards_compat! {
    #[tag="v", version=3]
    compat TournamentConfig {
        1: V1TournamentConfig
        2: V2TournamentConfig
        3: V3TournamentConfig
    }
}

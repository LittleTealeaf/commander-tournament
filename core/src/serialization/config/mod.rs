pub mod v1;

use crate::{config::TournamentConfig, serialization::config::v1::V1TournamentConfig};

use backwards_compat::backwards_compat;

backwards_compat! {
    #[tag="v", version=1]
    compat TournamentConfig {
        1: V1TournamentConfig
    }
}

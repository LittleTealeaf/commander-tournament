pub mod v1;

use backwards_compat::backwards_compat;

use crate::{config::game::GameConfig, serialization::config::game::v1::V1GameConfig};

backwards_compat! {
    #[tag="v", version=1]
    compat GameConfig {
        1: V1GameConfig
    }
}

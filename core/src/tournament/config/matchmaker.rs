use backwards_compat::backwards_compat;

use crate::game::POD_SIZE;

#[derive(
    Debug,
    Clone,
    PartialEq,
    getset::CopyGetters,
    getset::Setters,
    getset::WithSetters,
    getset::MutGetters,
    derive_more::Constructor,
)]
#[getset(set = "pub", set_with = "pub", get_copy = "pub", get_mut = "pub")]
#[backwards_compat(tag = "v", version = 1)]
pub struct MatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 50.0,
            min_pool_size: (POD_SIZE - 1) * 3,
        }
    }
}

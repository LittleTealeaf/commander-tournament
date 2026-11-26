use backwards_compat::backwards_compat;
use serde_inline_default::serde_inline_default;

#[derive(Debug, serde::Deserialize)]
struct UnversionedMatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for UnversionedMatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 120.0,
            min_pool_size: 5,
        }
    }
}

impl From<UnversionedMatchmakerConfig> for MatchmakerConfigV1 {
    fn from(value: UnversionedMatchmakerConfig) -> Self {
        Self {
            elo_range: value.elo_range,
            min_pool_size: value.min_pool_size,
            max_unranked: 1,
            max_precon: 2,
            strict_calibrate: true,
        }
    }
}

impl From<UnversionedMatchmakerConfig> for MatchmakerConfig {
    fn from(value: UnversionedMatchmakerConfig) -> Self {
        MatchmakerConfigV1::from(value).into()
    }
}

#[serde_inline_default]
#[derive(serde::Deserialize)]
struct MatchmakerConfigV1 {
    elo_range: f64,
    min_pool_size: usize,
    #[serde_inline_default(1)]
    max_unranked: usize,
    #[serde_inline_default(2)]
    max_precon: usize,
    #[serde_inline_default(true)]
    #[serde(alias = "calibrate_with_elo")]
    strict_calibrate: bool,
}

impl From<MatchmakerConfigV1> for MatchmakerConfig {
    fn from(value: MatchmakerConfigV1) -> Self {
        Self {
            elo_range: value.elo_range,
            min_pool_size: value.min_pool_size,
            repeat_penalty: 15f64,
            strict_calibrate: value.strict_calibrate,
            max_precon: value.max_precon,
            max_unranked: value.max_unranked,
        }
    }
}

#[serde_inline_default]
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
#[backwards_compat(
    versions(
        _: MatchmakerConfigV1 => 2,
        _: UnversionedMatchmakerConfig => 2,
        2: Self
    )
)]
pub struct MatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
    repeat_penalty: f64,
    /// All calibration is done with absolute nearest neighbors
    strict_calibrate: bool,
    max_precon: usize,
    /// Only effective when `strict_calibrate` is false
    max_unranked: usize,
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 75f64,
            min_pool_size: 5,
            repeat_penalty: 15f64,
            strict_calibrate: true,
            max_precon: 2,
            max_unranked: 1,
        }
    }
}

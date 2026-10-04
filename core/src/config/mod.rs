pub(crate) mod compat;
pub mod game;
pub mod matchmaker;

use backwards_compat::backwards_compat;

use self::compat::{V1TournamentConfig, V2TournamentConfig};
use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    getset::Getters,
    getset::Setters,
    getset::WithSetters,
    derive_more::Constructor,
)]
#[getset(set = "pub", get = "pub", set_with = "pub")]
#[backwards_compat(
    tag = "v",
    version = 3,
    versions(
        1: V1TournamentConfig => 3,
        2: V2TournamentConfig => 3,
    )
)]
pub struct TournamentConfig {
    game: GameConfig,
    matchmaker: MatchmakerConfig,
}

use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig};
use backwards_compat::backwards_compat;

#[derive(Debug, Clone, PartialEq, Default, derive_more::Constructor)]
#[backwards_compat(
    tag = "v",
    versions(
        _: Self,
        3 | "1" | "2" | "3": Self
    )
)]
pub struct TournamentConfig {
    pub game: GameConfig,
    pub matchmaker: MatchmakerConfig,
}

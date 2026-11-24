pub mod decks;
pub mod history;
pub mod overview;
pub mod play;
pub mod settings;

use nerd_font_symbols::md;

#[derive(
    Debug, Clone, Copy, Default, strum::VariantArray, derive_more::Display, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ScreenKind {
    #[default]
    Overview,
    Play,
    Decks,
    #[display("Match History")]
    MatchHistory,
    Settings,
}

impl ScreenKind {
    #[must_use]
    pub const fn icon(self) -> &'static str {
        match self {
            Self::Overview => md::MD_VIEW_GRID_OUTLINE,
            Self::Play => md::MD_PLAY_OUTLINE,
            Self::Decks => md::MD_CARDS_OUTLINE,
            Self::MatchHistory => md::MD_HISTORY,
            Self::Settings => md::MD_COG_OUTLINE,
        }
    }
}

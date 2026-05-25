use edh_tourn::{game::matchup::Matchup, player::PlayerId};
pub use next_mode::*;
pub use play_mode::*;
pub use update::*;

use crate::traits::Component;

mod next_mode;
mod play_mode;
mod update;
mod view;

#[derive(Debug, Clone)]
pub struct PlayComponent {
    mode: PlayMode,
    allow_mode_changes: bool,
    preview: Option<MatchPreview>,
}

impl Default for PlayMode {
    fn default() -> Self {
        Self::Next {
            mode: PlayNextMode::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchPreview {
    matchup: Matchup,
    winner: Option<PlayerId>,
}

impl Component for PlayComponent {
    type Message = PlayComponentMsg;
    type OutMessage = PlayComponentOut;
}

impl PlayComponent {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: PlayMode::Next {
                mode: PlayNextMode::LongestBreak,
            },
            allow_mode_changes: true,
            preview: None,
        }
    }
}
impl Default for PlayComponent {
    fn default() -> Self {
        Self::new()
    }
}

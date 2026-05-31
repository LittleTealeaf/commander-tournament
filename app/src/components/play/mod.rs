use edh_tourn::{game::matchup::Matchup, player::PlayerId, tournament::Tournament};
pub use next_mode::*;
pub use play_mode::*;
pub use update::*;

use crate::traits::Component;

mod next_mode;
mod play_mode;
mod update;
mod view;

#[derive(Debug, Clone, Default)]
pub struct PlayComponent {
    mode: PlayMode,
    preview: Option<MatchPreview>,
}

impl Default for PlayMode {
    fn default() -> Self {
        Self::next()
    }
}

impl PlayComponent {
    #[must_use]
    pub const fn mode(&self) -> &PlayMode {
        &self.mode
    }

    pub fn refresh(&mut self, tournament: &Tournament) {
        self.preview = self.mode.create_matchup(tournament).map(|matchup| MatchPreview {
            matchup,
            winner: None,
        });
    }

    pub fn set_mode(&mut self, mode: PlayMode, tournament: &Tournament) {
        self.mode = mode;
        self.refresh(tournament);
    }

    pub fn set_next_mode(&mut self, next_mode: PlayNextMode, tournament: &Tournament) {
        let PlayMode::Next(mode) = &mut self.mode else {
            return;
        };
        *mode = next_mode;
        self.refresh(tournament);
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
    pub fn new(mode: PlayMode, tournament: &Tournament) -> Self {
        let mut component = Self { mode, preview: None };
        component.refresh(tournament);
        component
    }
}

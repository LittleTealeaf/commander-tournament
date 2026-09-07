use edh_tourn::{config::matchmaker::MatchmakerConfig, tournament::Tournament};
use iced::widget::{button, column, row, text};
use iced_aw::number_input;
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::views::ViewScreen;

#[derive(Debug, Clone)]
pub struct MatchmakerConfigView {
    config: MatchmakerConfig,
}

impl MatchmakerConfigView {
    #[must_use]
    pub const fn new(config: MatchmakerConfig) -> Self {
        Self { config }
    }
}

#[derive(Clone, Debug)]
pub enum MatchmakerConfigMsg {
    Close,
    Save,
    SetDefault,
    Reset,
    SetEloRange(f64),
    SetMinPoolSize(usize),
}

#[derive(Debug, Clone)]
pub enum MatchmakerConfigOut {
    Close,
    SaveAndClose(MatchmakerConfig),
}

impl Model for MatchmakerConfigView {
    type Message = MatchmakerConfigMsg;
    type OutMessage = MatchmakerConfigOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            MatchmakerConfigMsg::Close => {
                return Signal::out(MatchmakerConfigOut::Close).ok();
            }
            MatchmakerConfigMsg::Save => {
                return Signal::out(MatchmakerConfigOut::SaveAndClose(self.config.clone())).ok();
            }
            MatchmakerConfigMsg::SetMinPoolSize(size) => {
                self.config.set_min_pool_size(size);
            }
            MatchmakerConfigMsg::SetEloRange(range) => {
                self.config.set_elo_range(range);
            }
            MatchmakerConfigMsg::SetDefault => self.config = MatchmakerConfig::default(),
            MatchmakerConfigMsg::Reset => self.config = context.matchmaker_config().clone(),
        }
        Signal::done()
    }
}

impl ViewScreen for MatchmakerConfigView {
    const CLOSE_MESSAGE: Self::Message = MatchmakerConfigMsg::Close;
    fn title<'a>(&'a self, _context: Self::Context<'a>) -> String {
        "Play Settings".to_owned()
    }

    fn primary_actions<'a>(
        &'a self,
        _context: Self::Context<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(MatchmakerConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        _context: Self::Context<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(MatchmakerConfigMsg::Reset),
            button(MD_RESTORE).on_press(MatchmakerConfigMsg::SetDefault),
        ]
    }
}

impl Component for MatchmakerConfigView {
    fn render<'a>(&'a self, _context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        column![
            row![
                text("Minimum Pool Size"),
                number_input(
                    &self.config.min_pool_size(),
                    0..1000,
                    MatchmakerConfigMsg::SetMinPoolSize,
                )
                .ignore_buttons(true),
            ]
            .spacing(10),
            row![
                text("Pool Elo Range"),
                number_input(
                    &self.config.elo_range(),
                    0.0..1000.0,
                    MatchmakerConfigMsg::SetEloRange,
                )
                .ignore_buttons(true)
                .step(5.0),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .into()
    }
}

use commander_tournament_core::tournament::{GameConfig, Tournament};

use iced::widget::{button, column, row, text};
use iced_aw::number_input;
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::views::ViewScreen;

#[derive(Debug, Clone)]
pub struct GameConfigView {
    config: GameConfig,
}

impl GameConfigView {
    #[must_use]
    pub const fn new(config: GameConfig) -> Self {
        Self { config }
    }

    #[must_use]
    pub const fn config(&self) -> &GameConfig {
        &self.config
    }
}

#[derive(Clone, Debug)]
pub enum GameConfigMsg {
    Close,
    Save,
    SetDefault,
    Reset,
    SetInitialElo(f64),
    SetLogisticScale(f64),
    SetInitialK(f64),
    SetBaseK(f64),
    SetCalibrationGames(u32),
}

#[derive(Debug, Clone)]
pub enum GameConfigOut {
    Close,
    SaveAndClose(GameConfig),
}

impl Model for GameConfigView {
    type Message = GameConfigMsg;
    type OutMessage = GameConfigOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            GameConfigMsg::Close => Signal::out(GameConfigOut::Close).ok(),
            GameConfigMsg::Save => Signal::out(GameConfigOut::SaveAndClose(self.config.clone())).ok(),
            GameConfigMsg::SetDefault => {
                self.config = GameConfig::default();
                Signal::done()
            }
            GameConfigMsg::Reset => {
                self.config = context.game_config().clone();
                Signal::done()
            }
            GameConfigMsg::SetInitialElo(val) => {
                self.config.set_initial_elo(val);
                Signal::done()
            }
            GameConfigMsg::SetLogisticScale(val) => {
                self.config.set_logistic_scale(val);
                Signal::done()
            }
            GameConfigMsg::SetInitialK(val) => {
                self.config.set_initial_k(val);
                Signal::done()
            }
            GameConfigMsg::SetBaseK(val) => {
                self.config.set_base_k(val);
                Signal::done()
            }
            GameConfigMsg::SetCalibrationGames(val) => {
                self.config.set_calibration_games(val);
                Signal::done()
            }
        }
    }
}

impl ViewScreen for GameConfigView {
    const CLOSE_MESSAGE: Self::Message = GameConfigMsg::Close;

    fn title<'a>(&'a self, _context: Self::Context<'a>) -> String {
        "Game Settings".to_owned()
    }

    // Adding the save and reset buttons to match MatchmakerConfig
    fn primary_actions<'a>(
        &'a self,
        _context: Self::Context<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(GameConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        _context: Self::Context<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(GameConfigMsg::Reset),
            button(MD_RESTORE).on_press(GameConfigMsg::SetDefault),
        ]
    }
}

impl Component for GameConfigView {
    fn render<'a>(&'a self, _context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        column![
            row![
                text("Starting Elo"),
                number_input(
                    &self.config.initial_elo(),
                    0.0..10000.0,
                    GameConfigMsg::SetInitialElo,
                )
                .step(10.0)
                .ignore_buttons(true),
            ]
            .spacing(10),
            row![
                text("Logistic Scale"),
                number_input(
                    &self.config.logistic_scale(),
                    1.0..10000.0,
                    GameConfigMsg::SetLogisticScale,
                )
                .step(10.0)
                .ignore_buttons(true),
            ]
            .spacing(10),
            row![
                text("Initial K"),
                number_input(&self.config.initial_k(), 0.0..1000.0, GameConfigMsg::SetInitialK,)
                    .step(1.0)
                    .ignore_buttons(true),
            ]
            .spacing(10),
            row![
                text("Base K"),
                number_input(&self.config.base_k(), 0.0..1000.0, GameConfigMsg::SetBaseK,)
                    .step(1.0)
                    .ignore_buttons(true),
            ]
            .spacing(10),
            row![
                text("Calibration Games"),
                number_input(
                    &self.config.calibration_games(),
                    0..1000,
                    GameConfigMsg::SetCalibrationGames,
                )
                .ignore_buttons(true),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .into()
    }
}

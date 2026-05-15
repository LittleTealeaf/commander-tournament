use edh_tourn::{config::game::GameConfig, tournament::Tournament};

use iced::widget::{button, column, row, rule, text};
use iced_aw::number_input;
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
    views::ViewScreen,
};

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
    SetStartingElo(f64),
    SetGamePoints(f64),
    SetEloPowScale(f64),
    SetWrPowScale(f64),
    SetEloWeight(f64),
    SetWrWeight(f64),
}

#[derive(Debug, Clone)]
pub enum GameConfigOut {
    Close,
    SaveAndClose(GameConfig),
}

impl Component for GameConfigView {
    type Message = GameConfigMsg;
    type OutMessage = GameConfigOut;
}

impl ComponentUpdate for GameConfigView {
    type UpdateContext<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            GameConfigMsg::Close => Effect::out(GameConfigOut::Close).ok(),
            GameConfigMsg::Save => Effect::out(GameConfigOut::SaveAndClose(self.config.clone())).ok(),
            GameConfigMsg::SetDefault => {
                self.config = GameConfig::new();
                Effect::done()
            }
            GameConfigMsg::Reset => {
                self.config = context.game_config().clone();
                Effect::done()
            }
            GameConfigMsg::SetStartingElo(val) => {
                self.config.starting_elo = val;
                Effect::done()
            }
            GameConfigMsg::SetGamePoints(val) => {
                self.config.game_points = val;
                Effect::done()
            }
            GameConfigMsg::SetEloPowScale(val) => {
                self.config.game_elo_pow_scale = val;
                Effect::done()
            }
            GameConfigMsg::SetWrPowScale(val) => {
                self.config.game_wr_pow_scale = val;
                Effect::done()
            }
            GameConfigMsg::SetEloWeight(val) => {
                self.config.game_elo_weight = val;
                Effect::done()
            }
            GameConfigMsg::SetWrWeight(val) => {
                self.config.game_wr_weight = val;
                Effect::done()
            }
        }
    }
}

impl ViewScreen for GameConfigView {
    const CLOSE_MESSAGE: Self::Message = GameConfigMsg::Close;

    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Game Settings".to_owned()
    }

    // Adding the save and reset buttons to match MatchmakerConfig
    fn primary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(GameConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(GameConfigMsg::Reset),
            button(MD_RESTORE).on_press(GameConfigMsg::SetDefault),
        ]
    }
}

impl ComponentView for GameConfigView {
    type ViewContext<'a> = ();

    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let general_section = column![
            text("General").size(20),
            row![
                text("Starting Elo"),
                number_input(
                    &self.config.starting_elo,
                    0.0..10000.0,
                    GameConfigMsg::SetStartingElo,
                )
                .step(10.0)
                .ignore_buttons(true)
            ]
            .spacing(10),
            row![
                text("Game Points"),
                number_input(
                    &self.config.game_points,
                    0.0..1000.0,
                    GameConfigMsg::SetGamePoints,
                )
                .step(1.0)
                .ignore_buttons(true)
            ]
            .spacing(10),
        ]
        .spacing(15);

        let elo_section = column![
            text("Elo Ratings").size(20),
            row![
                text("Power Scale"),
                number_input(
                    &self.config.game_elo_pow_scale,
                    0.0..100.0,
                    GameConfigMsg::SetEloPowScale,
                )
                .step(0.1)
                .ignore_buttons(true)
            ]
            .spacing(10),
            row![
                text("Weight"),
                number_input(
                    &self.config.game_elo_weight,
                    0.0..100.0,
                    GameConfigMsg::SetEloWeight,
                )
                .step(1.0)
                .ignore_buttons(true)
            ]
            .spacing(10),
        ]
        .spacing(15);

        let wr_section = column![
            text("Win Rate (WR)").size(20),
            row![
                text("Power Scale"),
                number_input(
                    &self.config.game_wr_pow_scale,
                    0.0..100.0,
                    GameConfigMsg::SetWrPowScale,
                )
                .step(0.1)
                .ignore_buttons(true)
            ]
            .spacing(10),
            row![
                text("Weight"),
                number_input(
                    &self.config.game_wr_weight,
                    0.0..100.0,
                    GameConfigMsg::SetWrWeight,
                )
                .step(1.0)
                .ignore_buttons(true)
            ]
            .spacing(10),
        ]
        .spacing(15);

        // Combine everything: General on top, then a divider, then Elo and WR side-by-side
        column![
            general_section,
            rule::horizontal(10),
            row![elo_section, wr_section].spacing(40)
        ]
        .spacing(20)
        .into()
    }
}

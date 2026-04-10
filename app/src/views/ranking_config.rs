use edh_tourn::{config::ranking::RankingConfig, tournament::Tournament};
use iced::widget::{button, column, row, text};
use iced_aw::number_input;
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
    views::ViewScreen,
};

#[derive(Debug, Clone)]
pub struct RankingConfigView {
    config: RankingConfig,
}

impl RankingConfigView {
    #[must_use]
    pub const fn new(config: RankingConfig) -> Self {
        Self { config }
    }
}

#[derive(Clone, Debug)]
pub enum RankingConfigMsg {
    Close,
    Save,
    SetDefault,
    Reset,
    SetLeastPlayed(usize),
    SetNemesis(usize),
    SetLostWith(usize),
    SetEloNeighbor(usize),
    SetWrNeighbor(usize),
    SetExpectedNeighbor(usize),
}

#[derive(Debug, Clone)]
pub enum RankingConfigOut {
    Close,
    SaveAndClose(RankingConfig),
}

impl Component for RankingConfigView {
    type Message = RankingConfigMsg;
    type OutMessage = RankingConfigOut;
}

impl ComponentUpdate for RankingConfigView {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            RankingConfigMsg::Close => return Effect::out(RankingConfigOut::Close).ok(),
            RankingConfigMsg::Save => {
                return Effect::out(RankingConfigOut::SaveAndClose(self.config.clone())).ok();
            }
            RankingConfigMsg::SetLeastPlayed(value) => {
                self.config.least_played = value;
            }
            RankingConfigMsg::SetNemesis(value) => self.config.nemesis = value,
            RankingConfigMsg::SetLostWith(value) => self.config.lost_with = value,
            RankingConfigMsg::SetEloNeighbor(value) => self.config.elo_neighbor = value,
            RankingConfigMsg::SetWrNeighbor(value) => self.config.wr_neighbor = value,
            RankingConfigMsg::SetExpectedNeighbor(value) => self.config.expected_neighbor = value,
            RankingConfigMsg::SetDefault => self.config = RankingConfig::default(),
            RankingConfigMsg::Reset => self.config = context.ranking_config().clone(),
        }
        Effect::done()
    }
}

impl ViewScreen for RankingConfigView {
    const CLOSE_MESSAGE: Self::Message = RankingConfigMsg::Close;
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Play Settings".to_owned()
    }

    fn primary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(RankingConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(RankingConfigMsg::Reset),
            button(MD_RESTORE).on_press(RankingConfigMsg::SetDefault),
        ]
    }
}

impl ComponentView for RankingConfigView {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let options = [
            (
                "Least Played",
                number_input(
                    &self.config.least_played,
                    0..1000,
                    RankingConfigMsg::SetLeastPlayed,
                )
                .ignore_buttons(true),
            ),
            (
                "Nemesis",
                number_input(&self.config.nemesis, 0..1000, RankingConfigMsg::SetNemesis)
                    .ignore_buttons(true),
            ),
            (
                "Lost With",
                number_input(
                    &self.config.lost_with,
                    0..1000,
                    RankingConfigMsg::SetLostWith,
                )
                .ignore_buttons(true),
            ),
            (
                "Elo Neighbor",
                number_input(
                    &self.config.elo_neighbor,
                    0..1000,
                    RankingConfigMsg::SetEloNeighbor,
                )
                .ignore_buttons(true),
            ),
            (
                "WR Neighbor",
                number_input(
                    &self.config.wr_neighbor,
                    0..1000,
                    RankingConfigMsg::SetWrNeighbor,
                )
                .ignore_buttons(true),
            ),
            (
                "Expected Neighbor",
                number_input(
                    &self.config.expected_neighbor,
                    0..1000,
                    RankingConfigMsg::SetExpectedNeighbor,
                )
                .ignore_buttons(true),
            ),
        ];

        column(options.map(|(label_text, input)| row![text(label_text), input].spacing(10).into()))
            .spacing(10)
            .into()
    }
}

use edh_tourn::{config::ranking::RankingConfig, tournament::Tournament};
use iced::widget::{button, column, row, text};
use iced_aw::number_input;
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView, ViewScreen},
};

#[derive(Debug, Clone)]
pub struct PlayConfig {
    config: RankingConfig,
}

impl PlayConfig {
    #[must_use]
    pub const fn new(config: RankingConfig) -> Self {
        Self { config }
    }
}

#[derive(Clone, Debug)]
pub enum PlayConfigMsg {
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
pub enum PlayConfigOut {
    Close,
    SaveAndClose(RankingConfig),
}

impl Component for PlayConfig {
    type Message = PlayConfigMsg;
    type OutMessage = PlayConfigOut;
}

impl ComponentUpdate for PlayConfig {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayConfigMsg::Close => return Effect::out(PlayConfigOut::Close).ok(),
            PlayConfigMsg::Save => {
                return Effect::out(PlayConfigOut::SaveAndClose(self.config.clone())).ok();
            }
            PlayConfigMsg::SetLeastPlayed(value) => {
                self.config.least_played = value;
            }
            PlayConfigMsg::SetNemesis(value) => self.config.nemesis = value,
            PlayConfigMsg::SetLostWith(value) => self.config.lost_with = value,
            PlayConfigMsg::SetEloNeighbor(value) => self.config.elo_neighbor = value,
            PlayConfigMsg::SetWrNeighbor(value) => self.config.wr_neighbor = value,
            PlayConfigMsg::SetExpectedNeighbor(value) => self.config.expected_neighbor = value,
            PlayConfigMsg::SetDefault => self.config = RankingConfig::default(),
            PlayConfigMsg::Reset => self.config = context.ranking_config().clone(),
        }
        Effect::done()
    }
}

impl ViewScreen for PlayConfig {
    const CLOSE_MESSAGE: Self::Message = PlayConfigMsg::Close;
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Play Settings".to_owned()
    }

    fn primary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(PlayConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(PlayConfigMsg::Reset),
            button(MD_RESTORE).on_press(PlayConfigMsg::SetDefault),
        ]
    }
}

impl ComponentView for PlayConfig {
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
                    PlayConfigMsg::SetLeastPlayed,
                )
                .ignore_buttons(true),
            ),
            (
                "Nemesis",
                number_input(&self.config.nemesis, 0..1000, PlayConfigMsg::SetNemesis)
                    .ignore_buttons(true),
            ),
            (
                "Lost With",
                number_input(&self.config.lost_with, 0..1000, PlayConfigMsg::SetLostWith)
                    .ignore_buttons(true),
            ),
            (
                "Elo Neighbor",
                number_input(
                    &self.config.elo_neighbor,
                    0..1000,
                    PlayConfigMsg::SetNeighbor,
                )
                .ignore_buttons(true),
            ),
            (
                "WR Neighbor",
                number_input(&self.config.wr_neighbor, 0..1000, PlayConfigMsg::SetNemesis)
                    .ignore_buttons(true),
            ),
            (
                "Expected Neighbor",
                number_input(
                    &self.config.expected_neighbor,
                    0..1000,
                    PlayConfigMsg::SetExpected,
                )
                .ignore_buttons(true),
            ),
        ];

        column(options.map(|(label_text, input)| row![text(label_text), input].spacing(10).into()))
            .spacing(10)
            .into()
    }
}

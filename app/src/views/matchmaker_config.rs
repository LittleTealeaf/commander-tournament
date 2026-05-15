use edh_tourn::{config::matchmaker::MatchmakerConfig, tournament::Tournament};
use iced::widget::{button, column, row, text};
use iced_aw::number_input;
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_RESTORE, MD_UNDO};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
    views::ViewScreen,
};

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
    SetLeastPlayed(usize),
    SetNemesis(usize),
    SetLostWith(usize),
    SetEloNeighbor(usize),
    SetWrNeighbor(usize),
    SetExpectedNeighbor(usize),
}

#[derive(Debug, Clone)]
pub enum MatchmakerConfigOut {
    Close,
    SaveAndClose(MatchmakerConfig),
}

impl Component for MatchmakerConfigView {
    type Message = MatchmakerConfigMsg;
    type OutMessage = MatchmakerConfigOut;
}

impl ComponentUpdate for MatchmakerConfigView {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            MatchmakerConfigMsg::Close => {
                return Effect::out(MatchmakerConfigOut::Close).ok();
            }
            MatchmakerConfigMsg::Save => {
                return Effect::out(MatchmakerConfigOut::SaveAndClose(self.config.clone())).ok();
            }
            MatchmakerConfigMsg::SetLeastPlayed(value) => {
                self.config.player_least_played = value;
            }
            MatchmakerConfigMsg::SetNemesis(value) => self.config.player_nemesis = value,
            MatchmakerConfigMsg::SetLostWith(value) => self.config.player_lost_with = value,
            MatchmakerConfigMsg::SetEloNeighbor(value) => self.config.elo_neighbor = value,
            MatchmakerConfigMsg::SetWrNeighbor(value) => self.config.wr_neighbor = value,
            MatchmakerConfigMsg::SetExpectedNeighbor(value) => {
                self.config.expected_neighbor = value;
            }
            MatchmakerConfigMsg::SetDefault => self.config = MatchmakerConfig::default(),
            MatchmakerConfigMsg::Reset => self.config = context.matchmaker_config().clone(),
        }
        Effect::done()
    }
}

impl ViewScreen for MatchmakerConfigView {
    const CLOSE_MESSAGE: Self::Message = MatchmakerConfigMsg::Close;
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Play Settings".to_owned()
    }

    fn primary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE).on_press(MatchmakerConfigMsg::Save)]
    }

    fn secondary_actions<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        [
            button(MD_UNDO).on_press(MatchmakerConfigMsg::Reset),
            button(MD_RESTORE).on_press(MatchmakerConfigMsg::SetDefault),
        ]
    }
}

impl ComponentView for MatchmakerConfigView {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let options = [
            (
                "Least Played",
                number_input(
                    &self.config.player_least_played,
                    0..1000,
                    MatchmakerConfigMsg::SetLeastPlayed,
                )
                .ignore_buttons(true),
            ),
            (
                "Nemesis",
                number_input(
                    &self.config.player_nemesis,
                    0..1000,
                    MatchmakerConfigMsg::SetNemesis,
                )
                .ignore_buttons(true),
            ),
            (
                "Lost With",
                number_input(
                    &self.config.player_lost_with,
                    0..1000,
                    MatchmakerConfigMsg::SetLostWith,
                )
                .ignore_buttons(true),
            ),
            (
                "Elo Neighbor",
                number_input(
                    &self.config.elo_neighbor,
                    0..1000,
                    MatchmakerConfigMsg::SetEloNeighbor,
                )
                .ignore_buttons(true),
            ),
            (
                "WR Neighbor",
                number_input(
                    &self.config.wr_neighbor,
                    0..1000,
                    MatchmakerConfigMsg::SetWrNeighbor,
                )
                .ignore_buttons(true),
            ),
            (
                "Expected Neighbor",
                number_input(
                    &self.config.expected_neighbor,
                    0..1000,
                    MatchmakerConfigMsg::SetExpectedNeighbor,
                )
                .ignore_buttons(true),
            ),
        ];

        column(options.map(|(label_text, input)| row![text(label_text), input].spacing(10).into()))
            .spacing(10)
            .into()
    }
}

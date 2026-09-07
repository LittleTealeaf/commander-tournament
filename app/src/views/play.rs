use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::widget::button;
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md::MD_COGS;

use crate::{
    components::play::{PlayComponent, PlayComponentMsg, PlayComponentOut, PlayMode},
    views::ViewScreen,
};

#[derive(Debug, Clone)]
pub struct PlayView(PlayComponent);

impl PlayView {
    #[must_use]
    pub fn new(mode: PlayMode, tournament: &Tournament) -> Self {
        Self(PlayComponent::new(mode, tournament))
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum PlayViewMsg {
    Play(PlayComponentMsg),
    OpenMatchmakerConfig,
    Close,
}

#[derive(Debug, Clone)]
pub enum PlayViewOut {
    Close,
    OpenMatchmakerConfig,
    OpenPlayer(PlayerId),
    OpenLink(String),
    RecordGame(Box<GameRecord>),
}

impl Model for PlayView {
    type Message = PlayViewMsg;
    type OutMessage = PlayViewOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            PlayViewMsg::Play(msg) => self.0.map_update(msg, context, |out| {
                Signal::out(match out {
                    PlayComponentOut::OpenPlayer(player_id) => PlayViewOut::OpenPlayer(player_id),
                    PlayComponentOut::OpenLink(link) => PlayViewOut::OpenLink(link),
                    PlayComponentOut::RecordGame(game_record) => PlayViewOut::RecordGame(game_record),
                })
                .ok()
            }),
            PlayViewMsg::Close => Signal::out(PlayViewOut::Close).ok(),
            PlayViewMsg::OpenMatchmakerConfig => Signal::out(PlayViewOut::OpenMatchmakerConfig).ok(),
        }
    }
}

impl Component for PlayView {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        self.0.render_into(context)
    }
}

impl ViewScreen for PlayView {
    const CLOSE_MESSAGE: Self::Message = PlayViewMsg::Close;
    const ON_RESUME: Option<Self::Message> = Some(PlayViewMsg::Play(PlayComponentMsg::Refresh));

    fn secondary_actions<'a>(
        &'a self,
        _: Self::Context<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_COGS).on_press(PlayViewMsg::OpenMatchmakerConfig)]
    }

    fn title<'a>(&'a self, context: Self::Context<'a>) -> String {
        match &self.0.mode() {
            PlayMode::Player(id) => format!(
                "Play: {}",
                context
                    .get_player_name(id)
                    .map_or("Unknown Player", |id| id.as_ref())
            ),
            PlayMode::Next { .. } => "Play Tournament".to_owned(),
            PlayMode::Custom { .. } => "Custom Games".to_owned(),
        }
    }
}

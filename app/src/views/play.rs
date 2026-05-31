use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::widget::button;
use nerd_font_symbols::md::MD_COGS;

use crate::{
    components::play::{PlayComponent, PlayComponentMsg, PlayComponentOut, PlayMode},
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
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

impl Component for PlayView {
    type Message = PlayViewMsg;
    type OutMessage = PlayViewOut;
}

impl ComponentUpdate for PlayView {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayViewMsg::Play(msg) => self.0.map_update(msg, context, |out| {
                Effect::out(match out {
                    PlayComponentOut::OpenPlayer(player_id) => PlayViewOut::OpenPlayer(player_id),
                    PlayComponentOut::OpenLink(link) => PlayViewOut::OpenLink(link),
                    PlayComponentOut::RecordGame(game_record) => PlayViewOut::RecordGame(game_record),
                })
                .ok()
            }),
            PlayViewMsg::Close => Effect::out(PlayViewOut::Close).ok(),
            PlayViewMsg::OpenMatchmakerConfig => Effect::out(PlayViewOut::OpenMatchmakerConfig).ok(),
        }
    }
}

impl ComponentView for PlayView {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        self.0.view_into(context)
    }
}

impl ViewScreen for PlayView {
    const CLOSE_MESSAGE: Self::Message = PlayViewMsg::Close;
    const ON_RESUME: Option<Self::Message> = Some(PlayViewMsg::Play(PlayComponentMsg::Refresh));

    fn secondary_actions<'a>(
        &'a self,
        _: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_COGS).on_press(PlayViewMsg::OpenMatchmakerConfig)]
    }

    fn title<'a>(&'a self, context: Self::ViewContext<'a>) -> String {
        match &self.0.mode() {
            PlayMode::Player(id) => format!(
                "Play: {}",
                id.and_then(|id| context.get_player_name(&id))
                    .map_or("Unknown Player", |id| id.as_ref())
            ),
            PlayMode::Next { .. } => "Play Tournament".to_owned(),
            PlayMode::Custom { .. } => "Custom Games".to_owned(),
        }
    }
}

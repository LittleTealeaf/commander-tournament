use edh_tourn::tournament::Tournament;
use iced::Element;

use crate::modals::Modal;
use crate::views::ViewScreen;
use crate::views::game_config::{GameConfigMsg, GameConfigOut, GameConfigView};
use crate::views::{
    matchmaker_config::{MatchmakerConfigMsg, MatchmakerConfigOut, MatchmakerConfigView},
    play::PlayMode,
};
use crate::{
    App,
    app::message::Message,
    core::tournament::TournamentAction,
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
    views::{
        play::{PlayMsg, PlayOut, PlayView},
        player::{PlayerDetailsMsg, PlayerDetailsOut, PlayerView},
    },
};

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Play(PlayView),
    PlayConfig(MatchmakerConfigView),
    GameConfig(GameConfigView),
    PlayerDetails(PlayerView),
}

impl View {
    pub fn on_resume(&self) -> Option<ViewMsg> {
        match self {
            Self::Play(_) => PlayView::ON_RESUME.map(Into::into),
            Self::PlayConfig(_) => MatchmakerConfigView::ON_RESUME.map(Into::into),
            Self::PlayerDetails(_) => PlayerView::ON_RESUME.map(Into::into),
            Self::GameConfig(_) => GameConfigView::ON_RESUME.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ViewMsg {
    PlayerDetails(PlayerDetailsMsg),
    PlayConfig(MatchmakerConfigMsg),
    GameConfig(GameConfigMsg),
    Play(PlayMsg),
}

impl Component for View {
    type OutMessage = Message;
    type Message = ViewMsg;
}

#[derive(derive_more::Constructor, Debug)]
pub struct ViewUpdateContext<'a> {
    tourn: &'a Tournament,
}

impl ComponentUpdate for View {
    type UpdateContext<'a> = ViewUpdateContext<'a>;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        const CLOSE_VIEW: Effect<ViewMsg, Message> = Effect::Out(Message::CloseView);

        match (self, message) {
            (Self::PlayerDetails(state), ViewMsg::PlayerDetails(msg)) => {
                state.map_update(msg, (), |out| match out {
                    PlayerDetailsOut::OpenPlayerDetails(player_id) => {
                        Effect::Out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayerDetailsOut::DeletePlayer(player_id) => {
                        Effect::out(TournamentAction::DeletePlayer(player_id))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                    PlayerDetailsOut::OpenLink(link) => Effect::Out(Message::OpenLink(link)).ok(),
                    PlayerDetailsOut::SaveAndClose(player_id, player_info) => Effect::out(match player_id {
                        Some(id) => TournamentAction::SetPlayerInfo(id, player_info),
                        None => TournamentAction::Register(player_info),
                    })
                    .chain(CLOSE_VIEW)
                    .ok(),
                    PlayerDetailsOut::OpenPlayerMatches(player_id) => {
                        Effect::out(Message::OpenPlay(PlayMode::player(player_id))).ok()
                    }
                    PlayerDetailsOut::Close => CLOSE_VIEW.ok(),
                })
            }
            (Self::Play(state), ViewMsg::Play(msg)) => {
                state.map_update(msg, context.tourn, |out| match out {
                    PlayOut::OpenLink(link) => Effect::out(Message::OpenLink(link)).ok(),
                    PlayOut::RecordGame(game_record) => {
                        Effect::out(TournamentAction::Record(game_record)).ok()
                    }
                    PlayOut::OpenPlayerInfo(player_id) => {
                        Effect::out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayOut::Close => CLOSE_VIEW.ok(),
                    PlayOut::OpenRankingConfig => Effect::out(Message::OpenPlayConfig).ok(),
                })
            }
            (Self::PlayConfig(state), ViewMsg::PlayConfig(msg)) => {
                state.map_update(msg, context.tourn, |out| match out {
                    MatchmakerConfigOut::Close => CLOSE_VIEW.ok(),
                    MatchmakerConfigOut::SaveAndClose(ranking_config) => {
                        Effect::out(TournamentAction::SetMatchmakerConfig(ranking_config))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                })
            }
            (Self::GameConfig(state), ViewMsg::GameConfig(msg)) => {
                state.map_update(msg, context.tourn, |out| match out {
                    GameConfigOut::Close => CLOSE_VIEW.ok(),
                    GameConfigOut::SaveAndClose(game_config) => {
                        Effect::out(TournamentAction::SetGameConfig(game_config))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                })
            }
            (_, message) => {
                eprintln!("Received Message {message:?} when view did not expect it.");
                Effect::done()
            }
        }
    }
}

impl ComponentView for View {
    type ViewContext<'a>
        = &'a App
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::Message> {
        match self {
            Self::PlayConfig(settings) => settings.screen_view_into(()),
            Self::PlayerDetails(player_details) => player_details.screen_view_into(context.tournament()),
            Self::Play(play) => play.screen_view_into(context.tournament()),
            Self::GameConfig(game_config) => game_config.screen_view_into(()),
        }
    }
}

impl App {
    #[must_use]
    pub fn handle_view(&self) -> Element<'_, Message> {
        let content = self.views.last().map_or_else(
            || self.home.view_into((self.tournament(), &self.file)),
            |view| view.view_into(self),
        );

        if let Some(modal) = self.modals.last() {
            modal.overlay(content)
        } else {
            content
        }
    }

    #[must_use]
    pub fn get_view(&self) -> Option<&View> {
        self.views.last()
    }

    pub fn error(&mut self, error: String) {
        self.modals.push(Modal::Error { error });
    }

    pub fn push_view<V>(&mut self, view: V) -> Effect<Message, ()>
    where
        V: Into<View>,
    {
        let view: View = view.into();
        let on_resume = view.on_resume();
        self.views.push(view);
        on_resume.map(Effect::msg).unwrap_or_default()
    }
}

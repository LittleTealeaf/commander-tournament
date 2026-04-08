use edh_tourn::tournament::Tournament;
use iced::Element;

use crate::error::{ErrorMsg, ErrorOut};
use crate::modals::Modal;
use crate::play::PlayMode;
use crate::play_config::{PlayConfig, PlayConfigMsg, PlayConfigOut};
use crate::traits::ViewScreen;
use crate::{
    App,
    app::message::Message,
    core::tournament::TournamentAction,
    effect::Effect,
    error::ErrorView,
    play::{PlayMsg, PlayOut, PlayView},
    player_details::{PlayerDetails, PlayerDetailsMsg, PlayerDetailsOut},
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Error(ErrorView),
    Play(PlayView),
    PlaySettings(PlayConfig),
    PlayerDetails(PlayerDetails),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ViewMsg {
    PlayerDetails(PlayerDetailsMsg),
    PlaySettings(PlayConfigMsg),
    Play(PlayMsg),
    Error(ErrorMsg),
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
                state.update(msg, ())?.map(|out| match out {
                    PlayerDetailsOut::OpenPlayerDetails(player_id) => {
                        Effect::Out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayerDetailsOut::DeletePlayer(player_id) => {
                        Effect::out(TournamentAction::DeletePlayer(player_id))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                    PlayerDetailsOut::OpenLink(link) => Effect::Out(Message::OpenLink(link)).ok(),
                    PlayerDetailsOut::SaveAndClose(player_id, player_info) => {
                        Effect::out(match player_id {
                            Some(id) => TournamentAction::SetPlayerInfo(id, player_info),
                            None => TournamentAction::Register(player_info),
                        })
                        .chain(CLOSE_VIEW)
                        .ok()
                    }
                    PlayerDetailsOut::OpenPlayerMatches(player_id) => {
                        Effect::out(Message::OpenPlay(PlayMode::player(player_id))).ok()
                    }
                    PlayerDetailsOut::Close => CLOSE_VIEW.ok(),
                })
            }
            (Self::Play(state), ViewMsg::Play(msg)) => {
                state.update(msg, context.tourn)?.map(|out| match out {
                    PlayOut::OpenLink(link) => Effect::out(Message::OpenLink(link)).ok(),
                    PlayOut::RecordGame(game_record) => {
                        Effect::out(TournamentAction::Record(game_record)).ok()
                    }
                    PlayOut::OpenPlayerInfo(player_id) => {
                        Effect::out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayOut::Close => CLOSE_VIEW.ok(),
                    PlayOut::OpenPlayConfig => Effect::out(Message::OpenPlayConfig).ok(),
                })
            }
            (Self::Error(state), ViewMsg::Error(msg)) => {
                state.update(msg, ())?.map(|out| match out {
                    ErrorOut::Close => CLOSE_VIEW.ok(),
                })
            }
            (Self::PlaySettings(state), ViewMsg::PlaySettings(msg)) => {
                state.update(msg, context.tourn)?.map(|out| match out {
                    PlayConfigOut::Close => CLOSE_VIEW.ok(),
                    PlayConfigOut::SaveAndClose(ranking_config) => {
                        Effect::out(TournamentAction::SetRankingConfig(ranking_config))
                            .chain(CLOSE_VIEW)
                            .chain(Effect::msg(PlayMsg::RefreshMatchup))
                            .ok()
                    }
                })
            }
            (_, _) => Effect::done(),
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
            Self::PlaySettings(settings) => settings.screen_view_into(()),
            Self::Error(error) => error.screen_view_into(()),
            Self::PlayerDetails(player_details) => {
                player_details.screen_view_into(context.tournament())
            }
            Self::Play(play) => play.screen_view_into(context.tournament()),
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

    pub fn push_view<V>(&mut self, view: V)
    where
        V: Into<View>,
    {
        self.views.push(view.into());
    }
}

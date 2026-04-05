use edh_tourn::tournament::Tournament;
use iced::Element;

use crate::error::{ErrorMsg, ErrorOut};
use crate::play::PlayMode;
use crate::traits::ViewScreen;
use crate::{
    App,
    app::message::Message,
    components::confirm::{ConfirmDialog, ConfirmDialogMsg, ConfirmDialogOut},
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
    PlayerDetails(PlayerDetails),
    Confirm(ConfirmDialog<Message>),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ViewMsg {
    PlayerDetails(PlayerDetailsMsg),
    Play(PlayMsg),
    Confirm(ConfirmDialogMsg),
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
                    PlayerDetailsOut::SaveAndClose(player_id, player_info) => match player_id {
                        Some(id) => Effect::out(TournamentAction::SetPlayerInfo(id, player_info)),
                        None => Effect::out(TournamentAction::Register(player_info)),
                    }
                    .chain(CLOSE_VIEW)
                    .ok(),
                    PlayerDetailsOut::ConfirmDialog(confirm_dialog) => {
                        let confirm: ConfirmDialog<ViewMsg> = confirm_dialog.map();
                        Effect::out(Message::OpenConfirm(Box::new(confirm.map()))).ok()
                    }
                    PlayerDetailsOut::OpenPlayerMatches(player_id) => {
                        Effect::out(Message::OpenPlay(PlayMode::player(player_id))).ok()
                    }
                    PlayerDetailsOut::Close => CLOSE_VIEW.ok(),
                })
            }
            (Self::Confirm(state), ViewMsg::Confirm(msg)) => {
                state.update(msg, ())?.map(|out| match out {
                    ConfirmDialogOut::Message(message) => Effect::out(message).ok(),
                    ConfirmDialogOut::Close => CLOSE_VIEW.ok(),
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
                })
            }
            (Self::Error(state), ViewMsg::Error(msg)) => {
                state.update(msg, ())?.map(|out| match out {
                    ErrorOut::Close => CLOSE_VIEW.ok(),
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
            Self::Error(error) => error.screen_view_into(()),
            Self::PlayerDetails(player_details) => {
                player_details.screen_view_into(context.tournament())
            }
            Self::Confirm(confirm) => confirm.view_into(()),
            Self::Play(play) => play.screen_view_into(context.tournament()),
        }
    }
}

impl App {
    #[must_use]
    pub fn handle_view(&self) -> Element<'_, Message> {
        self.views.last().map_or_else(
            || self.home.view_into((self.tournament(), &self.file)),
            |view| view.view_into(self),
        )
    }

    #[must_use]
    pub fn get_view(&self) -> Option<&View> {
        self.views.last()
    }

    pub fn error(&mut self, error: String) {
        self.push_view(ErrorView::new(error));
    }

    pub fn push_view<V>(&mut self, view: V)
    where
        V: Into<View>,
    {
        self.views.push(view.into());
    }
}

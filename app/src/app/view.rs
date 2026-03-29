use edh_tourn::tournament::Tournament;
use iced::Element;

use crate::{
    App,
    app::message::Message,
    components::confirm::{ConfirmDialog, ConfirmDialogMsg, ConfirmDialogOut},
    core::tournament::TournamentAction,
    effect::Effect,
    error::{Error, ErrorMsg},
    player_details::{PlayerDetails, PlayerDetailsMsg, PlayerDetailsOut},
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Error(Error),
    PlayerDetails(PlayerDetails),
    Confirm(ConfirmDialog<Message>),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ViewMsg {
    Error(ErrorMsg),
    PlayerDetails(PlayerDetailsMsg),
    Confirm(ConfirmDialogMsg),
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
        _: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match (self, message) {
            (Self::PlayerDetails(state), ViewMsg::PlayerDetails(msg)) => {
                state.update(msg, ())?.map(|message| match message {
                    PlayerDetailsOut::Close => Effect::Out(Message::CloseView).ok(),
                    PlayerDetailsOut::OpenPlayerDetails(player_id) => {
                        Effect::Out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayerDetailsOut::DeletePlayer(player_id) => {
                        Effect::Out(TournamentAction::DeletePlayer(player_id).into()).ok()
                    }
                    PlayerDetailsOut::OpenLink(link) => Effect::Out(Message::OpenLink(link)).ok(),
                    PlayerDetailsOut::Save(player_id, player_info) => match player_id {
                        Some(id) => {
                            Effect::out(TournamentAction::SetPlayerInfo(id, player_info)).ok()
                        }
                        None => Effect::out(TournamentAction::Register(player_info)).ok(),
                    },
                    PlayerDetailsOut::ConfirmDialog(confirm_dialog) => {
                        let confirm: ConfirmDialog<ViewMsg> = confirm_dialog.map();
                        Effect::out(Message::OpenConfirm(Box::new(confirm.map()))).ok()
                    }
                })
            }
            (Self::Error(state), ViewMsg::Error(msg)) => {
                state.update(msg, ())?.map(|message| match message {
                    ErrorMsg::CloseError => Effect::Out(Message::CloseView).ok(),
                })
            }
            (Self::Confirm(state), ViewMsg::Confirm(msg)) => {
                state.update(msg, ())?.map(|msg| match msg {
                    ConfirmDialogOut::Message(message) => Effect::out(message).ok(),
                    ConfirmDialogOut::Close => Effect::out(Message::CloseView).ok(),
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
            Self::Error(error) => error.view_into(()),
            Self::PlayerDetails(player_details) => player_details.view_into(context.tournament()),
            Self::Confirm(confirm) => confirm.view_into(()),
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
        self.push_view(Error::new(error));
    }

    pub fn push_view<V>(&mut self, view: V)
    where
        V: Into<View>,
    {
        self.views.push(view.into());
    }
}

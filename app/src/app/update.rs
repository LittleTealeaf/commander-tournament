use iced_tea::{Model, Signal};

use crate::{
    ComTourApp,
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
        tournament::TournamentAction,
    },
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Refresh,
    Nothing,
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tournament(TournamentAction),
    TournFile(FileAction),
    #[from(ignore)]
    Error(String),
    ClearError,
    ConfirmOverwrite,
    ClearOverwrite,
    QuitRequested,
    QuitConfirm(bool),
}

impl Model for ComTourApp {
    type Message = Message;
    type OutMessage = ();
    type Context<'a> = ();

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        log::debug!("Processing Update: {message:?}");

        todo!()
    }
}

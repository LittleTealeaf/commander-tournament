use iced_tea::{Model, Signal};

use crate::{
    ComTourApp,
    core::state::{AppState, AppStateMsg},
    models::tournament::TournamentMessage,
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Refresh,
    Nothing,
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tourn(TournamentMessage),
    #[from(ignore)]
    Error(String),
    RequestQuit,
    Quit,
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
        match message {
            Message::Refresh => todo!(),
            Message::Nothing => todo!(),
            Message::AppState(msg) => todo!(),
            Message::AppStateLoaded(app_state) => todo!(),
            Message::Tourn(msg) => self.tourn.update(msg, ())?.map_empty(),
            Message::Error(_) => todo!(),
            Message::RequestQuit => {
                Ok(Signal::msg(Message::Tourn(TournamentMessage::Close))
                    .chain(Signal::delayed(Message::Quit)))
            }
            Message::Quit => {
                if self.tourn.tournament().is_none() {
                    std::process::exit(0);
                }
                Signal::done()
            }
        }
    }
}

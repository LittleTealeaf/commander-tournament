use iced_tea::{Model, Signal};

use crate::{
    ComTourApp,
    models::{
        file::FileModelOut,
        state::{AppState, AppStateMsg},
        tournament::TournamentMessage,
    },
    view::{AppViewMsg, AppViewOut},
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Nothing,
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tourn(TournamentMessage),
    View(AppViewMsg),
    #[from(ignore)]
    Error(String),
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
        if log::log_enabled!(log::Level::Debug) {
            let msg = format!("{message:?}");
            let suffix = if msg.chars().count() > 200 { "..." } else { "" };
            log::debug!("Processing Update: {msg:.200}{suffix}");
        }
        match message {
            Message::View(msg) => self.view.map_update(msg, &self.tourn, |out| {
                Ok(match out {
                    AppViewOut::NewFile => Signal::msg(TournamentMessage::New),
                    AppViewOut::OpenFile => Signal::msg(TournamentMessage::Open),
                    AppViewOut::SaveFile => Signal::msg(TournamentMessage::SAVE),
                    AppViewOut::SaveFileAs => Signal::msg(TournamentMessage::SAVE_AS),
                    AppViewOut::ImportGames { source, prepend } => {
                        Signal::msg(TournamentMessage::MergeTournament {
                            other: source,
                            prepend,
                        })
                    }
                    AppViewOut::RecordGame(record) => Signal::msg(TournamentMessage::Record(record)),
                    AppViewOut::RegisterDeck(info) => Signal::msg(TournamentMessage::RegisterPlayer(info)),
                    AppViewOut::UpdateDeck(id, info) => {
                        Signal::msg(TournamentMessage::SetPlayerInfo(id, info))
                    }
                    AppViewOut::DeleteDeck(id) => Signal::msg(TournamentMessage::DeletePlayer(id)),
                    AppViewOut::MergeDecks { target, other } => {
                        Signal::msg(TournamentMessage::MergePlayer(target, other))
                    }
                    AppViewOut::DeleteGame(gid) => Signal::msg(TournamentMessage::DeleteGame(gid)),
                    AppViewOut::SaveGameConfig(cfg) => Signal::msg(TournamentMessage::SetGameConfig(cfg)),
                    AppViewOut::SaveMatchmakerConfig(cfg) => {
                        Signal::msg(TournamentMessage::SetMatchmakerConfig(cfg))
                    }
                    AppViewOut::ReloadRatings => Signal::msg(TournamentMessage::Reload),
                })
            }),
            Message::Nothing | Message::Error(_) => Signal::done(),
            Message::AppState(msg) => self
                .state
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("State does not exist"))?
                .empty_update(msg, ()),
            Message::AppStateLoaded(app_state) => {
                self.state = app_state;

                Ok(self
                    .state
                    .as_ref()
                    .and_then(|state| state.last_opened().as_ref())
                    .map_or_default(|path| Signal::msg(TournamentMessage::open(path.clone()))))
            }
            Message::Tourn(msg) => self.tourn.update(msg, ())?.map(|out| {
                Ok(match out {
                    FileModelOut::Opened(path_buf) | FileModelOut::Saved(path_buf) => {
                        Signal::msg(AppStateMsg::SetOpenedFile(path_buf))
                    }
                    FileModelOut::Closed => Signal::msg(AppStateMsg::ClearOpenedFile),
                })
            }),
            Message::Quit => {
                let modified = self.tourn.modified();
                Ok(Signal::future(async move {
                    if !modified || close_unsaved_dialog().await {
                        std::process::exit(0)
                    }
                    Message::Nothing
                }))
            }
        }
    }
}

async fn close_unsaved_dialog() -> bool {
    let result = rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Warning)
        .set_title("Quit?")
        .set_description("Are you sure you want to lose any unsaved changes?")
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
        .await;

    result == rfd::MessageDialogResult::Yes
}

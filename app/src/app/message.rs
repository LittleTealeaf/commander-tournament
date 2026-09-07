use edh_tourn::player::PlayerId;

use crate::{
    app::{MenuMsg, ViewMsg},
    components::play::PlayMode,
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
        tournament::TournamentAction,
    },
    home::HomeMsg,
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Refresh,
    Nothing,
    OpenPlayView(PlayMode),
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tournament(TournamentAction),
    TournFile(FileAction),
    OpenPlayerDetails(Option<PlayerId>),
    OpenMatchmakerConfig,
    OpenGameConfig,
    CloseView,
    #[from(ignore)]
    Error(String),
    Home(HomeMsg),
    Menu(MenuMsg),
    View(ViewMsg),
    #[from(ignore)]
    OpenLink(String),
    ClearError,
    ConfirmOverwrite,
    ClearOverwrite,
    QuitRequested,
    QuitConfirm(bool),
}

mod view;

use commander_tournament::Tournament;

use crate::app::view::player_info::MessagePlayerInfo;

pub struct AppState {
    tournament: Tournament,
    error: Option<String>,
}

pub enum Message {
    OpenPlayerInfo(Option<usize>),
    ClosePlayerInfo,
    SubmitPlayerInfo,
    ViewPlayerInfo(MessagePlayerInfo),
}

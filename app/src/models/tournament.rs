use std::path::PathBuf;

use commander_tournament_core::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::record::GameRecord,
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};
use iced_tea::Signal;
use rfd::{AsyncMessageDialog, MessageButtons, MessageDialogResult, MessageLevel};
use thiserror::Error;

use crate::models::file::{FileModel, FileModelMsg};

use super::file::FileModelOut;

#[derive(Error, Debug)]
#[error("Tournament File is not Opened")]
pub struct NoFileOpened;

#[derive(Debug, getset::CopyGetters, Default)]
pub struct TournamentModel {
    file: FileModel<Tournament>,
    #[getset(get_copy = "pub")]
    modified: bool,
    path: Option<PathBuf>,
}

impl TournamentModel {
    #[must_use]
    pub const fn tournament(&self) -> Option<&Tournament> {
        self.file.data()
    }

    pub fn require_tournament(&self) -> Result<&Tournament, NoFileOpened> {
        self.tournament().ok_or(NoFileOpened)
    }

    fn mut_tournament(&mut self) -> Result<&mut Tournament, NoFileOpened> {
        self.file.data_mut().ok_or(NoFileOpened)
    }
}

#[derive(Debug, Clone)]
pub enum TournamentMessage {
    // Tournament Actions
    RegisterPlayer(PlayerInfo),
    SetPlayerInfo(PlayerId, PlayerInfo),
    Reload,
    DeleteGame(usize),
    DeletePlayer(PlayerId),
    SetGameConfig(GameConfig),
    SetMatchmakerConfig(MatchmakerConfig),
    Record(Box<GameRecord>),
    MergeTournament(Box<Tournament>),
    MergePlayer(PlayerId, PlayerId),

    // Tournament File
    New,
    RequestOpen(PathBuf),
    Open,
    Close,
    /// Do not call this directly
    File(FileModelMsg<Tournament>),
    /// Do not call this directly
    ClearModifiedStatus,
    /// Do not call this directly
    Nothing,
}

impl TournamentMessage {
    pub const SAVE: Self = Self::File(FileModelMsg::Save);
    pub const SAVE_AS: Self = Self::File(FileModelMsg::SaveAs);

    #[must_use]
    pub const fn open(path: PathBuf) -> Self {
        Self::File(FileModelMsg::OpenFile(path))
    }
}

impl From<FileModelMsg<Tournament>> for TournamentMessage {
    fn from(value: FileModelMsg<Tournament>) -> Self {
        Self::File(value)
    }
}

impl iced_tea::Model for TournamentModel {
    type Context<'a>
        = ()
    where
        Self: 'a;
    type Message = TournamentMessage;
    type OutMessage = FileModelOut;

    fn update(
        &mut self,
        message: Self::Message,
        (): (),
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        let modified = self.modified;
        match message {
            TournamentMessage::RegisterPlayer(player_info) => {
                self.mut_tournament()?.register_player(player_info)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::SetPlayerInfo(player_id, player_info) => {
                self.mut_tournament()?.set_player_info(player_id, player_info)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::Reload => {
                self.mut_tournament()?.reload()?;
                Ok(Signal::Done)
            }
            TournamentMessage::DeleteGame(id) => {
                self.mut_tournament()?.delete_game(id)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::DeletePlayer(player_id) => {
                self.mut_tournament()?.unregister_player(player_id)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::SetGameConfig(game_config) => {
                self.mut_tournament()?.set_game_config(game_config)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::SetMatchmakerConfig(matchmaker_config) => {
                self.mut_tournament()?.set_matchmaker_config(matchmaker_config);
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::Record(game_record) => {
                self.mut_tournament()?.record_game(*game_record)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::MergeTournament(tournament) => {
                self.mut_tournament()?.merge(&tournament)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::MergePlayer(target, other) => {
                self.mut_tournament()?.merge_player(target, other)?;
                self.modified = true;
                Ok(Signal::Done)
            }
            TournamentMessage::New => Ok(Signal::future(async move {
                if !modified || unsaved_changes_dialog().await {
                    TournamentMessage::File(FileModelMsg::OpenNew(Box::new(Tournament::new())))
                } else {
                    TournamentMessage::Nothing
                }
            })),
            TournamentMessage::Open => Ok(Signal::future(async move {
                if !modified || unsaved_changes_dialog().await {
                    TournamentMessage::File(FileModelMsg::Open)
                } else {
                    TournamentMessage::Nothing
                }
            })),
            TournamentMessage::RequestOpen(path) => Ok(Signal::future(async move {
                if !modified || unsaved_changes_dialog().await {
                    TournamentMessage::File(FileModelMsg::OpenFile(path))
                } else {
                    TournamentMessage::Nothing
                }
            })),
            TournamentMessage::Close => Ok(Signal::future(async move {
                if !modified || unsaved_changes_dialog().await {
                    TournamentMessage::File(FileModelMsg::Close)
                } else {
                    TournamentMessage::Nothing
                }
            })),
            TournamentMessage::File(file_model_msg) => self
                .file
                .update(file_model_msg, ())?
                .map(|out| Ok(Signal::msg(TournamentMessage::ClearModifiedStatus).merge(Signal::out(out)))),
            TournamentMessage::ClearModifiedStatus => {
                self.modified = false;
                Signal::done()
            }
            TournamentMessage::Nothing => Signal::done(),
        }
    }
}

async fn unsaved_changes_dialog() -> bool {
    let result = AsyncMessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title("Confirm Action")
        .set_description("Are you sure you want to lose any unsaved changes?")
        .set_buttons(MessageButtons::YesNo)
        .show()
        .await;

    result == MessageDialogResult::Yes
}

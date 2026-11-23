use std::path::PathBuf;

use commander_tournament_core::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::record::GameRecord,
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};
use iced_tea::Signal;

use crate::models::file::FileModel;

#[derive(Debug, getset::Getters)]
pub struct TournamentModel {
    default_tourn: Tournament,
    file: FileModel<Tournament>,
    #[getset(get = "pub")]
    modified: bool,
    path: Option<PathBuf>,
}

impl TournamentModel {
    #[must_use]
    pub fn tournament(&self) -> &Tournament {
        self.file.data().unwrap_or(&self.default_tourn)
    }

    fn tournament_mut(&mut self) -> &mut Tournament {
        self.file.data_mut().unwrap_or(&mut self.default_tourn)
    }
}

#[derive(Debug, Clone)]
pub enum TournamentMessage {
    // Tournament Actions
    RegisterPlayer(PlayerInfo),
    SetPlayerInfo(PlayerId, PlayerInfo),
    Reload,
    DeleteGame(usize),
    SetGameConfig(GameConfig),
    SetMatchmakerConfig(MatchmakerConfig),
    Record(Box<GameRecord>),
    MergeTournament(Box<Tournament>),
    MergePlayer(PlayerId, PlayerId),
}

#[derive(Debug, Clone)]
pub enum TournamentOutMessage {
    /// Requests a prompt to confirm that the usre want's to lose all unsaved progress
    ///
    /// If true, then the provided message should be sent
    RequestOverwrite(TournamentMessage),
}

impl iced_tea::Model for TournamentModel {
    type Context<'a>
        = ()
    where
        Self: 'a;
    type Message = TournamentMessage;
    type OutMessage = ();

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: (),
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        todo!()
    }
}

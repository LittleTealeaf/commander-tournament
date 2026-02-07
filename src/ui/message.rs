use crate::ui::state::MatchupType;

#[derive(Debug, Clone)]
pub enum Message {
    ShowIngest(bool),
    UpdateIngest(String),
    SubmitIngest,
    ShowExport(bool),
    UpdateExport(String),
    ShowConfig(bool),
    UpdateScoreStartingElo(String),
    UpdateScoreGamePoints(String),
    UpdateScoreEloPow(String),
    UpdateScoreWrPow(String),
    UpdateScoreEloWeight(String),
    UpdateScoreWrWeight(String),

    UpdateMatchWeightLeastPlayed(String),
    UpdateMatchWeightNemesis(String),
    UpdateMatchWeightNeighbor(String),
    UpdateMatchWeightWrNeighbor(String),
    UpdateMatchWeightLostWith(String),
    SaveConfig,
    SelectPlayer(usize, Option<String>),
    SelectPlayers([String; 4]),
    AddPlayerToNextSlot(String),
    SelectWinner(String),
    SelectMatchPlayer(String),
    SubmitGame,
    ClearGame,
    Reload,
    CloseError,
    Load(String),
    Save(String),
    New,
    SetMatchupType(MatchupType),
    ShowGames(bool),
    OpenChangeWinnerModal(usize),
    ChangeGameWinner(usize, String),
    DeleteGame(usize),
    /// Open the player info/edit screen for a specific player
    ShowPlayerInfo(String),
    /// Close the player info/edit screen
    ClosePlayerInfo,
    /// Update the player name in player info
    SetPlayerName(String),
    /// Update the description in player info
    SetPlayerDescription(String),
    /// Update the moxfield link in player info
    SetPlayerMoxfieldLink(String),
    /// Toggle a color for the player
    TogglePlayerColor(crate::tournament::MtgColor),
    /// Save the player details
    SavePlayerDetails,
}

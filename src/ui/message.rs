use crate::tournament::ScoreConfig;
use crate::ui::state::MatchupType;

#[derive(Debug, Clone)]
pub enum Message {
    Ingest,
    SetChangePlayerName(Option<(Option<String>, String)>),
    ChangePlayerSubmit,
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
    SelectWinner(String),
    SelectMatchPlayer(String),
    DeletePlayer(String),
    SubmitGame,
    Reload,
    SetScoreConfig(ScoreConfig),
    CloseError,
    Load(String),
    Save(String),
    New,
    SetMatchupType(MatchupType),
}

pub mod match_preview;
mod update;
mod view;

use core::fmt::Display;

use edh_tourn::{
    game::{POD_SIZE, record::GameRecord},
    player::PlayerId,
    ranking::RankingMethod,
    tournament::Tournament,
};

use crate::{
    play::match_preview::{MatchPreview, MatchPreviewMsg},
    traits::{Component, ViewScreen},
};

#[derive(Debug, Clone)]
pub struct PlayView {
    mode: PlayMode,
    match_preview: Option<MatchPreview>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayNextMode {
    LeastGames,
    LongestBreak,
}

impl Display for PlayNextMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::LeastGames => "Least Games",
            Self::LongestBreak => "Longest Break",
        })
    }
}

impl PlayNextMode {
    const VALUES: [Self; 2] = [Self::LeastGames, Self::LongestBreak];
}

#[derive(Debug, Clone)]
pub enum PlayMode {
    Player {
        id: PlayerId,
        ranking: RankingMethod,
    },
    Next {
        ranking: RankingMethod,
        mode: PlayNextMode,
    },
    Custom {
        players: [Option<PlayerId>; POD_SIZE],
    },
}

impl PlayMode {
    #[must_use]
    pub const fn player(id: PlayerId) -> Self {
        Self::Player {
            id,
            ranking: RankingMethod::Combined,
        }
    }

    #[must_use]
    pub const fn next() -> Self {
        Self::Next {
            ranking: RankingMethod::Combined,
            mode: PlayNextMode::LongestBreak,
        }
    }

    #[must_use]
    pub const fn custom() -> Self {
        Self::Custom {
            players: [None; POD_SIZE],
        }
    }
}

impl PlayView {
    #[must_use]
    pub fn new(mode: PlayMode, tournament: &Tournament) -> Self {
        Self {
            match_preview: mode.create_matchup(tournament).map(MatchPreview::new),
            mode,
        }
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum PlayMsg {
    Close,
    RefreshMatchup,
    SetRankingMethod(RankingMethod),
    SetNextMode(PlayNextMode),
    OpenLink(String),
    OpenLinks(Vec<String>),
    SetPlayer(usize, Option<PlayerId>),
    Preview(MatchPreviewMsg),
}

#[derive(Clone, Debug)]
pub enum PlayOut {
    Close,
    OpenLink(String),
    RecordGame(Box<GameRecord>),
    OpenPlayerInfo(PlayerId),
}

impl Component for PlayView {
    type Message = PlayMsg;
    type OutMessage = PlayOut;
}

impl ViewScreen for PlayView {
    const CLOSE_MESSAGE: Self::Message = PlayMsg::Close;
    fn title<'a>(&'a self, context: Self::ViewContext<'a>) -> String {
        match &self.mode {
            PlayMode::Player { id, .. } => format!(
                "Play: {}",
                context
                    .get_player_name(id)
                    .map_or("Unknown Player", |id| id.as_ref())
            ),
            PlayMode::Next { .. } => "Play Tournament".to_owned(),
            PlayMode::Custom { .. } => "Custom Games".to_owned(),
        }
    }
}

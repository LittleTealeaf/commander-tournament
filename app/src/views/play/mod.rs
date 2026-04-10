pub mod match_preview;
mod update;
mod view;

use core::fmt::Display;

use auto_const_array::auto_const_array;
use edh_tourn::{
    game::{POD_SIZE, record::GameRecord},
    player::PlayerId,
    ranking::RankingMethod,
    tournament::Tournament,
};
use iced::widget::button;
use nerd_font_symbols::md::MD_COG;

use crate::{
    traits::Component,
    views::{
        ViewScreen,
        play::match_preview::{MatchPreview, MatchPreviewMsg},
    },
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
    LeastWins,
    OutlierWinrate,
    LowestWinrate,
    HighestWinrate,
}

impl Display for PlayNextMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::LeastGames => "Least Games",
            Self::LongestBreak => "Longest Break",
            Self::LeastWins => "Least Wins",
            Self::OutlierWinrate => "Outlier Winrate",
            Self::LowestWinrate => "Lowest Winrate",
            Self::HighestWinrate => "Highest Winrate",
        })
    }
}

impl PlayNextMode {
    auto_const_array! {
        const VALUES: [Self; _] = [
            Self::LongestBreak,
            Self::LeastGames,
            Self::LeastWins,
            Self::OutlierWinrate,
            Self::LowestWinrate,
            Self::HighestWinrate,
        ]
    }
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
        ignore_precons: bool,
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
            ignore_precons: false,
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
    OpenConfig,
    SetRankingMethod(RankingMethod),
    SetNextMode(PlayNextMode),
    OpenLink(String),
    OpenLinks(Vec<String>),
    SetPlayer(usize, Option<PlayerId>),
    Preview(MatchPreviewMsg),
    IgnorePrecons(bool),
}

#[derive(Clone, Debug)]
pub enum PlayOut {
    Close,
    OpenRankingConfig,
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
    const ON_RESUME: Option<Self::Message> = Some(PlayMsg::RefreshMatchup);

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

    fn secondary_actions<'a>(
        &'a self,
        _: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        matches!(self.mode, PlayMode::Next { .. } | PlayMode::Player { .. })
            .then_some(button(MD_COG).on_press(PlayMsg::OpenConfig))
    }
}

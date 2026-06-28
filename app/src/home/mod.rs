use edh_tourn::{game::{next_mode::NextPlayerMode, record::GameRecord}, player::PlayerId, tournament::Tournament};
use iced::{
    Length,
    widget::{button, column, container, pick_list, responsive, row, space, text},
};
use nerd_font_symbols::md::{MD_CLOSE, MD_COGS};

use crate::{
    components::{
        play::{PlayComponent, PlayComponentMsg, PlayComponentOut, PlayMode},
        tab_bar,
    },
    effect::Effect,
    home::leaderboard::{Leaderboard, LeaderboardMsg, LeaderboardOut},
    traits::{Component, ComponentUpdate, ComponentView},
};

pub mod leaderboard;

const SCREEN_WIDTH_BREAKPOINT: f32 = 1500.0;

#[derive(Debug, Default)]
pub struct Home {
    leaderboard: Leaderboard,
    play: PlayComponent,
    tab: HomeTab,
}

#[derive(Debug, Clone, Copy, derive_more::Display, Default, PartialEq, Eq)]
pub enum HomeTab {
    #[default]
    Leaderboard,
    #[display("Play Game")]
    PlayGame,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeMsg {
    Refresh,
    Leaderboard(LeaderboardMsg),
    SetTab(HomeTab),
    Play(PlayComponentMsg),
    SelectPlayNextMode(NextPlayerMode),
    SetPlayMode(PlayMode),
    OpenMatchmakerConfig,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeOut {
    RecordGame(Box<GameRecord>),
    OpenLink(String),
    OpenPlayerDetails(PlayerId),
    OpenNewPlayer,
    OpenMatchmakerConfig,
}

impl Component for Home {
    type Message = HomeMsg;
    type OutMessage = HomeOut;
}

impl ComponentView for Home {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        responsive(|size: iced::Size| match size.width {
            ..=SCREEN_WIDTH_BREAKPOINT => column![
                tab_bar(
                    &self.tab,
                    [HomeTab::Leaderboard, HomeTab::PlayGame],
                    HomeMsg::SetTab
                ),
                container(match self.tab {
                    HomeTab::Leaderboard => self.leaderboard.view_into(context),
                    HomeTab::PlayGame => self.view_component_play(context),
                })
                .width(Length::Fill)
            ]
            .into(),
            _ => row![
                container(self.leaderboard.view_into(context)).width(Length::FillPortion(1)),
                container(self.view_component_play(context)).width(Length::FillPortion(1))
            ]
            .into(),
        })
        .into()
    }
}

impl Home {
    fn view_component_play<'a>(&'a self, context: &'a Tournament) -> iced::Element<'a, HomeMsg> {
        column![
            row![
                match self.play.mode() {
                    PlayMode::Next(mode) => {
                        row![
                            pick_list(NextPlayerMode::VALUES, Some(mode), |select| {
                                HomeMsg::SelectPlayNextMode(select)
                            }),
                            space().width(Length::Fill),
                            button("Custom").on_press(HomeMsg::SetPlayMode(PlayMode::custom()))
                        ]
                    }
                    PlayMode::Custom(_) => {
                        row![
                            button(MD_CLOSE).on_press(HomeMsg::SetPlayMode(PlayMode::next())),
                            container(text("Custom Game")).padding(button::DEFAULT_PADDING),
                            space().width(Length::Fill),
                        ]
                    }
                    PlayMode::Player(player) => {
                        let name = context.get_player_name(player).map_or_else(
                            || "Player Game: <Unknown>".to_owned(),
                            |name| format!("Play Game: {name}"),
                        );

                        row![
                            button(MD_CLOSE).on_press(HomeMsg::SetPlayMode(PlayMode::next())),
                            container(text(name)).padding(button::DEFAULT_PADDING),
                            space().width(Length::Fill),
                        ]
                    }
                }
                .width(Length::Fill),
                button(MD_COGS).on_press(HomeMsg::OpenMatchmakerConfig)
            ]
            .spacing(10)
            .padding(10),
            self.play.view_into(context)
        ]
        .into()
    }
}

impl ComponentUpdate for Home {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            HomeMsg::SetPlayMode(mode_type) => {
                self.play.set_mode(mode_type, context);
                Effect::done()
            }
            HomeMsg::SelectPlayNextMode(mode) => {
                self.play.set_next_mode(mode, context);
                Effect::done()
            }
            HomeMsg::OpenMatchmakerConfig => Effect::out(HomeOut::OpenMatchmakerConfig).ok(),
            HomeMsg::Refresh => Effect::msg(HomeMsg::Play(PlayComponentMsg::Refresh)).ok(),
            HomeMsg::Leaderboard(message) => self.leaderboard.map_update(message, (), |msg| match msg {
                LeaderboardOut::RankPlayer(id) => Effect::msg(HomeMsg::SetPlayMode(PlayMode::Player(id)))
                    .merge(Effect::msg(HomeMsg::SetTab(HomeTab::PlayGame)))
                    .ok(),
                LeaderboardOut::OpenPlayerDetails(player_id) => {
                    Effect::out(HomeOut::OpenPlayerDetails(player_id)).ok()
                }
                LeaderboardOut::OpenNewPlayer => Effect::out(HomeOut::OpenNewPlayer).ok(),
            }),
            HomeMsg::SetTab(home_tab) => {
                self.tab = home_tab;
                Effect::done()
            }
            HomeMsg::Play(msg) => self.play.map_update(msg, context, |out| match out {
                PlayComponentOut::OpenLink(link) => Effect::out(HomeOut::OpenLink(link)).ok(),
                PlayComponentOut::OpenPlayer(player_id) => {
                    Effect::out(HomeOut::OpenPlayerDetails(player_id)).ok()
                }
                PlayComponentOut::RecordGame(game_record) => {
                    Effect::out(HomeOut::RecordGame(game_record)).ok()
                }
            }),
        }
    }
}

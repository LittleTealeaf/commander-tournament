use core::fmt::Display;

use edh_tourn::Tournament;
use iced::{
    Length, Task,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, table, text},
};
use itertools::{Itertools, chain};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::home::{HomeMessage, matchup::MatchupMessage},
};

pub struct MatchMakerView {
    method: MatchMethod,
    player: Option<u32>,
    leaderboard: Vec<u32>,
    show_count: usize,
}
impl Default for MatchMakerView {
    fn default() -> Self {
        Self {
            method: MatchMethod::default(),
            player: None,
            leaderboard: Vec::new(),
            show_count: 5,
        }
    }
}

impl MatchMakerView {
    fn get_leaderboard<'a>(&'a self, tournament: &'a Tournament) -> Option<Vec<u32>> {
        self.player.and_then(|id| {
            Some(match &self.method {
                MatchMethod::LeastPlayed => tournament
                    .rank_least_played(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::ExpectedNeighbors => tournament
                    .rank_expected_neighbors(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::LossWith => tournament
                    .rank_loss_with(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::Nemesis => tournament
                    .rank_nemesis(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::EloNeighbors => tournament
                    .rank_elo_neighbors(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::WRNeighbors => tournament
                    .rank_wr_neighbors(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
                MatchMethod::Combined => tournament
                    .rank_combined(id)
                    .ok()?
                    .take(self.show_count)
                    .collect_vec(),
            })
        })
    }

    fn update(&mut self, tournament: &Tournament) -> anyhow::Result<()> {
        let Some(id) = self.player else {
            self.leaderboard = Vec::new();
            return Ok(());
        };

        self.leaderboard = match &self.method {
            MatchMethod::LeastPlayed => tournament.rank_least_played(id)?.collect_vec(),
            MatchMethod::ExpectedNeighbors => tournament.rank_expected_neighbors(id)?.collect_vec(),
            MatchMethod::LossWith => tournament.rank_loss_with(id)?.collect_vec(),
            MatchMethod::Nemesis => tournament.rank_nemesis(id)?.collect_vec(),
            MatchMethod::EloNeighbors => tournament.rank_elo_neighbors(id)?.collect_vec(),
            MatchMethod::WRNeighbors => tournament.rank_wr_neighbors(id)?.collect_vec(),
            MatchMethod::Combined => tournament.rank_combined(id)?.collect_vec(),
        }
        .into_iter()
        .take(self.show_count)
        .collect_vec();

        Ok(())
    }
}

#[derive(Clone, Default, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchMethod {
    LeastPlayed,
    ExpectedNeighbors,
    LossWith,
    Nemesis,
    EloNeighbors,
    WRNeighbors,
    #[default]
    Combined,
}

impl MatchMethod {
    pub const VALUES: [Self; 7] = [
        Self::Combined,
        Self::LeastPlayed,
        Self::Nemesis,
        Self::ExpectedNeighbors,
        Self::EloNeighbors,
        Self::WRNeighbors,
        Self::LossWith,
    ];
}

impl Display for MatchMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LeastPlayed => write!(f, "Least Played"),
            Self::ExpectedNeighbors => write!(f, "Expected Neighbors"),
            Self::LossWith => write!(f, "Loss With"),
            Self::Nemesis => write!(f, "Nemesis"),
            Self::EloNeighbors => write!(f, "Elo Neighbors"),
            Self::WRNeighbors => write!(f, "WR Neighbors"),
            Self::Combined => write!(f, "Combined"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchMakerMessage {
    Method(MatchMethod),
    Player(Option<u32>),
    ViewCount(usize),
    LoadTopThree,
}

impl From<MatchMakerMessage> for Message {
    fn from(value: MatchMakerMessage) -> Self {
        HomeMessage::MatchmakerMessage(value).into()
    }
}

impl HandleMessage<MatchMakerMessage> for App {
    fn update(
        &mut self,
        msg: MatchMakerMessage,
    ) -> anyhow::Result<iced::Task<crate::logic::Message>> {
        let view = &mut self.home.matchmaker;
        match msg {
            MatchMakerMessage::Method(match_method) => {
                view.method = match_method;
                view.update(&self.tournament)?;
                Message::done()
            }
            MatchMakerMessage::Player(player) => {
                view.player = player;
                view.update(&self.tournament)?;
                Message::done()
            }
            MatchMakerMessage::ViewCount(count) => {
                view.show_count = count;
                view.update(&self.tournament)?;
                Message::done()
            }
            MatchMakerMessage::LoadTopThree => {
                let Some(id) = view.player else {
                    return Message::done();
                };

                let matchup_updates = chain!(
                    [MatchupMessage::Clear, MatchupMessage::AddPlayer(id)],
                    view.leaderboard
                        .iter()
                        .take(3)
                        .copied()
                        .map(MatchupMessage::AddPlayer)
                )
                .collect_vec();

                let mut tasks = Vec::new();

                for msg in matchup_updates {
                    tasks.push(self.update(msg)?);
                }

                Ok(Task::batch(tasks))
            }
        }
    }
}

impl View<MatchMakerView> for App {
    fn view<'a>(&'a self, scene: &'a MatchMakerView) -> iced::Element<'a, Message> {
        container(
            column![
                text("Match Maker")
                    .size(18)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                pick_list(
                    self.tournament()
                        .get_registered_players()
                        .sorted_by(|a, b| a.info().name().cmp(b.info().name()))
                        .collect_vec(),
                    scene
                        .player
                        .and_then(|id| self.tournament().get_registered_player(id).ok()),
                    |player| MatchMakerMessage::Player(Some(player.id())).into()
                )
                .width(Length::Fill),
                row![
                    pick_list(MatchMethod::VALUES, Some(scene.method), |method| {
                        MatchMakerMessage::Method(method).into()
                    }),
                    button("Load Top 3").on_press_maybe(
                        scene
                            .player
                            .is_some()
                            .then_some(MatchMakerMessage::LoadTopThree.into())
                    ),
                    button("⚙")
                ]
                .spacing(10),
                table(
                    [table::column(text(""), |player: u32| {
                        text(
                            self.tournament()
                                .get_player_name(&player)
                                .cloned()
                                .unwrap_or_default(),
                        )
                    })],
                    scene.get_leaderboard(self.tournament()).unwrap_or_default()
                )
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

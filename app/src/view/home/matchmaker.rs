use edh_tourn::{Tournament, matches::RankMethod};
use iced::{
    Length, Task,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::{Itertools, chain};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::{
        config_matchmaker::MessageMatchmakerConfig,
        home::{HomeMessage, matchup::MatchupMessage},
    },
};

#[derive(Debug)]
pub struct MatchMakerView {
    method: RankMethod,
    player: Option<u32>,
    show_count: usize,
}

impl Default for MatchMakerView {
    fn default() -> Self {
        Self {
            method: RankMethod::default(),
            player: None,
            show_count: 7,
        }
    }
}

impl MatchMakerView {
    fn get_leaderboard<'a>(&'a self, tournament: &'a Tournament) -> Option<Vec<u32>> {
        self.player.and_then(|id| {
            Some(
                tournament
                    .ranked_opponents(id, self.method)
                    .ok()?
                    .into_iter()
                    .take(self.show_count)
                    .collect(),
            )
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchMakerMessage {
    Method(RankMethod),
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
                Message::done()
            }
            MatchMakerMessage::Player(player) => {
                view.player = player;
                Message::done()
            }
            MatchMakerMessage::ViewCount(count) => {
                view.show_count = count;
                Message::done()
            }
            MatchMakerMessage::LoadTopThree => {
                let Some(id) = view.player else {
                    return Message::done();
                };

                let matchup_updates = chain!(
                    [MatchupMessage::Clear, MatchupMessage::AddPlayer(id)],
                    view.get_leaderboard(&self.tournament)
                        .unwrap_or_default()
                        .into_iter()
                        .take(3)
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

fn results_table(
    results: Vec<u32>,
    tournament: &Tournament,
) -> iced::widget::table::Table<'_, Message> {
    table(
        [
            table::column(text(""), |player: u32| {
                text(
                    tournament
                        .get_player_name(&player)
                        .cloned()
                        .unwrap_or_default(),
                )
            }),
            table::column(text(""), |player: u32| {
                let stats = tournament.get_player_or_default_stats(player);
                let elo = stats.elo().round();
                let wr = stats.wr().map_or_else(
                    || "--% WR".to_owned(),
                    |wr| format!("{}% WR", (wr * 100.0).round()),
                );
                text(format!("{elo} Elo, {wr}"))
            }),
            table::column(text(""), |player: u32| {
                button("+").on_press(MatchupMessage::AddPlayer(player).into())
            }),
        ],
        results,
    )
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
                    pick_list(RankMethod::VALUES, Some(scene.method), |method| {
                        MatchMakerMessage::Method(method).into()
                    }),
                    space().width(Length::Fill),
                    button("Load Top 3").on_press_maybe(
                        scene
                            .player
                            .is_some()
                            .then_some(MatchMakerMessage::LoadTopThree.into())
                    ),
                    button("⚙").on_press(MessageMatchmakerConfig::Open.into())
                ]
                .spacing(10),
                results_table(
                    scene.get_leaderboard(self.tournament()).unwrap_or_default(),
                    self.tournament()
                )
                .width(Length::Fill)
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

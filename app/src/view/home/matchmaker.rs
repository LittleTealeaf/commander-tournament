use edh_tourn::{Tournament, analytics::ranking::RankingMethod, player::RegisteredPlayer};
use iced::{
    Length, Task,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::{Itertools, chain};
use nerd_font_symbols::oct::OCT_GEAR;

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::{
        config_matchmaker::MessageMatchmakerConfig,
        home::{HomeMessage, matchup::MatchupMessage},
        player::ViewPlayerMessage,
    },
};

#[derive(Debug)]
pub struct MatchMakerView {
    method: RankingMethod,
    player: Option<u32>,
    show_count: usize,
}

impl Default for MatchMakerView {
    fn default() -> Self {
        Self {
            method: RankingMethod::default(),
            player: None,
            show_count: 7,
        }
    }
}

impl MatchMakerView {
    fn get_leaderboard<'a>(
        &'a self,
        tournament: &'a Tournament,
    ) -> Option<Vec<RegisteredPlayer<'a>>> {
        self.player.and_then(|id| {
            Some(
                tournament
                    .get_player_ranked(id, self.method)
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
    Method(RankingMethod),
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
                        .map(|player| player.id())
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

fn results_table(results: Vec<RegisteredPlayer<'_>>) -> iced::widget::table::Table<'_, Message> {
    table(
        [
            table::column(text("Player"), |player: RegisteredPlayer<'_>| {
                button(text(player.info().name().clone()))
                    .style(button::text)
                    .on_press(ViewPlayerMessage::Open(Some(player.id())).into())
            }),
            table::column(text("Stats"), |player: RegisteredPlayer<'_>| {
                let elo = player.stats().elo().round();
                let wr = player.stats().wr().map_or_else(
                    || "--% WR".to_owned(),
                    |wr| format!("{}% WR", (wr * 100.0).round()),
                );
                text(format!("{elo} Elo, {wr}"))
            }),
            table::column(text("Add"), |player: RegisteredPlayer<'_>| {
                button("+").on_press(MatchupMessage::AddPlayer(player.id()).into())
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
                row![
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
                    button("󰘸").on_press_maybe(
                        scene
                            .player
                            .map(|id| ViewPlayerMessage::Open(Some(id)).into())
                    )
                ]
                .width(Length::Fill)
                .spacing(10),
                row![
                    pick_list(RankingMethod::VALUES, Some(scene.method), |method| {
                        MatchMakerMessage::Method(method).into()
                    }),
                    space().width(Length::Fill),
                    button("Load Top 3").on_press_maybe(
                        scene
                            .player
                            .is_some()
                            .then_some(MatchMakerMessage::LoadTopThree.into())
                    ),
                    button(OCT_GEAR).on_press(MessageMatchmakerConfig::Open.into())
                ]
                .spacing(10),
                results_table(scene.get_leaderboard(self.tournament()).unwrap_or_default())
                    .width(Length::Fill)
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

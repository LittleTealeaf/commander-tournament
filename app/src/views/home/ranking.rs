use anyhow::anyhow;

use edh_tourn::{player::RegisteredPlayer, ranking::RankingMethod, tournament::Tournament};
use iced::{
    Length,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::MD_CARDS;

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

#[derive(Debug, Default)]
pub struct State {
    method: RankingMethod,
    player: Option<u32>,
}

impl State {
    fn rank_players<'a>(&'a self, tourn: &'a Tournament) -> Option<Vec<RegisteredPlayer<'a>>> {
        self.player.and_then(|id| {
            Some(
                tourn
                    .get_player_ranked(id, self.method)
                    .ok()?
                    .into_iter()
                    .take(10)
                    .collect(),
            )
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectPlayer(u32),
    OpenPlayerDetails(u32),
    AddTopThree,
    SetMethod(RankingMethod),
}

#[derive(Debug)]
pub enum OutMessage {
    LoadGame([u32; 4]),
}

impl Component for State {
    type OutMessage = OutMessage;
    type Message = Message;
    type Context<'a> = &'a Tournament;
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayer(id) => {
                context.require_id_registered(id)?;
                self.player = Some(id);
                Effect::ok()
            }
            Message::AddTopThree => {
                let Some(id) = self.player else {
                    return Err(anyhow!("Player not specified"));
                };
                let ranked = context.get_player_ranked(id, self.method)?;
                let mut ranked_iter = ranked.iter().map(RegisteredPlayer::id);
                let players = [
                    id,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("No players found in ranking"))?,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("Only 1 player found in ranking"))?,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("Only 2 players found in ranking"))?,
                ];

                Effect::out(OutMessage::LoadGame(players))
            }
            Message::SetMethod(method) => {
                self.method = method;
                Effect::ok()
            }
            Message::OpenPlayerDetails(_) => todo!(),
        }
    }
}

impl ComponentView for State {
    fn view<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        let tourn = context;
        container(
            column![
                text("Match Maker")
                    .size(18)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                row![
                    pick_list(
                        tourn
                            .get_registered_players()
                            .sorted_by(|a, b| a.info().name().cmp(b.info().name()))
                            .collect_vec(),
                        self.player.and_then(|id| tourn.get_registered_player(id)),
                        |player| Message::SelectPlayer(player.id())
                    )
                    .width(Length::Fill),
                    button(MD_CARDS).on_press_maybe(self.player.map(Message::OpenPlayerDetails))
                ]
                .width(Length::Fill)
                .spacing(10),
                row![
                    pick_list(RankingMethod::VALUES, Some(self.method), |method| {
                        Message::SetMethod(method)
                    }),
                    space().width(Length::Fill),
                    button("Load Top 3")
                        .on_press_maybe(self.player.is_some().then_some(Message::AddTopThree)),
                ]
                .spacing(10),
                table(
                    [
                        table::column(text("Player"), |player: RegisteredPlayer<'_>| {
                            button(text(player.info().name().clone()))
                                .style(button::text)
                                .on_press(Message::OpenPlayerDetails(player.id()))
                        }),
                        table::column(text("Stats"), |player: RegisteredPlayer<'_>| {
                            let elo = player.stats().elo().round();
                            let wr = player.stats().wr().map_or_else(
                                || "--% WR".to_owned(),
                                |wr| format!("{}% WR", (wr * 100.0).round()),
                            );
                            text(format!("{elo} Elo, {wr}"))
                        }),
                    ],
                    self.rank_players(tourn).into_iter().flatten(),
                )
                .width(Length::Fill)
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

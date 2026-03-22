use anyhow::anyhow;
use edh_tourn::ranking::RankingMethod;
use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

use iced::{
    Length,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};

use itertools::Itertools;
use nerd_font_symbols::md::MD_CARDS;

#[derive(Debug, Default)]
pub struct State {
    method: RankingMethod,
    player: Option<u32>,
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
}

fn create_game_players<'a>(
    id: u32,
    ranked: impl IntoIterator<Item = RegisteredPlayer<'a>>,
) -> Option<[u32; 4]> {
    let mut iter = ranked.into_iter().map(|player| player.id());
    Some([id, iter.next()?, iter.next()?, iter.next()?])
}

impl ComponentUpdate for State {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayer(id) => {
                context.require_id_registered(id)?;
                self.player = Some(id);
                Effect::done()
            }
            Message::AddTopThree => {
                let Some(id) = self.player else {
                    return Err(anyhow!("Player not specified"));
                };
                let ranked = context.get_player_ranked(id, self.method)?;
                let players =
                    create_game_players(id, ranked).ok_or_else(|| anyhow!("Not enough players"))?;

                Effect::out(OutMessage::LoadGame(players))
            }
            Message::SetMethod(method) => {
                self.method = method;
                Effect::done()
            }
            Message::OpenPlayerDetails(id) => {
                Effect::global(crate::core::message::Message::OpenPlayerDetails(Some(id)))
            }
        }
    }
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

impl ComponentView for State {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
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

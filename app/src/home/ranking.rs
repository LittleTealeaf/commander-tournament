use anyhow::anyhow;
use edh_tourn::ranking::RankingMethod;
use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};

use crate::core::message::Message;
use crate::effect::Effect;
use crate::traits::{Component, ComponentUpdate, ComponentView};

use iced::{
    Length,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};

use itertools::Itertools;
use nerd_font_symbols::md::MD_CARDS;

#[derive(Debug, Default)]
pub struct Ranking {
    method: RankingMethod,
    player: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum RankingMsg {
    SelectPlayer(u32),
    OpenPlayerDetails(u32),
    AddTopThree,
    SetMethod(RankingMethod),
}

#[derive(Debug)]
pub enum RankingOut {
    LoadGame([u32; 4]),
}

impl Component for Ranking {
    type OutMessage = RankingOut;
    type Message = RankingMsg;
}

fn create_game_players<'a>(
    id: u32,
    ranked: impl IntoIterator<Item = RegisteredPlayer<'a>>,
) -> Option<[u32; 4]> {
    let mut iter = ranked.into_iter().map(|player| player.id());
    Some([id, iter.next()?, iter.next()?, iter.next()?])
}

impl ComponentUpdate for Ranking {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            RankingMsg::SelectPlayer(id) => {
                context.require_id_registered(id)?;
                self.player = Some(id);
                Effect::done()
            }
            RankingMsg::AddTopThree => {
                let Some(id) = self.player else {
                    return Err(anyhow!("Player not specified"));
                };
                let ranked = context.ranking().ranked(id, self.method)?;
                let players =
                    create_game_players(id, ranked).ok_or_else(|| anyhow!("Not enough players"))?;

                Effect::Out(RankingOut::LoadGame(players)).ok()
            }
            RankingMsg::SetMethod(method) => {
                self.method = method;
                Effect::done()
            }
            RankingMsg::OpenPlayerDetails(id) => {
                Effect::global(Message::OpenPlayerDetails(Some(id))).ok()
            }
        }
    }
}

impl Ranking {
    fn rank_players<'a>(&'a self, tourn: &'a Tournament) -> Option<Vec<RegisteredPlayer<'a>>> {
        self.player.and_then(|id| {
            Some(
                tourn
                    .ranking()
                    .ranked(id, self.method)
                    .ok()?
                    .into_iter()
                    .take(10)
                    .collect(),
            )
        })
    }
}

impl ComponentView for Ranking {
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
                        |player| RankingMsg::SelectPlayer(player.id())
                    )
                    .width(Length::Fill),
                    button(MD_CARDS).on_press_maybe(self.player.map(RankingMsg::OpenPlayerDetails))
                ]
                .width(Length::Fill)
                .spacing(10),
                row![
                    pick_list(RankingMethod::VALUES, Some(self.method), |method| {
                        RankingMsg::SetMethod(method)
                    }),
                    space().width(Length::Fill),
                    button("Load Top 3")
                        .on_press_maybe(self.player.is_some().then_some(RankingMsg::AddTopThree)),
                ]
                .spacing(10),
                table(
                    [
                        table::column(text("Player"), |player: RegisteredPlayer<'_>| {
                            button(text(player.info().name().clone()))
                                .style(button::text)
                                .on_press(RankingMsg::OpenPlayerDetails(player.id()))
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

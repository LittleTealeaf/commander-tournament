use edh_tourn::{player::RegisteredPlayer, ranking::RankingMethod, tournament::Tournament};

use crate::{traits::ComponentView, views::home::ranking::Message};
use iced::{
    Length,
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::MD_CARDS;

use crate::views::home::ranking::State;

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

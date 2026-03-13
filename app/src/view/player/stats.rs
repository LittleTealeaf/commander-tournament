use edh_tourn::{
    Tournament,
    analytics::winloss::MatchPerformance,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
        stats::PlayerStats,
    },
};
use iced::{
    Length, Padding,
    alignment::{Horizontal, Vertical},
    widget::{Container, button, column, container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::{logic::Message, view::player::ViewPlayerMessage};

pub fn view_stats_summary(stats: &PlayerStats) -> Container<'_, Message> {
    container(
        row![
            column![
                text(format!("{} Elo", stats.elo().round())).size(25),
                text(format!("{} Peak", stats.elo_peak().round())).size(15)
            ]
            .align_x(Horizontal::Left),
            space().width(Length::Fill),
            column![
                text(format!("Games Played: {}", stats.games())),
                text(format!("Games Won: {}", stats.wins())),
                {
                    stats.wr().map_or_else(
                        || text("--% WR"),
                        |wr| text(format!("{}% WR", (wr * 100.0).round())),
                    )
                }
            ]
            .align_x(Horizontal::Right)
        ]
        .align_y(Vertical::Center)
        .spacing(20),
    )
}

pub fn view_player_matchups(tournament: &Tournament, id: u32) -> Option<Container<'_, Message>> {
    type RowType<'a> = (RegisteredPlayer<'a>, MatchPerformance);
    let matchups = tournament
        .get_player_player_match_performance(id)
        .ok()?
        .sorted_by_key(|&(_, key)| key)
        .rev();

    Some(container(
        scrollable(
            table(
                [
                    table::column("Player", |(player, _): RowType| {
                        button(text(format!(
                            "{} ({})",
                            player.info().name(),
                            player.info().color_identity()
                        )))
                        .style(button::text)
                        .padding(Padding::new(0.0))
                        .on_press(ViewPlayerMessage::Open(Some(player.id())).into())
                    }),
                    table::column("Wins", |(_, perf): RowType| {
                        text(format!("{}", perf.wins()))
                    }),
                    table::column("Draws", |(_, perf): RowType| {
                        text(format!("{}", perf.draws()))
                    }),
                    table::column("Losses", |(_, perf): RowType| {
                        text(format!("{}", perf.losses()))
                    }),
                ],
                matchups,
            )
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ))
}

pub fn view_identity_matchups(tournament: &Tournament, id: u32) -> Option<Container<'_, Message>> {
    type RowType<'a> = (ColorIdentity, MatchPerformance);
    let matchups = tournament
        .get_player_identity_match_performance(id)
        .ok()?
        .into_iter()
        .sorted_by_key(|&(_, key)| key)
        .rev();

    Some(container(
        scrollable(
            table(
                [
                    table::column("Identity", |(identity, _): RowType| {
                        text(format!("{identity}"))
                    }),
                    table::column("Wins", |(_, perf): RowType| {
                        text(format!("{}", perf.wins()))
                    }),
                    table::column("Draws", |(_, perf): RowType| {
                        text(format!("{}", perf.draws()))
                    }),
                    table::column("Losses", |(_, perf): RowType| {
                        text(format!("{}", perf.losses()))
                    }),
                ],
                matchups,
            )
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ))
}

pub fn view_color_matchups(tournament: &Tournament, id: u32) -> Option<Container<'_, Message>> {
    type RowType<'a> = (MtgColor, MatchPerformance);
    let matchups = tournament
        .get_player_color_match_performance(id)
        .ok()?
        .into_iter()
        .sorted_by_key(|&(_, key)| key)
        .rev();

    Some(container(
        scrollable(
            table(
                [
                    table::column("Identity", |(identity, _): RowType| {
                        text(format!("{identity}"))
                    }),
                    table::column("Wins", |(_, perf): RowType| {
                        text(format!("{}", perf.wins()))
                    }),
                    table::column("Draws", |(_, perf): RowType| {
                        text(format!("{}", perf.draws()))
                    }),
                    table::column("Losses", |(_, perf): RowType| {
                        text(format!("{}", perf.losses()))
                    }),
                ],
                matchups,
            )
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ))
}

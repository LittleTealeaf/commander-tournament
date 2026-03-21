use edh_tourn::{
    analytics::winloss::MatchPerformance,
    game::record::GameRecord,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
        stats::PlayerStats,
    },
    tournament::Tournament,
};
use iced::{
    Element, Length, Padding,
    alignment::{Horizontal, Vertical},
    font,
    widget::{Container, button, column, container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::{style::default_font, views::player_details::Message};

pub fn stats_summary(stats: &PlayerStats) -> Container<'_, Message> {
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

pub fn stats_game_history(
    tournament: &Tournament,
    id: u32,
) -> Option<Container<'_, super::Message>> {
    let column_game = |game: &GameRecord| {
        column(game.players().iter().map(|player| {
            let elo = player.stats().elo().round();
            button(
                text(tournament.get_player_name(&player.id()).map_or_else(
                    || format!("({elo}) {}", player.id()),
                    |name| format!("({elo}) {name}"),
                ))
                .font_maybe((player.id() == game.winner()).then_some(
                    font::Font {
                        weight: font::Weight::Bold,
                        ..default_font()
                    },
                )),
            )
            .padding(Padding::new(0.0))
            .style(button::text)
            .on_press(Message::SelectPlayerReference(player.id()))
            .into()
        }))
    };

    let column_elo = |game: &GameRecord| {
        let elo_change = game.get_player_elo_change(id).unwrap_or_default();
        let elo_change_str = if elo_change >= 0f64 {
            format!("+{}", elo_change.round())
        } else {
            format!("{}", elo_change.round())
        };

        let old_elo = game.get_player(id).map_or_else(
            || tournament.default_stats().elo(),
            |player| player.stats().elo(),
        );

        let new_elo = (old_elo + elo_change).round();

        column![
            text(format!("{new_elo}")).size(20),
            text(elo_change_str).size(15)
        ]
        .spacing(5)
        .padding(5)
    };

    Some(container(table(
        [
            table::column("Games", column_game),
            table::column("Games", column_elo),
        ],
        tournament
            .get_player_games(id)
            .ok()?
            .collect_vec()
            .into_iter()
            .rev(),
    )))
}

fn col_losses<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::Message>> {
    text(format!("{}", performance.losses()))
}

fn col_draws<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::Message>> {
    text(format!("{}", performance.draws()))
}

fn col_wins<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::Message>> {
    text(format!("{}", performance.wins()))
}

fn table_wrapper(table: table::Table<'_, super::Message>) -> Container<'_, super::Message> {
    container(
        scrollable(table.width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
}

pub fn stats_player_matchups(
    tournament: &Tournament,
    id: u32,
) -> Option<Container<'_, super::Message>> {
    type RowType<'a> = (RegisteredPlayer<'a>, MatchPerformance);

    let matchups = tournament
        .get_player_player_match_performance(id)
        .ok()?
        .sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            perf_a
                .cmp(perf_b)
                .then_with(|| player_a.stats().elo().total_cmp(&player_b.stats().elo()))
        })
        .rev();

    let col_player = |(player, _): RowType| {
        button(text(player.info().name().to_owned()))
            .style(button::text)
            .padding(Padding::new(0.0))
            .on_press(super::Message::SelectPlayerReference(player.id()))
    };

    let col_identity = |(player, _): RowType| text(player.info().color_identity().to_string());

    Some(table_wrapper(table(
        [
            table::column("Player", col_player),
            table::column("Identity", col_identity),
            table::column("Wins", col_wins),
            table::column("Draws", col_draws),
            table::column("Losses", col_losses),
        ],
        matchups,
    )))
}

pub fn stats_identity_matchups(
    tournament: &Tournament,
    id: u32,
) -> Option<Container<'_, super::Message>> {
    type RowType = (ColorIdentity, MatchPerformance);
    let matchups = tournament
        .get_player_identity_match_performance(id)
        .ok()?
        .into_iter()
        .sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            perf_a
                .cmp(perf_b)
                .reverse()
                .then_with(|| player_a.cmp(player_b))
        });

    let col_identity = |(identity, _): RowType| text(format!("{identity}"));
    let col_colors =
        |(identity, _): RowType| text(identity.colors().map(MtgColor::letter).join(""));

    Some(table_wrapper(table(
        [
            table::column("Color Identity", col_identity),
            table::column("Colors", col_colors),
            table::column("Wins", col_wins),
            table::column("Draws", col_draws),
            table::column("Losses", col_losses),
        ],
        matchups,
    )))
}

pub fn stats_color_matchups(
    tournament: &Tournament,
    id: u32,
) -> Option<Container<'_, super::Message>> {
    type RowType = (MtgColor, MatchPerformance);
    let matchups = tournament
        .get_player_color_match_performance(id)
        .ok()?
        .into_iter()
        .sorted_by(|(color_a, perf_a), (color_b, perf_b)| {
            perf_a
                .cmp(perf_b)
                .reverse()
                .then_with(|| color_a.cmp(color_b))
        });

    let col_color = |(color, _): RowType| text(format!("{color}"));

    Some(table_wrapper(table(
        [
            table::column("Color", col_color),
            table::column("Wins", col_wins),
            table::column("Draws", col_draws),
            table::column("Losses", col_losses),
        ],
        matchups,
    )))
}

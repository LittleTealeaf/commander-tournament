use edh_tourn::{
    analytics::winloss::MatchPerformance,
    game::record::GameRecord,
    player::{
        PlayerId, RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
        stats::PlayerStats,
    },
    tournament::Tournament,
};
use iced::{
    Element, Length, Padding,
    alignment::{Horizontal, Vertical},
    widget::{Container, button, column, container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::{fonts::FONT_BOLD, views::player::PlayerDetailsMsg};

pub fn stats_summary(stats: &PlayerStats) -> Container<'_, PlayerDetailsMsg> {
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

fn col_losses<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::PlayerDetailsMsg>> {
    text(format!("{}", performance.losses()))
}

fn col_draws<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::PlayerDetailsMsg>> {
    text(format!("{}", performance.draws()))
}

fn col_wins<'a, T: 'a>(
    (_, performance): (T, MatchPerformance),
) -> impl Into<Element<'a, super::PlayerDetailsMsg>> {
    text(format!("{}", performance.wins()))
}

fn table_wrapper(
    table: table::Table<'_, super::PlayerDetailsMsg>,
) -> Container<'_, super::PlayerDetailsMsg> {
    container(
        scrollable(table.width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
}

pub fn stats_game_history(
    tournament: &Tournament,
    id: PlayerId,
) -> Option<Container<'_, super::PlayerDetailsMsg>> {
    let column_game = |game: &GameRecord| {
        column(game.players().iter().map(|player| {
            let elo = player.stats().elo().round();
            button(
                text(
                    tournament
                        .get_player_display_name(&player.id())
                        .map_or_else(
                            || format!("({elo}) {}", player.id()),
                            |name| format!("({elo}) {name}"),
                        ),
                )
                .font_maybe((player.id() == game.winner()).then_some(FONT_BOLD)),
            )
            .padding(Padding::new(0.0))
            .style(button::text)
            .on_press(PlayerDetailsMsg::SelectPlayerReference(player.id()))
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

    let games = tournament
        .get_player_games(id)
        .ok()?
        .collect_vec()
        .into_iter()
        .rev();

    Some(table_wrapper(table(
        [
            table::column("Games", column_game),
            table::column("Elo Change", column_elo),
        ],
        games,
    )))
}

pub fn stats_player_matchups(
    tournament: &Tournament,
    id: PlayerId,
) -> Option<Container<'_, super::PlayerDetailsMsg>> {
    type RowType<'a> = (RegisteredPlayer<'a>, MatchPerformance);

    let matchups = tournament
        .analytics()
        .player_vs_player_performance(id)
        .ok()?
        .sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            perf_a
                .cmp(perf_b)
                .then_with(|| player_a.stats().elo().total_cmp(&player_b.stats().elo()))
        })
        .rev();

    let col_player = |(player, _): RowType| {
        button(text(format!(
            "({}) {}",
            player.stats().elo().round(),
            player.info().display_name()
        )))
        .style(button::text)
        .padding(Padding::new(0.0))
        .on_press(super::PlayerDetailsMsg::SelectPlayerReference(player.id()))
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
    id: PlayerId,
) -> Option<Container<'_, super::PlayerDetailsMsg>> {
    type RowType = (ColorIdentity, MatchPerformance);
    let matchups = tournament
        .analytics()
        .player_vs_identity_performance(id)
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
    id: PlayerId,
) -> Option<Container<'_, super::PlayerDetailsMsg>> {
    type RowType = (MtgColor, MatchPerformance);
    let matchups = tournament
        .analytics()
        .player_vs_color_performance(id)
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

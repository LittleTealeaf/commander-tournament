use commander_tournament_core::{
    analytics::aggregate::AggregateStats,
    player::color::{ColorIdentity, MtgColor},
    tournament::Tournament,
};
use iced::{
    Length,
    alignment::Vertical,
    widget::{column, container, row, text},
};

use crate::{
    components::{mana_pips, progress_bar},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_SEMIBOLD},
    icons::color_icon,
    style::{self, COLORS, color_accent},
};

use super::{OverviewMsg, OverviewState};

pub(super) fn render_analytics<'a>(
    state: &OverviewState,
    tournament: &Tournament,
) -> iced::Element<'a, OverviewMsg> {
    let include_archived = state.include_archived;
    let include_precons = state.include_precons;

    let color_stats = tournament
        .analytics()
        .filter_precons(include_precons)
        .filter_pool(move |id| {
            include_archived
                || tournament
                    .get_player_info(&id)
                    .is_some_and(|info| !info.is_archived())
        })
        .aggregated_color_stats();

    let identity_stats = tournament
        .analytics()
        .filter_precons(include_precons)
        .filter_pool(move |id| {
            include_archived
                || tournament
                    .get_player_info(&id)
                    .is_some_and(|info| !info.is_archived())
        })
        .aggregated_identity_stats();

    let seat_card = render_seat_advantage(tournament);
    let color_card = render_color_stats(&color_stats);
    let identity_card = render_identity_stats(&identity_stats);

    column![
        row![color_card, seat_card]
            .spacing(16)
            .width(Length::Fill)
            .height(Length::Fixed(196.0)),
        identity_card,
    ]
    .spacing(16)
    .width(Length::Fill)
    .into()
}

fn render_seat_advantage<'a>(tournament: &Tournament) -> iced::Element<'a, OverviewMsg> {
    let mut seat_wins = [0_u32; 4];
    let total_games = tournament.games().len();

    for game in tournament.games() {
        if let Some(seat_idx) = game.ids().iter().position(|&id| id == game.winner())
            && let Some(slot) = seat_wins.get_mut(seat_idx)
        {
            *slot += 1;
        }
    }

    let mut col = column![
        text("Turn-Order Seat 1–4 Win Rate")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(10)
    .width(Length::Fill);

    for seat_idx in 0..4 {
        let wins = seat_wins.get(seat_idx).copied().unwrap_or(0);
        let ratio = if total_games > 0 {
            #[allow(clippy::cast_precision_loss, reason = "Game count fits in f64")]
            {
                f64::from(wins) / total_games as f64
            }
        } else {
            0.0
        };
        let accent = if ratio >= 0.28 {
            COLORS.green
        } else if ratio >= 0.22 {
            COLORS.blue
        } else {
            COLORS.peach
        };
        let pct_label = if total_games > 0 {
            format!("{:.1}%", ratio * 100.0)
        } else {
            "—".to_owned()
        };

        col = col.push(
            row![
                text(format!("Seat {}", seat_idx + 1))
                    .font(FONT_MEDIUM)
                    .size(13)
                    .color(COLORS.subtext1)
                    .width(Length::Fixed(64.0)),
                progress_bar(ratio, accent),
                text(pct_label)
                    .font(FONT_SEMIBOLD)
                    .size(12)
                    .color(COLORS.text)
                    .width(Length::Fixed(52.0)),
                text(format!("{wins}W"))
                    .size(11)
                    .color(COLORS.subtext0)
                    .width(Length::Fixed(48.0)),
            ]
            .spacing(10)
            .align_y(Vertical::Center)
            .width(Length::Fill),
        );
    }

    container(col)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn render_color_stats<'a>(
    stats_map: &std::collections::HashMap<MtgColor, AggregateStats>,
) -> iced::Element<'a, OverviewMsg> {
    let mut col = column![
        text("Win Rate & Avg Elo by Color")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(10)
    .width(Length::Fill);

    for color in MtgColor::COLORS {
        let agg = stats_map.get(&color).cloned().unwrap_or_default();
        let wr = agg.wr().unwrap_or(0.0);
        let wr_label = agg
            .wr()
            .map_or_else(|| "—".to_owned(), |v| format!("{:.1}%", v * 100.0));
        let elo_label = agg
            .avg_elo()
            .map_or_else(|| "—".to_owned(), |e| format!("{e:.0}"));
        let accent = color_accent(color);

        col = col.push(
            row![
                color_icon(color),
                text(color.to_string())
                    .size(12)
                    .color(COLORS.subtext1)
                    .width(Length::Fixed(52.0)),
                progress_bar(wr, accent),
                text(wr_label)
                    .font(FONT_SEMIBOLD)
                    .size(12)
                    .color(COLORS.text)
                    .width(Length::Fixed(48.0)),
                text(format!("{elo_label} Elo ({})", agg.count()))
                    .size(11)
                    .color(COLORS.subtext0)
                    .width(Length::Fixed(96.0)),
            ]
            .spacing(8)
            .align_y(Vertical::Center)
            .width(Length::Fill),
        );
    }

    container(col)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn render_identity_stats<'a>(
    stats_map: &std::collections::HashMap<ColorIdentity, AggregateStats>,
) -> iced::Element<'a, OverviewMsg> {
    let mut entries: Vec<(ColorIdentity, AggregateStats)> =
        stats_map.iter().map(|(id, agg)| (*id, agg.clone())).collect();

    entries.sort_by(|a, b| {
        b.1.avg_elo()
            .unwrap_or(0.0)
            .total_cmp(&a.1.avg_elo().unwrap_or(0.0))
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut col = column![
        text("Performance by Color Identity")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(8)
    .width(Length::Fill);

    if entries.is_empty() {
        col = col.push(
            text("No color identity stats available yet.")
                .size(12)
                .color(COLORS.subtext0),
        );
    } else {
        for (identity, agg) in entries {
            let wr = agg.wr().unwrap_or(0.0);
            let wr_label = agg
                .wr()
                .map_or_else(|| "—".to_owned(), |v| format!("{:.1}%", v * 100.0));
            let elo_label = agg
                .avg_elo()
                .map_or_else(|| "—".to_owned(), |e| format!("{e:.1}"));
            let pips = mana_pips(identity);

            col = col.push(
                container(
                    row![
                        text(identity.to_string())
                            .font(FONT_MEDIUM)
                            .size(13)
                            .color(COLORS.text)
                            .width(Length::Fixed(96.0)),
                        container(pips).width(Length::Fixed(96.0)),
                        progress_bar(wr, COLORS.blue),
                        text(wr_label)
                            .font(FONT_SEMIBOLD)
                            .size(12)
                            .color(COLORS.text)
                            .width(Length::Fixed(52.0)),
                        text(format!("{elo_label} Elo"))
                            .size(12)
                            .color(COLORS.subtext1)
                            .width(Length::Fixed(80.0)),
                        text(format!("{} decks · {}G", agg.count(), agg.games()))
                            .size(11)
                            .color(COLORS.subtext0)
                            .width(Length::Fixed(108.0)),
                    ]
                    .spacing(10)
                    .align_y(Vertical::Center)
                    .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding([8, 12])
                .style(style::table_row(false)),
            );
        }
    }

    container(col)
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

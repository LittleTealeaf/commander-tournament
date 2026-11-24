use core::cmp::Reverse;

use commander_tournament_core::{
    analytics::winloss::MatchPerformance,
    game::record::GameRecord,
    player::{PlayerId, RegisteredPlayer, color::MtgColor},
    tournament::Tournament,
};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, rule, space, text},
};

use crate::{
    components::{kpi_card, progress_bar},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_SEMIBOLD},
    icons::color_icon,
    style::{self, COLORS, color_accent},
};

use super::super::DecksMsg;
use super::matches::render_recent_matches_card;

pub(super) fn render_overview_tab<'a>(
    tournament: &'a Tournament,
    player: RegisteredPlayer<'a>,
    seat_games: &[u32; 4],
    seat_wins: &[u32; 4],
    deck_games: &[(usize, &'a GameRecord)],
) -> Element<'a, DecksMsg> {
    let id = player.id();
    let stats = player.stats();

    let mut elo_ranking: Vec<(PlayerId, f64)> = tournament
        .registered_players()
        .map(|p| (p.id(), p.stats().elo()))
        .collect();
    elo_ranking.sort_by(|a, b| b.1.total_cmp(&a.1));
    let rank = elo_ranking
        .iter()
        .position(|(pid, _)| *pid == id)
        .map_or(0, |idx| idx + 1);
    let total_decks = elo_ranking.len();

    let wr_display = stats
        .wr()
        .map_or_else(|| "—".to_owned(), |wr| format!("{:.1}%", wr * 100.0));

    let best_seat_text = seat_games
        .iter()
        .zip(seat_wins.iter())
        .enumerate()
        .filter(|(_, (g, _))| **g > 0)
        .max_by(|(_, (g1, w1)), (_, (g2, w2))| {
            let r1 = f64::from(**w1) / f64::from(**g1);
            let r2 = f64::from(**w2) / f64::from(**g2);
            r1.total_cmp(&r2)
        })
        .map_or_else(
            || ("—".to_owned(), "No seat data".to_owned()),
            |(idx, (g, w))| {
                let pct = (f64::from(*w) / f64::from(*g)) * 100.0;
                (format!("Seat {}", idx + 1), format!("{pct:.1}% ({w}W / {g}G)"))
            },
        );

    let kpi_strip = row![
        kpi_card(
            "Current Elo",
            format!("{:.1}", stats.elo()),
            format!("Peak {:.1}", stats.elo_peak()),
            COLORS.blue,
        ),
        kpi_card(
            "1st-Place Win Rate",
            wr_display,
            format!("{}W / {}G", stats.wins(), stats.games()),
            COLORS.green,
        ),
        kpi_card(
            "Leaderboard Rank",
            if rank > 0 {
                format!("#{rank}")
            } else {
                "—".to_owned()
            },
            format!("of {total_decks} decks"),
            COLORS.mauve,
        ),
        kpi_card("Best Seat", best_seat_text.0, best_seat_text.1, COLORS.sapphire),
    ]
    .spacing(12)
    .width(Length::Fill);

    column![
        kpi_strip,
        row![
            render_seat_breakdown(seat_games, seat_wins),
            render_color_matchups(tournament, id),
        ]
        .spacing(16)
        .width(Length::Fill)
        .height(Length::Fixed(196.0)),
        render_notable_opponents(tournament, id),
        render_recent_matches_card(tournament, id, deck_games, 8),
    ]
    .spacing(16)
    .width(Length::Fill)
    .into()
}

fn render_seat_breakdown<'a>(seat_games: &[u32; 4], seat_wins: &[u32; 4]) -> Element<'a, DecksMsg> {
    let mut rows = column![
        text("Turn-Order Seat Performance")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(10)
    .width(Length::Fill);

    for seat_idx in 0..4 {
        let g = seat_games.get(seat_idx).copied().unwrap_or(0);
        let w = seat_wins.get(seat_idx).copied().unwrap_or(0);
        let ratio = if g > 0 { f64::from(w) / f64::from(g) } else { 0.0 };
        let accent = if ratio >= 0.30 {
            COLORS.green
        } else if ratio >= 0.20 {
            COLORS.blue
        } else {
            COLORS.peach
        };
        let pct_label = if g > 0 {
            format!("{:.1}%", ratio * 100.0)
        } else {
            "—".to_owned()
        };

        rows = rows.push(
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
                text(format!("{w}W / {g}G"))
                    .size(11)
                    .color(COLORS.subtext0)
                    .width(Length::Fixed(68.0)),
            ]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill),
        );
    }

    container(rows)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn render_color_matchups<'a>(tournament: &Tournament, id: PlayerId) -> Element<'a, DecksMsg> {
    let color_perf = tournament
        .analytics()
        .player_vs_color_performance(id)
        .unwrap_or_default();

    let mut col = column![
        text("Matchup Win Rate vs. Colors")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(10)
    .width(Length::Fill);

    for color in MtgColor::COLORS {
        let perf = color_perf.get(&color).copied().unwrap_or_default();
        let wr = perf.winrate();
        let ratio = wr.unwrap_or(0.0);
        let pct_label = wr.map_or_else(|| "—".to_owned(), |v| format!("{:.1}%", v * 100.0));

        col = col.push(
            row![
                color_icon(color),
                text(color.to_string())
                    .size(12)
                    .color(COLORS.subtext1)
                    .width(Length::Fixed(52.0)),
                progress_bar(ratio, color_accent(color)),
                text(pct_label)
                    .font(FONT_SEMIBOLD)
                    .size(12)
                    .color(COLORS.text)
                    .width(Length::Fixed(48.0)),
                text(format!("{}-{}-{}", perf.wins(), perf.losses(), perf.draws()))
                    .size(11)
                    .color(COLORS.subtext0)
                    .width(Length::Fixed(56.0)),
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

fn render_notable_opponents<'a>(tournament: &'a Tournament, id: PlayerId) -> Element<'a, DecksMsg> {
    let mut matchups: Vec<(RegisteredPlayer<'a>, MatchPerformance)> = tournament
        .analytics()
        .player_vs_player_performance(id)
        .map_or_default(Iterator::collect);

    let mut col = column![
        text("Notable Opponent Matchups")
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
    ]
    .spacing(10)
    .width(Length::Fill);

    if matchups.is_empty() {
        col = col.push(
            text("No opponent matchup data yet.")
                .size(12)
                .color(COLORS.subtext0),
        );
    } else {
        matchups.sort_by_key(|(p, perf)| (Reverse(*perf), p.id()));

        let best: Vec<_> = matchups.iter().take(4).copied().collect();
        let worst: Vec<_> = matchups
            .iter()
            .rev()
            .take(4)
            .filter(|(p, _)| !best.iter().any(|(bp, _)| bp.id() == p.id()))
            .copied()
            .collect();

        let mut best_col = column![
            text("Best Matchups (W-L-D)")
                .font(FONT_SEMIBOLD)
                .size(12)
                .color(COLORS.green),
        ]
        .spacing(6)
        .width(Length::FillPortion(1));

        for (opp, perf) in best {
            best_col = best_col.push(opponent_matchup_row(opp, perf, COLORS.green));
        }

        let mut split = row![best_col].spacing(16).width(Length::Fill);

        if !worst.is_empty() {
            let mut worst_col = column![
                text("Toughest Matchups (W-L-D)")
                    .font(FONT_SEMIBOLD)
                    .size(12)
                    .color(COLORS.red),
            ]
            .spacing(6)
            .width(Length::FillPortion(1));

            for (opp, perf) in worst {
                worst_col = worst_col.push(opponent_matchup_row(opp, perf, COLORS.red));
            }

            split = split.push(rule::vertical(1)).push(worst_col);
        }

        col = col.push(split);
    }

    container(col)
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn opponent_matchup_row(
    opp: RegisteredPlayer<'_>,
    perf: MatchPerformance,
    accent: catppuccin::Color,
) -> Element<'_, DecksMsg> {
    row![
        button(text(opp.info().display_name()).size(12).color(COLORS.text))
            .padding([2, 4])
            .style(style::icon_button)
            .on_press(DecksMsg::OpenDetail(opp.id())),
        space::horizontal(),
        text(format!("{}-{}-{}", perf.wins(), perf.losses(), perf.draws()))
            .font(FONT_MEDIUM)
            .size(12)
            .color(accent),
    ]
    .spacing(8)
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .into()
}

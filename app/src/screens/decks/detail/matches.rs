use commander_tournament_core::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, space, text},
};
use iced_tea::Component;

use crate::{
    components::{Paginator, pill},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_NORMAL, FONT_SEMIBOLD},
    style::{self, COLORS},
};

use super::super::DecksMsg;

pub(super) fn render_matches_tab<'a>(
    tournament: &'a Tournament,
    id: PlayerId,
    deck_games: &[(usize, &'a GameRecord)],
    paginator: &'a Paginator,
) -> Element<'a, DecksMsg> {
    if deck_games.is_empty() {
        return container(
            text("This deck hasn't played any matches yet.")
                .size(13)
                .color(COLORS.subtext0),
        )
        .width(Length::Fill)
        .padding(24)
        .style(style::card)
        .into();
    }

    let total = deck_games.len();
    let (skip, take) = paginator.slice_bounds(total);

    let rows = deck_games
        .iter()
        .rev()
        .skip(skip)
        .take(take)
        .fold(column![].spacing(6), |col, &(gid, record)| {
            col.push(render_deck_match_row(tournament, id, gid, record))
        })
        .width(Length::Fill);

    column![rows, paginator.view(total).map(DecksMsg::DetailPaginator)]
        .spacing(12)
        .width(Length::Fill)
        .into()
}

pub(super) fn render_recent_matches_card<'a>(
    tournament: &'a Tournament,
    id: PlayerId,
    deck_games: &[(usize, &'a GameRecord)],
    limit: usize,
) -> Element<'a, DecksMsg> {
    let mut col = column![
        row![
            text("Recent Matches").font(FONT_BOLD).size(15).color(COLORS.text),
            space::horizontal(),
            text(format!("{} total games", deck_games.len()))
                .size(12)
                .color(COLORS.subtext0),
        ]
        .align_y(Vertical::Center)
        .width(Length::Fill),
    ]
    .spacing(8)
    .width(Length::Fill);

    if deck_games.is_empty() {
        col = col.push(
            text("This deck hasn't played any matches yet.")
                .size(12)
                .color(COLORS.subtext0),
        );
    } else {
        for &(gid, record) in deck_games.iter().rev().take(limit) {
            col = col.push(render_deck_match_row(tournament, id, gid, record));
        }
    }

    container(col)
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn render_deck_match_row<'a>(
    tournament: &'a Tournament,
    id: PlayerId,
    gid: usize,
    record: &GameRecord,
) -> Element<'a, DecksMsg> {
    let won = record.winner() == id;
    let delta = record.get_player_elo_change(id).unwrap_or(0.0);
    let old_elo = record
        .get_player(id)
        .map_or_else(|| tournament.default_stats().elo(), |p| p.stats().elo());
    let new_elo = old_elo + delta;

    let seat = record
        .ids()
        .iter()
        .position(|&pid| pid == id)
        .map_or(0, |idx| idx + 1);

    let opponents = record
        .ids()
        .into_iter()
        .filter(|&pid| pid != id)
        .fold(row![].spacing(8).align_y(Vertical::Center), |r, pid| {
            let name = tournament
                .get_player_display_name(&pid)
                .unwrap_or_else(|| format!("Deck {pid:?}"));
            let is_winner = pid == record.winner();
            r.push(
                button(
                    text(name)
                        .font(if is_winner { FONT_BOLD } else { FONT_NORMAL })
                        .size(12)
                        .color(if is_winner { COLORS.text } else { COLORS.subtext1 }),
                )
                .padding([1, 4])
                .style(style::icon_button)
                .on_press(DecksMsg::OpenDetail(pid)),
            )
        })
        .wrap();

    let (result_label, result_color, delta_text) = if won {
        ("WIN", COLORS.green, format!("+{delta:.1}"))
    } else {
        ("LOSS", COLORS.red, format!("{delta:.1}"))
    };

    container(
        row![
            text(format!("#{}", gid + 1))
                .font(FONT_SEMIBOLD)
                .size(12)
                .color(COLORS.subtext0)
                .width(Length::Fixed(44.0)),
            pill(result_label, result_color),
            text(format!("{new_elo:.0} ({delta_text})"))
                .font(FONT_MEDIUM)
                .size(12)
                .color(result_color)
                .width(Length::Fixed(96.0)),
            text(format!("Seat {seat}"))
                .size(11)
                .color(COLORS.subtext0)
                .width(Length::Fixed(48.0)),
            opponents,
        ]
        .spacing(10)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([8, 12])
    .style(style::table_row(false))
    .into()
}

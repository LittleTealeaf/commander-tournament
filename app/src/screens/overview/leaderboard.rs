use commander_tournament_core::{player::RegisteredPlayer, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, pick_list, row, space, text, text_input},
};
use iced_tea::Component;
use nerd_font_symbols::md;

use crate::{
    components::{empty_card, mana_pips, pill},
    fonts::{FONT_BOLD, FONT_SEMIBOLD},
    style::{self, COLORS, bracket_accent},
};

use super::{OverviewBracketFilter, OverviewMsg, OverviewState};

pub(super) fn render_leaderboard<'a>(
    state: &'a OverviewState,
    tournament: &'a Tournament,
) -> iced::Element<'a, OverviewMsg> {
    if tournament.games().is_empty() && tournament.players().is_empty() {
        return empty_card(
            md::MD_TROPHY_OUTLINE,
            "No goldfish matches recorded yet",
            "Add decks and record pods to populate the Plackett-Luce leaderboard.",
            Some(
                button(text("Go to Play").font(FONT_BOLD).size(13))
                    .padding([8, 14])
                    .style(style::primary_button)
                    .on_press(OverviewMsg::GoToPlay)
                    .into(),
            ),
        );
    }

    let search_lower = state.search.trim().to_lowercase();

    let mut ranked: Vec<RegisteredPlayer<'_>> = tournament
        .registered_players()
        .filter(|p| {
            let info = p.info();
            if !state.include_archived && info.is_archived() {
                return false;
            }
            if !state.include_precons && info.is_precon() {
                return false;
            }
            if let OverviewBracketFilter::Bracket(b) = state.bracket_filter
                && info.bracket() != Some(b)
            {
                return false;
            }
            if !search_lower.is_empty() {
                let name_ok = info.display_name().to_lowercase().contains(&search_lower);
                let desc_ok = info.description().to_lowercase().contains(&search_lower);
                if !name_ok && !desc_ok {
                    return false;
                }
            }
            true
        })
        .collect();

    ranked.sort_by(|a, b| {
        b.stats()
            .elo()
            .total_cmp(&a.stats().elo())
            .then_with(|| a.info().name().cmp(b.info().name()))
    });

    let filter_bar = row![
        text_input("Search leaderboard...", &state.search)
            .on_input(OverviewMsg::SearchChanged)
            .size(13)
            .padding([8, 12])
            .width(Length::Fixed(260.0))
            .style(style::search_input),
        pick_list(
            OverviewBracketFilter::all_options(),
            Some(state.bracket_filter),
            OverviewMsg::BracketFilterChanged,
        )
        .text_size(13)
        .width(Length::Fixed(180.0))
        .padding([8, 12]),
        space::horizontal(),
        pill(format!("{} ranked decks", ranked.len()), COLORS.sapphire),
    ]
    .spacing(10)
    .align_y(Vertical::Center)
    .width(Length::Fill);

    if ranked.is_empty() {
        return column![
            filter_bar,
            container(
                text("No decks match the current leaderboard filters.")
                    .size(13)
                    .color(COLORS.subtext0),
            )
            .width(Length::Fill)
            .padding(24)
            .style(style::card),
        ]
        .spacing(12)
        .width(Length::Fill)
        .into();
    }

    let total = ranked.len();
    let (skip, take) = state.paginator.slice_bounds(total);

    let header = row![
        text("Rank")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(48.0)),
        text("Deck")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fill),
        text("Colors")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(96.0)),
        text("Elo")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(72.0)),
        text("Win Rate")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(76.0)),
        text("Games")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(56.0)),
    ]
    .spacing(12)
    .padding([0, 16]);

    let rows = ranked
        .into_iter()
        .enumerate()
        .skip(skip)
        .take(take)
        .fold(column![header].spacing(6), |col, (idx, player)| {
            col.push(render_leaderboard_row(idx + 1, player))
        })
        .width(Length::Fill);

    let mut content = column![filter_bar, rows].spacing(12).width(Length::Fill);
    if state.paginator.total_pages(total) > 1 {
        let pager: Element<'a, OverviewMsg> = state.paginator.view_into(total);
        content = content.push(pager);
    }
    content.into()
}

fn render_leaderboard_row(rank: usize, player: RegisteredPlayer<'_>) -> iced::Element<'_, OverviewMsg> {
    let info = player.info();
    let stats = player.stats();

    let rank_color = match rank {
        1 => COLORS.yellow,
        2 => COLORS.subtext1,
        3 => COLORS.peach,
        _ => COLORS.subtext0,
    };

    let pips = mana_pips(info.color_identity());

    let mut name_row = row![
        button(
            text(info.name().clone())
                .font(FONT_BOLD)
                .size(14)
                .color(COLORS.text)
        )
        .padding([2, 4])
        .style(style::icon_button)
        .on_press(OverviewMsg::InspectDeck(player.id())),
    ]
    .spacing(8)
    .align_y(Vertical::Center);

    if let Some(bracket) = info.bracket() {
        name_row = name_row.push(pill(bracket.short_label(), bracket_accent(bracket)));
    }
    if info.is_precon() {
        name_row = name_row.push(pill("Precon", COLORS.teal));
    }

    let wr_text = stats
        .wr()
        .map_or_else(|| "—".to_owned(), |wr| format!("{:.1}%", wr * 100.0));

    container(
        row![
            text(format!("#{rank}"))
                .font(FONT_BOLD)
                .size(14)
                .color(rank_color)
                .width(Length::Fixed(48.0)),
            container(name_row.wrap()).width(Length::Fill),
            container(pips).width(Length::Fixed(96.0)),
            column![
                text(format!("{:.1}", stats.elo()))
                    .font(FONT_BOLD)
                    .size(14)
                    .color(COLORS.text),
                text(format!("Peak {:.0}", stats.elo_peak()))
                    .size(10)
                    .color(COLORS.subtext0),
            ]
            .width(Length::Fixed(72.0)),
            text(wr_text)
                .font(FONT_SEMIBOLD)
                .size(13)
                .color(COLORS.subtext1)
                .width(Length::Fixed(76.0)),
            text(stats.games().to_string())
                .size(13)
                .color(COLORS.subtext0)
                .width(Length::Fixed(56.0)),
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([10, 16])
    .style(style::table_row(false))
    .into()
}

use commander_tournament_core::{
    player::{RegisteredPlayer, color::MtgColor},
    tournament::Tournament,
};
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{button, checkbox, column, container, pick_list, row, space, text, text_input},
};
use iced_tea::Component;
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{empty_card, mana_pips, pill},
    fonts::{FONT_BOLD, FONT_SEMIBOLD},
    icons::{color_icon, colorless_icon},
    style::{self, COLORS, bracket_accent},
};

use super::{BracketFilter, ColorFilterMode, DeckSortField, DecksMsg, DecksState, editor::DeckEditor};

pub(super) fn render_decks_list<'a>(
    state: &'a DecksState,
    tournament: &'a Tournament,
) -> iced::Element<'a, DecksMsg> {
    let search_lower = state.search.trim().to_lowercase();
    let total_decks = tournament.players().len();

    let mut players: Vec<RegisteredPlayer<'_>> = tournament
        .registered_players()
        .filter(|player| matches_filters(state, player, &search_lower))
        .collect();

    sort_players(&mut players, state.sort_by, state.sort_asc);

    let toolbar = render_toolbar(state, players.len(), total_decks);

    let mut layout = column![toolbar].spacing(16).width(Length::Fill);

    let total_filtered = players.len();
    let (skip, take) = state.paginator.slice_bounds(total_filtered);

    let editing_id = state.editor.as_ref().and_then(DeckEditor::editing_id);
    let editor_rendered_inline = editing_id.is_some_and(|id| {
        players
            .iter()
            .skip(skip)
            .take(take)
            .any(|player| player.id() == id)
    });

    if let Some(editor) = &state.editor
        && !editor_rendered_inline
    {
        layout = layout.push(editor.view_into(tournament));
    }

    let body: iced::Element<'a, DecksMsg> = if total_decks == 0 {
        empty_card(
            md::MD_CARDS_OUTLINE,
            "No decks registered yet",
            "Add your first Commander deck to start tracking ratings and generating pods.",
            Some(
                button(
                    row![
                        text(md::MD_PLUS).size(14),
                        text("Add Deck").font(FONT_BOLD).size(13),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Center),
                )
                .padding([8, 14])
                .style(style::primary_button)
                .on_press(DecksMsg::OpenAddDeck)
                .into(),
            ),
        )
    } else if players.is_empty() {
        empty_card(
            md::MD_MAGNIFY,
            "No matching decks found",
            "Try clearing or adjusting your search, color, bracket, or archive filters.",
            Some(
                button(text("Clear Filters").size(13))
                    .padding([8, 14])
                    .style(style::secondary_button)
                    .on_press(DecksMsg::ClearFilters)
                    .into(),
            ),
        )
    } else {
        let rows = players
            .into_iter()
            .skip(skip)
            .take(take)
            .fold(column![].spacing(8), |mut col, player| {
                let is_editing = editing_id == Some(player.id());
                if is_editing && let Some(editor) = &state.editor {
                    col = col.push(editor.view_into(tournament));
                }
                col.push(render_deck_row(player, is_editing))
            })
            .width(Length::Fill);

        let pager: Element<'a, DecksMsg> = state.paginator.view_into(total_filtered);

        column![rows, pager].spacing(16).width(Length::Fill).into()
    };

    layout.push(body).into()
}

fn matches_filters(state: &DecksState, player: &RegisteredPlayer<'_>, search_lower: &str) -> bool {
    let info = player.info();

    if !state.show_archived && info.is_archived() {
        return false;
    }

    if state.precons_only && !info.is_precon() {
        return false;
    }

    match state.bracket_filter {
        BracketFilter::All => {}
        BracketFilter::Unassigned => {
            if info.bracket().is_some() {
                return false;
            }
        }
        BracketFilter::Bracket(target) => {
            if info.bracket() != Some(target) {
                return false;
            }
        }
    }

    match state.color_mode {
        ColorFilterMode::ColorlessOnly => {
            if !info.color_identity().is_colorless() {
                return false;
            }
        }
        ColorFilterMode::Exact => {
            if !state.color_filter.is_colorless() && info.color_identity() != state.color_filter {
                return false;
            }
        }
        ColorFilterMode::Includes => {
            if !state.color_filter.is_colorless()
                && (info.color_identity() & state.color_filter) != state.color_filter
            {
                return false;
            }
        }
    }

    if !search_lower.is_empty() {
        let name_match = info.display_name().to_lowercase().contains(search_lower);
        let desc_match = info.description().to_lowercase().contains(search_lower);
        if !name_match && !desc_match {
            return false;
        }
    }

    true
}

fn sort_players(players: &mut [RegisteredPlayer<'_>], field: DeckSortField, asc: bool) {
    players.sort_by(|a, b| {
        let ord = match field {
            DeckSortField::Name => a
                .info()
                .display_name()
                .to_lowercase()
                .cmp(&b.info().display_name().to_lowercase()),
            DeckSortField::Elo => a.stats().elo().total_cmp(&b.stats().elo()),
            DeckSortField::WinRate => a.stats().wr_unwrap().total_cmp(&b.stats().wr_unwrap()),
            DeckSortField::Games => a.stats().games().cmp(&b.stats().games()),
            DeckSortField::Bracket => a.info().bracket().cmp(&b.info().bracket()),
        };
        let ord = if asc { ord } else { ord.reverse() };
        ord.then_with(|| a.info().name().cmp(b.info().name()))
    });
}

fn render_toolbar(state: &DecksState, shown_count: usize, total_count: usize) -> iced::Element<'_, DecksMsg> {
    let search_box = text_input("Search decks or descriptions...", &state.search)
        .on_input(DecksMsg::SearchChanged)
        .size(13)
        .padding([8, 12])
        .width(Length::Fixed(260.0))
        .style(style::search_input);

    let bracket_picker = pick_list(
        BracketFilter::all_options(),
        Some(state.bracket_filter),
        DecksMsg::BracketFilterChanged,
    )
    .text_size(13)
    .width(Length::Fixed(180.0))
    .padding([8, 12]);

    let sort_picker = pick_list(
        DeckSortField::VARIANTS,
        Some(state.sort_by),
        DecksMsg::SortFieldChanged,
    )
    .text_size(13)
    .width(Length::Fixed(160.0))
    .padding([8, 12]);

    let sort_dir_icon = if state.sort_asc {
        md::MD_SORT_ASCENDING
    } else {
        md::MD_SORT_DESCENDING
    };
    let sort_dir_btn = button(text(sort_dir_icon).size(15))
        .padding([8, 10])
        .style(style::secondary_button)
        .on_press(DecksMsg::ToggleSortDirection);

    let add_btn = button(
        row![
            text(md::MD_PLUS).size(14),
            text("Add Deck").font(FONT_BOLD).size(13),
        ]
        .spacing(6)
        .align_y(Vertical::Center),
    )
    .padding([8, 14])
    .style(style::primary_button)
    .on_press(DecksMsg::OpenAddDeck);

    let top_row = row![
        search_box,
        bracket_picker,
        sort_picker,
        sort_dir_btn,
        space::horizontal(),
        add_btn,
    ]
    .spacing(8)
    .align_y(Vertical::Center)
    .width(Length::Fill);

    let colorless_only = state.color_mode == ColorFilterMode::ColorlessOnly;
    let exact_match = state.color_mode == ColorFilterMode::Exact;

    let color_buttons = MtgColor::COLORS.into_iter().fold(
        row![text("Colors:").size(12).color(COLORS.subtext0)]
            .spacing(6)
            .align_y(Vertical::Center),
        |r, color| {
            let selected = !colorless_only && state.color_filter.has_color(color);
            r.push(
                button(color_icon(color))
                    .padding([5, 7])
                    .style(style::segment_button(selected))
                    .on_press(DecksMsg::ToggleColorFilter(color)),
            )
        },
    );

    let colorless_btn = button(colorless_icon())
        .padding([5, 7])
        .style(style::segment_button(colorless_only))
        .on_press(DecksMsg::ToggleColorlessFilter);

    let exact_btn = button(text(if exact_match { "Exact" } else { "Includes" }).size(11))
        .padding([5, 10])
        .style(style::segment_button(exact_match))
        .on_press(DecksMsg::ToggleExactColorMatch);

    let precons_check = checkbox(state.precons_only)
        .label("Precons only")
        .text_size(12)
        .on_toggle(DecksMsg::TogglePreconsOnly);

    let archived_check = checkbox(state.show_archived)
        .label("Show archived")
        .text_size(12)
        .on_toggle(DecksMsg::ToggleShowArchived);

    let count_label = if shown_count == total_count {
        format!("{total_count} decks")
    } else {
        format!("{shown_count} / {total_count} decks")
    };
    let count_badge = pill(count_label, COLORS.sapphire);

    let filter_row = row![
        color_buttons,
        colorless_btn,
        exact_btn,
        space::horizontal().width(12),
        precons_check,
        archived_check,
        space::horizontal(),
        count_badge,
    ]
    .spacing(8)
    .align_y(Vertical::Center)
    .width(Length::Fill);

    column![top_row, filter_row]
        .spacing(10)
        .width(Length::Fill)
        .into()
}

fn render_deck_row(player: RegisteredPlayer<'_>, is_editing: bool) -> iced::Element<'_, DecksMsg> {
    let id = player.id();
    let info = player.info();
    let stats = player.stats();

    let pips = mana_pips(info.color_identity());

    let mut title_row = row![
        button(
            text(info.name().clone())
                .font(FONT_BOLD)
                .size(14)
                .color(if info.is_archived() {
                    COLORS.subtext0
                } else {
                    COLORS.text
                }),
        )
        .padding([2, 4])
        .style(style::icon_button)
        .on_press(DecksMsg::OpenDetail(id)),
        pips,
    ]
    .spacing(8)
    .align_y(Vertical::Center);

    if let Some(bracket) = info.bracket() {
        title_row = title_row.push(pill(bracket.short_label(), bracket_accent(bracket)));
    }

    if info.is_precon() {
        title_row = title_row.push(pill("Precon", COLORS.teal));
    }

    if info.is_archived() {
        title_row = title_row.push(pill("Archived", COLORS.overlay1));
    }

    let mut info_col = column![title_row.wrap()].spacing(4).width(Length::Fill);

    if !info.description().is_empty() {
        info_col = info_col.push(text(info.description().clone()).size(12).color(COLORS.subtext0));
    }

    let wr_text = stats
        .wr()
        .map_or_else(|| "—".to_owned(), |wr| format!("{:.1}%", wr * 100.0));
    let stats_row = row![
        column![
            text(format!("{:.1}", stats.elo()))
                .font(FONT_BOLD)
                .size(14)
                .color(COLORS.text),
            text(format!("Peak {:.0}", stats.elo_peak()))
                .size(11)
                .color(COLORS.subtext0),
        ]
        .align_x(Horizontal::Right)
        .width(Length::Fixed(72.0)),
        column![
            text(wr_text).font(FONT_SEMIBOLD).size(13).color(COLORS.subtext1),
            text(format!("{}W / {}G", stats.wins(), stats.games()))
                .size(11)
                .color(COLORS.subtext0),
        ]
        .align_x(Horizontal::Right)
        .width(Length::Fixed(72.0)),
    ]
    .spacing(12)
    .align_y(Vertical::Center);

    let mut action_row = row![].spacing(4).align_y(Vertical::Center);

    if let Some(link) = info.moxfield_link() {
        action_row = action_row.push(
            button(text(md::MD_OPEN_IN_NEW).size(15))
                .padding([6, 8])
                .style(style::icon_button)
                .on_press(DecksMsg::OpenMoxfield(link)),
        );
    }

    let actions = action_row
        .push(
            button(text(md::MD_CHART_BAR).size(15))
                .padding([6, 8])
                .style(style::icon_button)
                .on_press(DecksMsg::OpenDetail(id)),
        )
        .push(
            button(text(md::MD_PENCIL_OUTLINE).size(15))
                .padding([6, 8])
                .style(style::icon_button)
                .on_press(DecksMsg::OpenEditDeck(id)),
        );

    container(
        row![info_col, stats_row, actions]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(style::table_row(is_editing))
    .into()
}

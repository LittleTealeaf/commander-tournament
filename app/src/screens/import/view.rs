use commander_tournament_core::{game::record::GameRecord, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, rule, space, text},
};
use iced_tea::Component;
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{empty_card, pill},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_SEMIBOLD},
    models::tournament::TournamentModel,
    style::{self, COLORS},
};

use super::{ImportMsg, ImportOrder, ImportState};

pub(super) fn render_import<'a>(
    state: &'a ImportState,
    context: &'a TournamentModel,
) -> Element<'a, ImportMsg> {
    let Some(current) = context.tournament() else {
        return empty_card(
            md::MD_FOLDER_OPEN_OUTLINE,
            "No tournament file open",
            "Open or create a tournament file before importing games into it.",
            Some(
                button(text("Back").size(13))
                    .padding([8, 14])
                    .style(style::secondary_button)
                    .on_press(ImportMsg::Cancel)
                    .into(),
            ),
        );
    };

    let header = render_header(state.source.data().is_some());
    let mut layout = column![header].spacing(16).width(Length::Fill);

    if let Some(err) = &state.error {
        layout = layout.push(
            container(text(err).size(13).color(COLORS.red))
                .width(Length::Fill)
                .padding([10, 14])
                .style(style::card),
        );
    }

    let Some(imported) = state.source.data() else {
        return layout
            .push(empty_card(
                md::MD_DATABASE_IMPORT,
                "Select a tournament file to import",
                "Pick a .ron, .json, or .toml file to merge its decks and games into your current tournament.",
                Some(
                    button(
                        row![
                            text(md::MD_FOLDER_OPEN_OUTLINE).size(15),
                            text("Choose File...").font(FONT_BOLD).size(13),
                        ]
                        .spacing(8)
                        .align_y(Vertical::Center),
                    )
                    .padding([8, 16])
                    .style(style::primary_button)
                    .on_press(ImportMsg::OpenFile)
                    .into(),
                ),
            ))
            .into();
    };

    let file_name = state.source.save_path().map_or_else(
        || "Loaded Tournament".to_owned(),
        |path| {
            path.file_name().map_or_else(
                || path.display().to_string(),
                |n| n.to_string_lossy().into_owned(),
            )
        },
    );

    layout
        .push(render_summary_card(&file_name, current, imported))
        .push(render_order_card(state.order))
        .push(render_deck_match_card(current, imported))
        .push(render_games_preview(imported, &state.paginator))
        .push(render_footer(imported))
        .into()
}

fn render_header<'a>(has_source: bool) -> Element<'a, ImportMsg> {
    let back_btn = button(
        row![text(md::MD_ARROW_LEFT).size(15), text("Back").size(13)]
            .spacing(6)
            .align_y(Vertical::Center),
    )
    .padding([8, 12])
    .style(style::secondary_button)
    .on_press(ImportMsg::Cancel);

    let mut actions = row![].spacing(8).align_y(Vertical::Center);
    if has_source {
        actions = actions
            .push(
                button(text("Clear File").size(13))
                    .padding([8, 12])
                    .style(style::secondary_button)
                    .on_press(ImportMsg::ClearFile),
            )
            .push(
                button(
                    row![
                        text(md::MD_FOLDER_OPEN_OUTLINE).size(14),
                        text("Change File").size(13),
                    ]
                    .spacing(6)
                    .align_y(Vertical::Center),
                )
                .padding([8, 12])
                .style(style::secondary_button)
                .on_press(ImportMsg::OpenFile),
            );
    }

    container(
        row![
            back_btn,
            text("Import & Splice Games")
                .font(FONT_BOLD)
                .size(18)
                .color(COLORS.text),
            space::horizontal(),
            actions,
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(style::card)
    .into()
}

fn render_summary_card<'a>(
    file_name: &str,
    current: &Tournament,
    imported: &Tournament,
) -> Element<'a, ImportMsg> {
    let total_games = imported.games().len();
    let total_decks = imported.players().len();
    let matched_decks = imported
        .players()
        .values()
        .filter(|info| current.get_player_id(info.name()).is_some())
        .count();
    let new_decks = total_decks.saturating_sub(matched_decks);

    container(
        row![
            row![
                text(md::MD_FILE_DOCUMENT_OUTLINE).size(18).color(COLORS.blue),
                text(file_name.to_owned())
                    .font(FONT_BOLD)
                    .size(14)
                    .color(COLORS.text),
            ]
            .spacing(8)
            .align_y(Vertical::Center),
            space::horizontal(),
            pill(format!("{total_games} games"), COLORS.sapphire),
            pill(format!("{matched_decks} matched decks"), COLORS.green),
            pill(format!("{new_decks} new decks"), COLORS.blue),
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(style::card)
    .into()
}

fn render_order_card<'a>(active_order: ImportOrder) -> Element<'a, ImportMsg> {
    let buttons = ImportOrder::VARIANTS.iter().copied().fold(
        row![].spacing(8).align_y(Vertical::Center),
        |r, order| {
            r.push(
                button(text(order.to_string()).font(FONT_MEDIUM).size(13))
                    .padding([8, 14])
                    .style(style::segment_button(order == active_order))
                    .on_press(ImportMsg::SetOrder(order)),
            )
        },
    );

    let explanation = match active_order {
        ImportOrder::Append => {
            "Appends imported games after your current match history and continues Plackett-Luce Elo calculation."
        }
        ImportOrder::Prepend => {
            "Inserts imported games before Game 1, shifts existing games after them, and replays all Plackett-Luce ratings chronologically."
        }
    };

    container(
        column![
            text("Timeline Splice Position")
                .font(FONT_BOLD)
                .size(15)
                .color(COLORS.text),
            buttons,
            text(explanation).size(12).color(COLORS.subtext0),
        ]
        .spacing(10)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn render_deck_match_card<'a>(current: &Tournament, imported: &Tournament) -> Element<'a, ImportMsg> {
    let mut decks: Vec<(String, bool)> = imported
        .players()
        .values()
        .map(|info| {
            let is_matched = current.get_player_id(info.name()).is_some();
            (info.display_name(), is_matched)
        })
        .collect();
    decks.sort_by_key(|(name, _)| name.to_lowercase());

    let chips = decks
        .into_iter()
        .fold(
            row![].spacing(6).align_y(Vertical::Center),
            |r, (name, matched)| {
                let accent = if matched { COLORS.green } else { COLORS.blue };
                r.push(pill(name, accent))
            },
        )
        .wrap();

    container(
        column![
            row![
                text("Decks in Import File")
                    .font(FONT_BOLD)
                    .size(15)
                    .color(COLORS.text),
                space::horizontal(),
                pill("Green = Matched by Name", COLORS.green),
                pill("Blue = New Deck", COLORS.blue),
            ]
            .spacing(8)
            .align_y(Vertical::Center)
            .width(Length::Fill),
            rule::horizontal(1),
            chips,
        ]
        .spacing(10)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn render_games_preview<'a>(
    imported: &'a Tournament,
    paginator: &'a crate::components::Paginator,
) -> Element<'a, ImportMsg> {
    let total = imported.games().len();
    if total == 0 {
        return container(
            text("This file contains no games (only decks will be imported).")
                .size(12)
                .color(COLORS.subtext0),
        )
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into();
    }

    let (skip, take) = paginator.slice_bounds(total);

    let rows = imported
        .games()
        .iter()
        .enumerate()
        .skip(skip)
        .take(take)
        .fold(column![].spacing(6), |col, (idx, record)| {
            col.push(render_preview_row(imported, idx, record))
        })
        .width(Length::Fill);

    let mut content = column![
        text(format!("Games to Import ({total})"))
            .font(FONT_BOLD)
            .size(15)
            .color(COLORS.text),
        rows,
    ]
    .spacing(10)
    .width(Length::Fill);

    if paginator.total_pages(total) > 1 {
        let pager: Element<'a, ImportMsg> = paginator.view_into(total);
        content = content.push(pager);
    }

    container(content)
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
}

fn render_preview_row<'a>(imported: &Tournament, idx: usize, record: &GameRecord) -> Element<'a, ImportMsg> {
    let winner_name = imported
        .get_player_display_name(&record.winner())
        .unwrap_or_else(|| format!("Deck {:?}", record.winner()));

    let mut winner_seen = false;
    let losers: Vec<String> = record
        .ids()
        .into_iter()
        .filter(|&id| {
            if !winner_seen && id == record.winner() {
                winner_seen = true;
                false
            } else {
                true
            }
        })
        .map(|id| {
            imported
                .get_player_display_name(&id)
                .unwrap_or_else(|| format!("Deck {id:?}"))
        })
        .collect();

    container(
        row![
            text(format!("#{}", idx + 1))
                .font(FONT_SEMIBOLD)
                .size(12)
                .color(COLORS.subtext0)
                .width(Length::Fixed(44.0)),
            text(md::MD_CROWN_OUTLINE).size(14).color(COLORS.green),
            text(winner_name).font(FONT_BOLD).size(13).color(COLORS.text),
            text(format!("vs {}", losers.join(" · ")))
                .size(12)
                .color(COLORS.subtext1)
                .width(Length::Fill),
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([8, 12])
    .style(style::table_row(false))
    .into()
}

fn render_footer<'a>(imported: &Tournament) -> Element<'a, ImportMsg> {
    let can_apply = !imported.games().is_empty() || !imported.players().is_empty();
    let label = format!("Import {} Games", imported.games().len());

    let mut apply_btn = button(
        row![
            text(md::MD_DATABASE_IMPORT).size(14),
            text(label).font(FONT_BOLD).size(13),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding([8, 16])
    .style(style::primary_button);

    if can_apply {
        apply_btn = apply_btn.on_press(ImportMsg::Apply);
    }

    container(
        row![
            space::horizontal(),
            button(text("Cancel").size(13))
                .padding([8, 14])
                .style(style::secondary_button)
                .on_press(ImportMsg::Cancel),
            apply_btn,
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(style::card)
    .into()
}

mod matches;
mod matchups;
mod overview;

use commander_tournament_core::{game::record::GameRecord, player::RegisteredPlayer, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, space, text},
};
use iced_tea::Component;
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{Paginator, mana_pips, pill},
    fonts::{FONT_BOLD, FONT_MEDIUM},
    style::{self, COLORS, bracket_accent},
};

use super::{DecksMsg, DetailTab, editor::DeckEditor};

pub(super) fn render_deck_detail<'a>(
    tournament: &'a Tournament,
    player: RegisteredPlayer<'a>,
    active_tab: DetailTab,
    detail_paginator: &'a Paginator,
    editor: Option<&'a DeckEditor>,
) -> Element<'a, DecksMsg> {
    let id = player.id();

    let mut seat_games = [0_u32; 4];
    let mut seat_wins = [0_u32; 4];
    let mut deck_games: Vec<(usize, &'a GameRecord)> = Vec::new();

    for (gid, record) in tournament.games().iter().enumerate() {
        if !record.has_player(id) {
            continue;
        }
        deck_games.push((gid, record));

        let mut winner_counted = false;
        for (seat_idx, mp) in record.players().iter().enumerate() {
            if mp.id() == id
                && let (Some(g), Some(w)) = (seat_games.get_mut(seat_idx), seat_wins.get_mut(seat_idx))
            {
                *g += 1;
                if !winner_counted && record.winner() == id {
                    *w += 1;
                    winner_counted = true;
                }
            }
        }
    }

    let header = render_detail_header(player);
    let tab_bar = render_detail_tabs(active_tab);

    let mut content = column![header].spacing(16).width(Length::Fill);

    if let Some(ed) = editor {
        content = content.push(ed.view_into(tournament));
    }

    content = content.push(tab_bar);

    let tab_body: Element<'a, DecksMsg> = match active_tab {
        DetailTab::Overview => {
            overview::render_overview_tab(tournament, player, &seat_games, &seat_wins, &deck_games)
        }
        DetailTab::VsDecks => matchups::render_vs_decks_tab(tournament, id, detail_paginator),
        DetailTab::VsIdentities => matchups::render_vs_identities_tab(tournament, id),
        DetailTab::Matches => matches::render_matches_tab(tournament, id, &deck_games, detail_paginator),
    };

    content.push(tab_body).into()
}

fn render_detail_header(player: RegisteredPlayer<'_>) -> Element<'_, DecksMsg> {
    let id = player.id();
    let info = player.info();

    let back_btn = button(
        row![text(md::MD_ARROW_LEFT).size(15), text("Back to Decks").size(13)]
            .spacing(6)
            .align_y(Vertical::Center),
    )
    .padding([8, 12])
    .style(style::secondary_button)
    .on_press(DecksMsg::CloseDetail);

    let mut badges = row![mana_pips(info.color_identity())]
        .spacing(8)
        .align_y(Vertical::Center);
    if let Some(bracket) = info.bracket() {
        badges = badges.push(pill(bracket.short_label(), bracket_accent(bracket)));
    }
    if info.is_precon() {
        badges = badges.push(pill("Precon", COLORS.teal));
    }
    if info.is_archived() {
        badges = badges.push(pill("Archived", COLORS.overlay1));
    }

    let mut header_actions = row![].spacing(8).align_y(Vertical::Center);
    if let Some(link) = info.moxfield_link() {
        header_actions = header_actions.push(
            button(
                row![text(md::MD_OPEN_IN_NEW).size(14), text("Moxfield").size(13)]
                    .spacing(6)
                    .align_y(Vertical::Center),
            )
            .padding([8, 12])
            .style(style::secondary_button)
            .on_press(DecksMsg::OpenMoxfield(link)),
        );
    }
    header_actions = header_actions
        .push(
            button(
                row![text(md::MD_PENCIL_OUTLINE).size(14), text("Edit").size(13)]
                    .spacing(6)
                    .align_y(Vertical::Center),
            )
            .padding([8, 12])
            .style(style::secondary_button)
            .on_press(DecksMsg::OpenEditDeck(id)),
        )
        .push(
            button(
                row![
                    text(md::MD_TARGET).size(14),
                    text("Practice This Deck").font(FONT_BOLD).size(13),
                ]
                .spacing(6)
                .align_y(Vertical::Center),
            )
            .padding([8, 14])
            .style(style::primary_button)
            .on_press(DecksMsg::PracticeDeck(id)),
        );

    let mut title_block = column![
        row![
            text(info.name().clone())
                .font(FONT_BOLD)
                .size(20)
                .color(COLORS.text),
            badges,
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .wrap(),
    ]
    .spacing(4)
    .width(Length::Fill);

    if !info.description().is_empty() {
        title_block = title_block.push(text(info.description().clone()).size(13).color(COLORS.subtext1));
    }

    container(
        column![
            row![back_btn, space::horizontal(), header_actions]
                .spacing(12)
                .align_y(Vertical::Center)
                .width(Length::Fill),
            title_block,
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn render_detail_tabs<'a>(active: DetailTab) -> Element<'a, DecksMsg> {
    DetailTab::VARIANTS
        .iter()
        .copied()
        .fold(row![].spacing(6).align_y(Vertical::Center), |r, tab| {
            r.push(
                button(text(tab.to_string()).font(FONT_MEDIUM).size(13))
                    .padding([8, 14])
                    .style(style::segment_button(tab == active))
                    .on_press(DecksMsg::SetDetailTab(tab)),
            )
        })
        .into()
}

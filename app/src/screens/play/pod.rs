use commander_tournament_core::{
    game::{match_player::MatchPlayer, matchup::Matchup},
    player::info::PlayerInfo,
    tournament::Tournament,
};
use iced::{
    Length,
    alignment::Vertical,
    widget::{button, column, container, row, rule, space, text},
};
use nerd_font_symbols::md;

use crate::{
    components::{mana_pips, pill},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_SEMIBOLD},
    style::{self, COLORS, bracket_accent},
};

use super::{PlayMsg, PodMode};

pub(super) fn render_pod_section<'a>(
    tournament: &Tournament,
    matchup: &Matchup,
    mode: PodMode,
    selected_winner: Option<usize>,
) -> iced::Element<'a, PlayMsg> {
    let cards = matchup.players().iter().enumerate().fold(
        row![]
            .spacing(12)
            .width(Length::Fill)
            .height(Length::Fixed(300.0)),
        |r, (seat_idx, mp)| {
            let is_target = mode == PodMode::TargetDeck && seat_idx == 0;
            let is_winner = selected_winner == Some(seat_idx);
            r.push(render_seat_card(tournament, seat_idx, mp, is_target, is_winner))
        },
    );

    let footer = render_confirmation_footer(tournament, matchup, selected_winner);

    column![cards, footer].spacing(16).width(Length::Fill).into()
}

fn render_seat_card<'a>(
    tournament: &Tournament,
    seat_idx: usize,
    mp: &MatchPlayer,
    is_target: bool,
    is_winner: bool,
) -> iced::Element<'a, PlayMsg> {
    let id = mp.id();
    let info = tournament.get_player_info(&id);
    let name = info.map_or_else(|| format!("Deck {id:?}"), |i| i.name().clone());

    let role_badge: Option<iced::Element<'a, PlayMsg>> = if is_winner {
        Some(
            container(
                row![
                    text(md::MD_CROWN_OUTLINE).size(12),
                    text("1st Place").font(FONT_BOLD).size(11),
                ]
                .spacing(4)
                .align_y(Vertical::Center),
            )
            .padding([2, 8])
            .style(style::pill_badge(COLORS.green))
            .into(),
        )
    } else if is_target {
        Some(
            container(text("Target").font(FONT_MEDIUM).size(11))
                .padding([2, 8])
                .style(style::pill_badge(COLORS.blue))
                .into(),
        )
    } else {
        None
    };

    let mut header = row![
        text(format!("Seat {}", seat_idx + 1))
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0),
        space::horizontal(),
    ]
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .height(Length::Fixed(24.0));

    if let Some(badge) = role_badge {
        header = header.push(badge);
    }

    let pips: iced::Element<'a, PlayMsg> =
        info.map_or_else(|| row![].into(), |i| mana_pips(i.color_identity()));

    let mut tags = row![pips].spacing(6).align_y(Vertical::Center);
    if let Some(bracket) = info.and_then(PlayerInfo::bracket) {
        tags = tags.push(pill(format!("B{}", bracket.number()), bracket_accent(bracket)));
    }
    if info.is_some_and(PlayerInfo::is_precon) {
        tags = tags.push(pill("Precon", COLORS.teal));
    }

    let stats_block = column![
        row![
            text("Elo").size(11).color(COLORS.subtext0),
            space::horizontal(),
            text(format!("{:.1}", mp.stats().elo()))
                .font(FONT_BOLD)
                .size(13)
                .color(COLORS.text),
        ],
        row![
            text("Expected Win").size(11).color(COLORS.subtext0),
            space::horizontal(),
            text(format!("{:.1}%", mp.expected() * 100.0))
                .font(FONT_MEDIUM)
                .size(12)
                .color(COLORS.subtext1),
        ],
        row![
            text("Stakes").size(11).color(COLORS.subtext0),
            space::horizontal(),
            row![
                text(format!("+{:.1}", mp.elo_win()))
                    .font(FONT_MEDIUM)
                    .size(11)
                    .color(COLORS.green),
                text("/").size(11).color(COLORS.overlay0),
                text(format!("-{:.1}", mp.elo_loss()))
                    .font(FONT_MEDIUM)
                    .size(11)
                    .color(COLORS.red),
            ]
            .spacing(4),
        ],
    ]
    .spacing(4)
    .width(Length::Fill);

    let mut card_col = column![
        header,
        button(text(name).font(FONT_BOLD).size(15).color(COLORS.text))
            .padding([2, 0])
            .style(style::icon_button)
            .on_press(PlayMsg::InspectDeck(id)),
        tags.wrap(),
        rule::horizontal(1),
        stats_block,
        space::vertical(),
    ]
    .spacing(8)
    .width(Length::Fill)
    .height(Length::Fill);

    if let Some(goldfish_url) = info.and_then(PlayerInfo::moxfield_goldfish_link) {
        card_col = card_col.push(
            button(
                row![
                    text(md::MD_OPEN_IN_NEW).size(12),
                    text("Goldfish on Moxfield").size(11),
                ]
                .spacing(6)
                .align_y(Vertical::Center),
            )
            .width(Length::Fill)
            .padding([6, 10])
            .style(style::secondary_button)
            .on_press(PlayMsg::OpenMoxfield(goldfish_url)),
        );
    }

    let winner_btn = if is_winner {
        button(
            row![
                text(md::MD_CROWN_OUTLINE).size(14),
                text("Selected Winner").font(FONT_BOLD).size(12),
            ]
            .spacing(6)
            .align_y(Vertical::Center),
        )
        .width(Length::Fill)
        .padding([8, 12])
        .style(style::primary_button)
        .on_press(PlayMsg::ClearWinner)
    } else {
        button(
            row![
                text(md::MD_CROWN_OUTLINE).size(14),
                text("Record 1st Place").size(12),
            ]
            .spacing(6)
            .align_y(Vertical::Center),
        )
        .width(Length::Fill)
        .padding([8, 12])
        .style(style::secondary_button)
        .on_press(PlayMsg::SelectWinner(seat_idx))
    };

    card_col = card_col.push(winner_btn);

    container(card_col)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(14)
        .style(style::pod_seat_card(is_target, is_winner))
        .into()
}

fn render_confirmation_footer<'a>(
    tournament: &Tournament,
    matchup: &Matchup,
    selected_winner: Option<usize>,
) -> iced::Element<'a, PlayMsg> {
    let status_text: iced::Element<'a, PlayMsg> = if let Some(seat_idx) = selected_winner
        && let Some(mp) = matchup.players().get(seat_idx)
    {
        let name = tournament
            .get_player_display_name(&mp.id())
            .unwrap_or_else(|| format!("Deck {:?}", mp.id()));
        row![
            text(md::MD_CROWN_OUTLINE).size(16).color(COLORS.green),
            text(format!(
                "Seat {} — {name} (+{:.1} Elo)",
                seat_idx + 1,
                mp.elo_win()
            ))
            .font(FONT_SEMIBOLD)
            .size(13)
            .color(COLORS.text),
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .into()
    } else {
        text("Select the 1st-place winner on one of the 4 seat cards above.")
            .size(13)
            .color(COLORS.subtext0)
            .into()
    };

    let mut confirm_btn = button(text("Confirm Result").size(13))
        .padding([8, 14])
        .style(style::secondary_button);
    let mut confirm_next_btn = button(text("Confirm & Next Pod").font(FONT_BOLD).size(13))
        .padding([8, 16])
        .style(style::primary_button);

    if selected_winner.is_some() {
        confirm_btn = confirm_btn.on_press(PlayMsg::ConfirmResult { generate_next: false });
        confirm_next_btn = confirm_next_btn.on_press(PlayMsg::ConfirmResult { generate_next: true });
    }

    container(
        row![
            status_text,
            space::horizontal(),
            button(text("Clear Pod").size(13))
                .padding([8, 12])
                .style(style::secondary_button)
                .on_press(PlayMsg::ClearPod),
            confirm_btn,
            confirm_next_btn,
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

use commander_tournament_core::{
    analytics::winloss::MatchPerformance,
    player::{PlayerId, RegisteredPlayer, color::ColorIdentity},
    tournament::Tournament,
};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, space, text},
};
use iced_tea::Component;

use crate::{
    components::{Paginator, mana_pips, progress_bar},
    fonts::{FONT_BOLD, FONT_MEDIUM, FONT_SEMIBOLD},
    style::{self, COLORS},
};

use super::super::DecksMsg;

pub(super) fn render_vs_decks_tab<'a>(
    tournament: &'a Tournament,
    id: PlayerId,
    paginator: &'a Paginator,
) -> Element<'a, DecksMsg> {
    let mut matchups: Vec<(RegisteredPlayer<'a>, MatchPerformance)> = tournament
        .analytics()
        .player_vs_player_performance(id)
        .map_or_default(Iterator::collect);

    matchups.sort_by(|(pa, perfa), (pb, perfb)| {
        perfb
            .cmp(perfa)
            .then_with(|| pb.stats().elo().total_cmp(&pa.stats().elo()))
    });

    if matchups.is_empty() {
        return container(
            text("No head-to-head deck matchups recorded yet.")
                .size(13)
                .color(COLORS.subtext0),
        )
        .width(Length::Fill)
        .padding(24)
        .style(style::card)
        .into();
    }

    let total = matchups.len();
    let (skip, take) = paginator.slice_bounds(total);

    let header_row = row![
        text("Opponent Deck")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fill),
        text("Identity")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(110.0)),
        text("Elo")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(60.0)),
        text("W")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.green)
            .width(Length::Fixed(40.0)),
        text("D")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.subtext0)
            .width(Length::Fixed(40.0)),
        text("L")
            .font(FONT_SEMIBOLD)
            .size(12)
            .color(COLORS.red)
            .width(Length::Fixed(40.0)),
    ]
    .spacing(10)
    .padding([0, 12]);

    let rows = matchups
        .into_iter()
        .skip(skip)
        .take(take)
        .fold(column![header_row].spacing(6), |col, (opp, perf)| {
            col.push(
                container(
                    row![
                        button(text(opp.info().display_name()).size(13).color(COLORS.text))
                            .padding([2, 4])
                            .style(style::icon_button)
                            .on_press(DecksMsg::OpenDetail(opp.id())),
                        space::horizontal(),
                        container(mana_pips(opp.info().color_identity())).width(Length::Fixed(110.0)),
                        text(format!("{:.0}", opp.stats().elo()))
                            .size(12)
                            .color(COLORS.subtext1)
                            .width(Length::Fixed(60.0)),
                        text(perf.wins().to_string())
                            .font(FONT_MEDIUM)
                            .size(12)
                            .color(COLORS.green)
                            .width(Length::Fixed(40.0)),
                        text(perf.draws().to_string())
                            .size(12)
                            .color(COLORS.subtext0)
                            .width(Length::Fixed(40.0)),
                        text(perf.losses().to_string())
                            .font(FONT_MEDIUM)
                            .size(12)
                            .color(COLORS.red)
                            .width(Length::Fixed(40.0)),
                    ]
                    .spacing(10)
                    .align_y(Vertical::Center)
                    .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding([8, 12])
                .style(style::table_row(false)),
            )
        })
        .width(Length::Fill);

    column![rows, paginator.view(total).map(DecksMsg::DetailPaginator)]
        .spacing(12)
        .width(Length::Fill)
        .into()
}

pub(super) fn render_vs_identities_tab(tournament: &Tournament, id: PlayerId) -> Element<'_, DecksMsg> {
    let mut matchups: Vec<(ColorIdentity, MatchPerformance)> = tournament
        .analytics()
        .player_vs_identity_performance(id)
        .unwrap_or_default()
        .into_iter()
        .collect();

    matchups.sort_by(|(id_a, perf_a), (id_b, perf_b)| perf_b.cmp(perf_a).then_with(|| id_a.cmp(id_b)));

    if matchups.is_empty() {
        return container(
            text("No color identity matchup data recorded yet.")
                .size(13)
                .color(COLORS.subtext0),
        )
        .width(Length::Fill)
        .padding(24)
        .style(style::card)
        .into();
    }

    let rows = matchups
        .into_iter()
        .fold(column![].spacing(6), |col, (identity, perf)| {
            let wr = perf.winrate();
            let ratio = wr.unwrap_or(0.0);
            let pct_label = wr.map_or_else(|| "—".to_owned(), |v| format!("{:.1}%", v * 100.0));

            col.push(
                container(
                    row![
                        text(identity.to_string())
                            .font(FONT_MEDIUM)
                            .size(13)
                            .color(COLORS.text)
                            .width(Length::Fixed(100.0)),
                        container(mana_pips(identity)).width(Length::Fixed(96.0)),
                        progress_bar(ratio, COLORS.blue),
                        text(pct_label)
                            .font(FONT_SEMIBOLD)
                            .size(12)
                            .color(COLORS.text)
                            .width(Length::Fixed(52.0)),
                        text(format!("{}-{}-{}", perf.wins(), perf.losses(), perf.draws()))
                            .size(12)
                            .color(COLORS.subtext0)
                            .width(Length::Fixed(64.0)),
                    ]
                    .spacing(12)
                    .align_y(Vertical::Center)
                    .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding([8, 12])
                .style(style::table_row(false)),
            )
        })
        .width(Length::Fill);

    container(
        column![
            text("Performance vs. Opposing Color Identities (W-L-D)")
                .font(FONT_BOLD)
                .size(15)
                .color(COLORS.text),
            rows,
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

mod analytics;
mod leaderboard;

use commander_tournament_core::{
    player::{PlayerId, bracket::Bracket},
    tournament::Tournament,
};
use iced::{
    Length,
    alignment::Vertical,
    widget::{button, checkbox, column, row, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{Paginator, PaginatorMsg, empty_card, kpi_card},
    fonts::FONT_MEDIUM,
    models::tournament::TournamentModel,
    style::{self, COLORS},
};

#[derive(Debug)]
pub struct OverviewState {
    tab: OverviewTab,
    search: String,
    bracket_filter: OverviewBracketFilter,
    include_precons: bool,
    include_archived: bool,
    paginator: Paginator,
}

impl Default for OverviewState {
    fn default() -> Self {
        Self {
            tab: OverviewTab::Leaderboard,
            search: String::new(),
            bracket_filter: OverviewBracketFilter::All,
            include_precons: true,
            include_archived: false,
            paginator: Paginator::compact(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::VariantArray, derive_more::Display)]
pub enum OverviewTab {
    #[default]
    Leaderboard,
    Analytics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, derive_more::Display)]
pub enum OverviewBracketFilter {
    #[default]
    #[display("All Brackets")]
    All,
    #[display("{}", _0.short_label())]
    Bracket(Bracket),
}

impl OverviewBracketFilter {
    fn all_options() -> Vec<Self> {
        let mut opts = Vec::with_capacity(Bracket::VARIANTS.len() + 1);
        opts.push(Self::All);
        opts.extend(Bracket::VARIANTS.iter().copied().map(Self::Bracket));
        opts
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum OverviewMsg {
    #[from(ignore)]
    SetTab(OverviewTab),
    #[from(ignore)]
    SearchChanged(String),
    #[from(ignore)]
    BracketFilterChanged(OverviewBracketFilter),
    #[from(ignore)]
    ToggleIncludePrecons(bool),
    #[from(ignore)]
    ToggleIncludeArchived(bool),
    Paginator(PaginatorMsg),
    #[from(ignore)]
    InspectDeck(PlayerId),
    GoToPlay,
}

#[derive(Debug, Clone)]
pub enum OverviewOut {
    InspectDeck(PlayerId),
    GoToPlay,
}

impl Model for OverviewState {
    type Message = OverviewMsg;
    type OutMessage = OverviewOut;
    type Context<'a>
        = &'a TournamentModel
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            OverviewMsg::SetTab(tab) => {
                self.tab = tab;
                Signal::done()
            }
            OverviewMsg::SearchChanged(query) => {
                self.search = query;
                self.paginator.reset();
                Signal::done()
            }
            OverviewMsg::BracketFilterChanged(filter) => {
                self.bracket_filter = filter;
                self.paginator.reset();
                Signal::done()
            }
            OverviewMsg::ToggleIncludePrecons(include) => {
                self.include_precons = include;
                self.paginator.reset();
                Signal::done()
            }
            OverviewMsg::ToggleIncludeArchived(include) => {
                self.include_archived = include;
                self.paginator.reset();
                Signal::done()
            }
            OverviewMsg::Paginator(msg) => self.paginator.empty_update(msg, 0),
            OverviewMsg::InspectDeck(id) => Signal::out(OverviewOut::InspectDeck(id)).ok(),
            OverviewMsg::GoToPlay => Signal::out(OverviewOut::GoToPlay).ok(),
        }
    }
}

impl Component for OverviewState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        let Some(tournament) = context.tournament() else {
            return empty_card(
                md::MD_FOLDER_OPEN_OUTLINE,
                "No tournament file open",
                "Create a new file or open an existing .ron file from the top bar.",
                None,
            );
        };

        let kpi_strip = render_kpi_strip(tournament);

        let tabs = OverviewTab::VARIANTS.iter().copied().fold(
            row![].spacing(6).align_y(Vertical::Center),
            |r, tab| {
                r.push(
                    button(text(tab.to_string()).font(FONT_MEDIUM).size(13))
                        .padding([8, 14])
                        .style(style::segment_button(tab == self.tab))
                        .on_press(OverviewMsg::SetTab(tab)),
                )
            },
        );

        let toggles = row![
            checkbox(self.include_precons)
                .label("Include precons")
                .text_size(12)
                .on_toggle(OverviewMsg::ToggleIncludePrecons),
            checkbox(self.include_archived)
                .label("Include archived")
                .text_size(12)
                .on_toggle(OverviewMsg::ToggleIncludeArchived),
        ]
        .spacing(12)
        .align_y(Vertical::Center);

        let sub_header = row![tabs, space::horizontal(), toggles]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill);

        let body = match self.tab {
            OverviewTab::Leaderboard => leaderboard::render_leaderboard(self, tournament),
            OverviewTab::Analytics => analytics::render_analytics(self, tournament),
        };

        column![kpi_strip, sub_header, body]
            .spacing(16)
            .width(Length::Fill)
            .into()
    }
}

fn render_kpi_strip<'a>(tournament: &Tournament) -> iced::Element<'a, OverviewMsg> {
    let total_decks = tournament.players().len();
    let active_decks = tournament
        .players()
        .values()
        .filter(|info| !info.is_archived())
        .count();
    let total_games = tournament.games().len();

    let top_deck = tournament
        .registered_players()
        .filter(|p| !p.info().is_archived())
        .max_by(|a, b| a.stats().elo().total_cmp(&b.stats().elo()));

    let (top_value, top_sub) = top_deck.map_or_else(
        || ("—".to_owned(), "No decks yet".to_owned()),
        |p| (format!("{:.1}", p.stats().elo()), p.info().display_name()),
    );

    let calibration = tournament.game_config().calibration_games;
    let calibrated = tournament
        .registered_players()
        .filter(|p| !p.info().is_archived() && p.stats().games() >= calibration)
        .count();

    row![
        kpi_card(
            "Active Decks",
            active_decks.to_string(),
            format!("{total_decks} total registered"),
            COLORS.blue,
        ),
        kpi_card(
            "Goldfish Games",
            total_games.to_string(),
            "Recorded 4-player pods".to_owned(),
            COLORS.sapphire,
        ),
        kpi_card("Top Rated Deck", top_value, top_sub, COLORS.green),
        kpi_card(
            "Calibrated Decks",
            format!("{calibrated} / {active_decks}"),
            format!("Threshold: {calibration} games"),
            COLORS.mauve,
        ),
    ]
    .spacing(12)
    .width(Length::Fill)
    .into()
}

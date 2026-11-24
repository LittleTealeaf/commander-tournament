use commander_tournament_core::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, space, text, text_input},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;

use crate::{
    components::{DeckPicker, DeckPickerMsg, Paginator, PaginatorMsg, empty_card, pill},
    fonts::{FONT_BOLD, FONT_SEMIBOLD},
    models::tournament::TournamentModel,
    style::{self, COLORS},
};

#[derive(Debug)]
pub struct MatchHistoryState {
    search: String,
    deck_picker: DeckPicker,
    oldest_first: bool,
    confirm_delete: Option<usize>,
    paginator: Paginator,
}

impl Default for MatchHistoryState {
    fn default() -> Self {
        Self {
            search: String::new(),
            deck_picker: DeckPicker::all_decks("Filter by deck...")
                .with_width(Length::Fixed(280.0))
                .with_clearable(true),
            oldest_first: false,
            confirm_delete: None,
            paginator: Paginator::default(),
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum MatchHistoryMsg {
    #[from(ignore)]
    SearchChanged(String),
    DeckPicker(DeckPickerMsg),
    ResetPage,
    ClearFilters,
    ToggleSortOrder,
    Paginator(PaginatorMsg),
    #[from(ignore)]
    RequestDelete(usize),
    CancelDelete,
    #[from(ignore)]
    ConfirmDelete(usize),
    OpenImport,
}

#[derive(Debug, Clone)]
pub enum MatchHistoryOut {
    DeleteGame(usize),
    OpenImport,
}

impl Model for MatchHistoryState {
    type Message = MatchHistoryMsg;
    type OutMessage = MatchHistoryOut;
    type Context<'a>
        = &'a TournamentModel
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            MatchHistoryMsg::SearchChanged(query) => {
                self.search = query;
                self.paginator.reset();
                self.confirm_delete = None;
                Signal::done()
            }
            MatchHistoryMsg::DeckPicker(msg) => {
                let Some(tournament) = context.tournament() else {
                    return Signal::done();
                };
                self.deck_picker
                    .map_update(msg, tournament, |_| Ok(Signal::msg(MatchHistoryMsg::ResetPage)))
            }
            MatchHistoryMsg::ResetPage => {
                self.paginator.reset();
                self.confirm_delete = None;
                Signal::done()
            }
            MatchHistoryMsg::ClearFilters => {
                self.search.clear();
                self.deck_picker.clear();
                self.paginator.reset();
                self.confirm_delete = None;
                Signal::done()
            }
            MatchHistoryMsg::ToggleSortOrder => {
                self.oldest_first = !self.oldest_first;
                self.paginator.reset();
                self.confirm_delete = None;
                Signal::done()
            }
            MatchHistoryMsg::Paginator(msg) => {
                self.confirm_delete = None;
                self.paginator.empty_update(msg, 0)
            }
            MatchHistoryMsg::RequestDelete(gid) => {
                self.confirm_delete = Some(gid);
                Signal::done()
            }
            MatchHistoryMsg::CancelDelete => {
                self.confirm_delete = None;
                Signal::done()
            }
            MatchHistoryMsg::ConfirmDelete(gid) => {
                self.confirm_delete = None;
                Signal::out(MatchHistoryOut::DeleteGame(gid)).ok()
            }
            MatchHistoryMsg::OpenImport => Signal::out(MatchHistoryOut::OpenImport).ok(),
        }
    }
}

impl Component for MatchHistoryState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let Some(tournament) = context.tournament() else {
            return empty_card(
                md::MD_FOLDER_OPEN_OUTLINE,
                "No tournament file open",
                "Create a new file or open an existing .ron file from the top bar to view match history.",
                None,
            );
        };

        let search_lower = self.search.trim().to_lowercase();
        let total_games = tournament.games().len();
        let deck_filter = self.deck_picker.selected();

        let mut filtered: Vec<(usize, &GameRecord)> = tournament
            .games()
            .iter()
            .enumerate()
            .filter(|(_, record)| matches_filters(tournament, record, deck_filter, &search_lower))
            .collect();

        if !self.oldest_first {
            filtered.reverse();
        }

        let toolbar = render_toolbar(
            tournament,
            &self.search,
            &self.deck_picker,
            self.oldest_first,
            filtered.len(),
            total_games,
        );

        let body: Element<'a, MatchHistoryMsg> = if total_games == 0 {
            empty_card(
                md::MD_HISTORY,
                "No matches recorded yet",
                "Play a pod or import games from a .ron file to populate your match history.",
                Some(
                    button(
                        row![
                            text(md::MD_DATABASE_IMPORT).size(14),
                            text("Import Games").font(FONT_BOLD).size(13),
                        ]
                        .spacing(8)
                        .align_y(Vertical::Center),
                    )
                    .padding([8, 14])
                    .style(style::primary_button)
                    .on_press(MatchHistoryMsg::OpenImport)
                    .into(),
                ),
            )
        } else if filtered.is_empty() {
            empty_card(
                md::MD_MAGNIFY,
                "No matching games found",
                "Try adjusting your deck filter or search query.",
                Some(
                    button(text("Clear Filters").size(13))
                        .padding([8, 14])
                        .style(style::secondary_button)
                        .on_press(MatchHistoryMsg::ClearFilters)
                        .into(),
                ),
            )
        } else {
            let total_filtered = filtered.len();
            let (skip, take) = self.paginator.slice_bounds(total_filtered);

            let rows = filtered
                .into_iter()
                .skip(skip)
                .take(take)
                .fold(column![].spacing(8), |col, (gid, record)| {
                    col.push(render_match_row(
                        tournament,
                        gid,
                        record,
                        self.confirm_delete == Some(gid),
                    ))
                })
                .width(Length::Fill);

            let pager: Element<'a, MatchHistoryMsg> = self.paginator.view_into(total_filtered);

            column![rows, pager].spacing(16).width(Length::Fill).into()
        };

        column![toolbar, body].spacing(16).width(Length::Fill).into()
    }
}

fn matches_filters(
    tournament: &Tournament,
    record: &GameRecord,
    deck_filter: Option<PlayerId>,
    search_lower: &str,
) -> bool {
    if let Some(target_id) = deck_filter
        && !record.has_player(target_id)
    {
        return false;
    }

    if search_lower.is_empty() {
        return true;
    }

    record.ids().into_iter().any(|id| {
        tournament
            .get_player_display_name(&id)
            .is_some_and(|name| name.to_lowercase().contains(search_lower))
    })
}

fn render_toolbar<'a>(
    tournament: &'a Tournament,
    search: &'a str,
    deck_picker: &'a DeckPicker,
    oldest_first: bool,
    shown_count: usize,
    total_count: usize,
) -> Element<'a, MatchHistoryMsg> {
    let search_box = text_input("Search decks...", search)
        .on_input(MatchHistoryMsg::SearchChanged)
        .size(13)
        .padding([8, 12])
        .width(Length::Fixed(220.0))
        .style(style::search_input);

    let (sort_icon, sort_label) = if oldest_first {
        (md::MD_SORT_ASCENDING, "Oldest First")
    } else {
        (md::MD_SORT_DESCENDING, "Newest First")
    };

    let sort_btn = button(
        row![text(sort_icon).size(14), text(sort_label).size(13)]
            .spacing(6)
            .align_y(Vertical::Center),
    )
    .padding([8, 12])
    .style(style::secondary_button)
    .on_press(MatchHistoryMsg::ToggleSortOrder);

    let count_label = if shown_count == total_count {
        format!("{total_count} games")
    } else {
        format!("{shown_count} / {total_count} games")
    };

    let count_badge = pill(count_label, COLORS.sapphire);

    let import_btn = button(
        row![
            text(md::MD_DATABASE_IMPORT).size(14),
            text("Import & Splice Games").font(FONT_BOLD).size(13),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding([8, 14])
    .style(style::primary_button)
    .on_press(MatchHistoryMsg::OpenImport);

    row![
        deck_picker.view_into(tournament),
        search_box,
        sort_btn,
        count_badge,
        space::horizontal(),
        import_btn,
    ]
    .spacing(10)
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .into()
}

fn render_match_row<'a>(
    tournament: &Tournament,
    gid: usize,
    record: &GameRecord,
    confirming_delete: bool,
) -> Element<'a, MatchHistoryMsg> {
    let mut winner_tracked = false;
    let mut winner_entry: Option<(String, f64)> = None;
    let mut loser_entries: Vec<(String, f64)> = Vec::with_capacity(3);

    for player in record.players() {
        let name = tournament
            .get_player_display_name(&player.id())
            .unwrap_or_else(|| format!("Deck {:?}", player.id()));

        if !winner_tracked && player.id() == record.winner() {
            winner_tracked = true;
            winner_entry = Some((name, *player.elo_win()));
        } else {
            loser_entries.push((name, *player.elo_loss()));
        }
    }

    let game_index = container(
        text(format!("#{}", gid + 1))
            .font(FONT_SEMIBOLD)
            .size(13)
            .color(COLORS.subtext0),
    )
    .width(Length::Fixed(48.0));

    let winner_col: Element<'a, MatchHistoryMsg> = if let Some((name, delta)) = winner_entry {
        row![
            text(md::MD_CROWN_OUTLINE).size(15).color(COLORS.green),
            text(name).font(FONT_BOLD).size(14).color(COLORS.text),
            pill(format!("+{delta:.1}"), COLORS.green),
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .into()
    } else {
        text("Unknown winner").size(13).color(COLORS.subtext0).into()
    };

    let losers_row = loser_entries
        .into_iter()
        .fold(row![].spacing(16).align_y(Vertical::Center), |r, (name, loss)| {
            r.push(
                row![
                    text(name).size(12).color(COLORS.subtext1),
                    text(format!("-{loss:.1}")).size(11).color(COLORS.red),
                ]
                .spacing(4)
                .align_y(Vertical::Center),
            )
        })
        .wrap();

    let details = column![winner_col, losers_row].spacing(6).width(Length::Fill);

    let actions: Element<'a, MatchHistoryMsg> = if confirming_delete {
        row![
            button(text("Cancel").size(12))
                .padding([6, 10])
                .style(style::secondary_button)
                .on_press(MatchHistoryMsg::CancelDelete),
            button(text("Delete").font(FONT_BOLD).size(12))
                .padding([6, 10])
                .style(style::danger_button)
                .on_press(MatchHistoryMsg::ConfirmDelete(gid)),
        ]
        .spacing(6)
        .align_y(Vertical::Center)
        .into()
    } else {
        button(text(md::MD_TRASH_CAN_OUTLINE).size(16))
            .padding([6, 8])
            .style(style::icon_button)
            .on_press(MatchHistoryMsg::RequestDelete(gid))
            .into()
    };

    container(
        row![game_index, details, actions]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(style::table_row(confirming_delete))
    .into()
}

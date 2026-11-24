mod detail;
mod editor;
mod list;

use commander_tournament_core::player::{
    PlayerId,
    bracket::Bracket,
    color::{ColorIdentity, MtgColor},
    info::PlayerInfo,
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{Paginator, PaginatorMsg, empty_card},
    models::tournament::TournamentModel,
};

use self::editor::{DeckEditor, DeckEditorMsg, DeckEditorOut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ColorFilterMode {
    #[default]
    Includes,
    Exact,
    ColorlessOnly,
}

#[derive(Debug)]
pub struct DecksState {
    search: String,
    color_filter: ColorIdentity,
    color_mode: ColorFilterMode,
    bracket_filter: BracketFilter,
    show_archived: bool,
    precons_only: bool,
    sort_by: DeckSortField,
    sort_asc: bool,
    paginator: Paginator,
    editor: Option<DeckEditor>,
    selected_deck: Option<PlayerId>,
    detail_tab: DetailTab,
    detail_paginator: Paginator,
}

impl Default for DecksState {
    fn default() -> Self {
        Self {
            search: String::new(),
            color_filter: ColorIdentity::COLORLESS,
            color_mode: ColorFilterMode::Includes,
            bracket_filter: BracketFilter::All,
            show_archived: false,
            precons_only: false,
            sort_by: DeckSortField::Elo,
            sort_asc: false,
            paginator: Paginator::default(),
            editor: None,
            selected_deck: None,
            detail_tab: DetailTab::Overview,
            detail_paginator: Paginator::compact(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::VariantArray, derive_more::Display)]
pub enum DetailTab {
    #[default]
    Overview,
    #[display("vs. Decks")]
    VsDecks,
    #[display("vs. Identities")]
    VsIdentities,
    #[display("Match History")]
    Matches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, derive_more::Display)]
pub enum BracketFilter {
    #[default]
    #[display("All Brackets")]
    All,
    #[display("No Bracket")]
    Unassigned,
    #[display("{}", _0.short_label())]
    Bracket(Bracket),
}

impl BracketFilter {
    fn all_options() -> Vec<Self> {
        let mut opts = Vec::with_capacity(Bracket::VARIANTS.len() + 2);
        opts.push(Self::All);
        opts.push(Self::Unassigned);
        opts.extend(Bracket::VARIANTS.iter().copied().map(Self::Bracket));
        opts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::VariantArray, derive_more::Display)]
pub enum DeckSortField {
    #[display("Sort: Name")]
    Name,
    #[default]
    #[display("Sort: Elo")]
    Elo,
    #[display("Sort: Win Rate")]
    WinRate,
    #[display("Sort: Games")]
    Games,
    #[display("Sort: Bracket")]
    Bracket,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum DecksMsg {
    #[from(ignore)]
    SearchChanged(String),
    #[from(ignore)]
    ToggleColorFilter(MtgColor),
    ToggleColorlessFilter,
    ToggleExactColorMatch,
    #[from(ignore)]
    BracketFilterChanged(BracketFilter),
    #[from(ignore)]
    ToggleShowArchived(bool),
    #[from(ignore)]
    TogglePreconsOnly(bool),
    #[from(ignore)]
    SortFieldChanged(DeckSortField),
    ToggleSortDirection,
    ClearFilters,
    Paginator(PaginatorMsg),

    // Deck detail sub-screen
    #[from(ignore)]
    OpenDetail(PlayerId),
    CloseDetail,
    #[from(ignore)]
    SetDetailTab(DetailTab),
    #[from(ignore)]
    DetailPaginator(PaginatorMsg),
    #[from(ignore)]
    PracticeDeck(PlayerId),

    // Quick row actions
    #[from(ignore)]
    OpenMoxfield(String),

    // Child components
    OpenAddDeck,
    #[from(ignore)]
    OpenEditDeck(PlayerId),
    CloseEditor,
    Editor(DeckEditorMsg),
}

#[derive(Debug, Clone)]
pub enum DecksOut {
    RegisterDeck(PlayerInfo),
    UpdateDeck(PlayerId, PlayerInfo),
    DeleteDeck(PlayerId),
    MergeDecks { target: PlayerId, other: PlayerId },
    PracticeDeck(PlayerId),
}

impl Model for DecksState {
    type Message = DecksMsg;
    type OutMessage = DecksOut;
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
            DecksMsg::SearchChanged(query) => {
                self.search = query;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ToggleColorFilter(color) => {
                if self.color_mode == ColorFilterMode::ColorlessOnly {
                    self.color_mode = ColorFilterMode::Includes;
                }
                if self.color_filter.has_color(color) {
                    self.color_filter.remove_color(color);
                } else {
                    self.color_filter.add_color(color);
                }
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ToggleColorlessFilter => {
                if self.color_mode == ColorFilterMode::ColorlessOnly {
                    self.color_mode = ColorFilterMode::Includes;
                } else {
                    self.color_mode = ColorFilterMode::ColorlessOnly;
                    self.color_filter = ColorIdentity::COLORLESS;
                }
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ToggleExactColorMatch => {
                self.color_mode = match self.color_mode {
                    ColorFilterMode::Exact => ColorFilterMode::Includes,
                    ColorFilterMode::Includes | ColorFilterMode::ColorlessOnly => ColorFilterMode::Exact,
                };
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::BracketFilterChanged(filter) => {
                self.bracket_filter = filter;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ToggleShowArchived(show) => {
                self.show_archived = show;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::TogglePreconsOnly(only) => {
                self.precons_only = only;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::SortFieldChanged(field) => {
                if self.sort_by != field {
                    self.sort_by = field;
                    self.sort_asc = matches!(field, DeckSortField::Name);
                }
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ToggleSortDirection => {
                self.sort_asc = !self.sort_asc;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::ClearFilters => {
                self.search.clear();
                self.color_filter = ColorIdentity::COLORLESS;
                self.color_mode = ColorFilterMode::Includes;
                self.bracket_filter = BracketFilter::All;
                self.show_archived = false;
                self.precons_only = false;
                self.paginator.reset();
                Signal::done()
            }
            DecksMsg::Paginator(msg) => self.paginator.empty_update(msg, 0),
            DecksMsg::OpenDetail(id) => {
                self.selected_deck = Some(id);
                self.detail_tab = DetailTab::Overview;
                self.detail_paginator.reset();
                self.editor = None;
                Signal::done()
            }
            DecksMsg::CloseDetail => {
                self.selected_deck = None;
                self.editor = None;
                Signal::done()
            }
            DecksMsg::SetDetailTab(tab) => {
                self.detail_tab = tab;
                self.detail_paginator.reset();
                Signal::done()
            }
            DecksMsg::DetailPaginator(msg) => {
                let _ = self.detail_paginator.update(msg, 0)?;
                Signal::done()
            }
            DecksMsg::PracticeDeck(id) => Signal::out(DecksOut::PracticeDeck(id)).ok(),
            DecksMsg::OpenMoxfield(url) => {
                if let Err(err) = opener::open(&url) {
                    log::warn!("Failed to open Moxfield link {url}: {err}");
                }
                Signal::done()
            }
            DecksMsg::OpenAddDeck => {
                self.editor = Some(DeckEditor::new_deck());
                Signal::done()
            }
            DecksMsg::OpenEditDeck(id) => {
                if let Some(tournament) = context.tournament()
                    && let Some(info) = tournament.get_player_info(&id)
                {
                    self.editor = Some(DeckEditor::from_existing(id, info, tournament));
                }
                Signal::done()
            }
            DecksMsg::CloseEditor => {
                self.editor = None;
                Signal::done()
            }
            DecksMsg::Editor(msg) => {
                let (Some(editor), Some(tournament)) = (&mut self.editor, context.tournament()) else {
                    return Signal::done();
                };
                editor.map_update(msg, tournament, |out| {
                    Ok(match out {
                        DeckEditorOut::Close => Signal::msg(DecksMsg::CloseEditor),
                        DeckEditorOut::Delete(id) => {
                            Signal::out(DecksOut::DeleteDeck(id)).chain(Signal::msg(DecksMsg::CloseDetail))
                        }
                        DeckEditorOut::Merge { target, other } => {
                            Signal::out(DecksOut::MergeDecks { target, other })
                                .chain(Signal::msg(DecksMsg::CloseEditor))
                        }
                        DeckEditorOut::Save { editing_id, info } => {
                            let save = match editing_id {
                                Some(id) => Signal::out(DecksOut::UpdateDeck(id, info)),
                                None => Signal::out(DecksOut::RegisterDeck(info)),
                            };
                            save.chain(Signal::msg(DecksMsg::CloseEditor))
                        }
                    })
                })
            }
        }
    }
}

impl Component for DecksState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        let Some(tournament) = context.tournament() else {
            return empty_card(
                md::MD_FOLDER_OPEN_OUTLINE,
                "No tournament file open",
                "Create a new file or open an existing .ron file from the top bar to manage decks.",
                None,
            );
        };

        if let Some(deck_id) = self.selected_deck
            && let Some(player) = tournament.get_registered_player(deck_id)
        {
            return detail::render_deck_detail(
                tournament,
                player,
                self.detail_tab,
                &self.detail_paginator,
                self.editor.as_ref(),
            );
        }

        list::render_decks_list(self, tournament)
    }
}

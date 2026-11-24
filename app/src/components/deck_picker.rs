use core::fmt;

use commander_tournament_core::{player::PlayerId, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, combo_box, combo_box::State as ComboState, row, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;

use crate::style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckOption {
    pub id: PlayerId,
    pub label: String,
}

impl fmt::Display for DeckOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

#[derive(Debug, Clone)]
pub enum DeckPickerMsg {
    Open,
    Select(PlayerId),
    Clear,
}

#[derive(Debug)]
pub struct DeckPicker {
    combo: ComboState<DeckOption>,
    selected: Option<PlayerId>,
    include_archived: bool,
    clearable: bool,
    placeholder: &'static str,
    width: Length,
}

impl Default for DeckPicker {
    fn default() -> Self {
        Self::active_only("Search deck...")
    }
}

impl DeckPicker {
    #[must_use]
    pub fn active_only(placeholder: &'static str) -> Self {
        Self {
            combo: ComboState::new(Vec::new()),
            selected: None,
            include_archived: false,
            clearable: false,
            placeholder,
            width: Length::Fill,
        }
    }

    #[must_use]
    pub fn all_decks(placeholder: &'static str) -> Self {
        Self {
            include_archived: true,
            ..Self::active_only(placeholder)
        }
    }

    #[must_use]
    pub const fn with_width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    #[must_use]
    pub const fn with_clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    #[must_use]
    pub const fn selected(&self) -> Option<PlayerId> {
        self.selected
    }

    pub const fn set_selected(&mut self, selected: Option<PlayerId>) {
        self.selected = selected;
    }

    pub const fn clear(&mut self) {
        self.selected = None;
    }

    pub fn sync(&mut self, tournament: &Tournament) {
        let mut options: Vec<DeckOption> = tournament
            .players()
            .iter()
            .filter(|(_, info)| self.include_archived || !info.is_archived())
            .map(|(id, info)| DeckOption {
                id: *id,
                label: info.display_name(),
            })
            .collect();
        options.sort_by_key(|o| o.label.to_lowercase());

        if self.combo.options() != options.as_slice() {
            self.combo = ComboState::new(options);
        }
    }
}

impl Model for DeckPicker {
    type Message = DeckPickerMsg;
    type OutMessage = Option<PlayerId>;
    type Context<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            DeckPickerMsg::Open => {
                self.sync(context);
                Signal::done()
            }
            DeckPickerMsg::Select(id) => {
                self.selected = Some(id);
                Signal::out(Some(id)).ok()
            }
            DeckPickerMsg::Clear => {
                self.selected = None;
                Signal::out(None).ok()
            }
        }
    }
}

impl Component for DeckPicker {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let selection = self.selected.and_then(|id| {
            context
                .get_player_display_name(&id)
                .map(|label| DeckOption { id, label })
        });

        let picker = combo_box(&self.combo, self.placeholder, selection.as_ref(), |opt| {
            DeckPickerMsg::Select(opt.id)
        })
        .on_open(DeckPickerMsg::Open)
        .size(13)
        .padding([8, 12])
        .width(self.width)
        .menu_height(Length::Fixed(320.0))
        .input_style(style::search_input);

        if self.clearable && self.selected.is_some() {
            row![
                picker,
                button(text(md::MD_CLOSE).size(15))
                    .padding([6, 8])
                    .style(style::icon_button)
                    .on_press(DeckPickerMsg::Clear),
            ]
            .spacing(4)
            .align_y(Vertical::Center)
            .into()
        } else {
            picker.into()
        }
    }
}

use commander_tournament_core::{
    player::{
        PlayerId,
        bracket::Bracket,
        color::{ColorIdentity, MtgColor},
        info::PlayerInfo,
    },
    tournament::Tournament,
};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, checkbox, column, container, row, rule, space, text, text_input},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{DeckPicker, DeckPickerMsg},
    fonts::{FONT_BOLD, FONT_SEMIBOLD},
    icons::color_icon,
    style::{self, COLORS},
};

#[derive(Debug, Clone)]
pub enum DeckEditorMsg {
    NameChanged(String),
    DescriptionChanged(String),
    ToggleColor(MtgColor),
    BracketChanged(Option<Bracket>),
    MoxfieldChanged(String),
    TogglePrecon(bool),
    ToggleArchived(bool),
    MergePicker(DeckPickerMsg),
    ConfirmMerge,
    RequestDelete,
    CancelDelete,
    ConfirmDelete,
    Submit,
    Close,
}

#[derive(Debug, Clone)]
pub enum DeckEditorOut {
    Save {
        editing_id: Option<PlayerId>,
        info: PlayerInfo,
    },
    Merge {
        target: PlayerId,
        other: PlayerId,
    },
    Delete(PlayerId),
    Close,
}

#[derive(Debug)]
pub struct DeckEditor {
    editing_id: Option<PlayerId>,
    name: String,
    description: String,
    color_identity: ColorIdentity,
    bracket: Option<Bracket>,
    moxfield_input: String,
    is_precon: bool,
    is_archived: bool,
    merge_picker: DeckPicker,
    confirm_delete: bool,
    error: Option<String>,
}

impl DeckEditor {
    #[must_use]
    pub fn new_deck() -> Self {
        Self {
            editing_id: None,
            name: String::new(),
            description: String::new(),
            color_identity: ColorIdentity::COLORLESS,
            bracket: None,
            moxfield_input: String::new(),
            is_precon: false,
            is_archived: false,
            merge_picker: DeckPicker::all_decks("Search deck to absorb into this deck..."),
            confirm_delete: false,
            error: None,
        }
    }

    #[must_use]
    pub fn from_existing(id: PlayerId, info: &PlayerInfo, tournament: &Tournament) -> Self {
        let mut merge_picker = DeckPicker::all_decks("Search deck to absorb into this deck...");
        merge_picker.sync(tournament);
        Self {
            editing_id: Some(id),
            name: info.name().clone(),
            description: info.description().clone(),
            color_identity: info.color_identity(),
            bracket: info.bracket(),
            moxfield_input: info.moxfield_id().cloned().unwrap_or_default(),
            is_precon: info.is_precon(),
            is_archived: info.is_archived(),
            merge_picker,
            confirm_delete: false,
            error: None,
        }
    }

    #[must_use]
    pub const fn editing_id(&self) -> Option<PlayerId> {
        self.editing_id
    }

    fn to_player_info(&self) -> PlayerInfo {
        let mut info = PlayerInfo::new(self.name.trim().to_owned())
            .with_description(self.description.trim().to_owned())
            .with_color_identity(self.color_identity)
            .with_bracket(self.bracket)
            .with_is_precon(self.is_precon)
            .with_is_archived(self.is_archived);

        let moxfield = self.moxfield_input.trim();
        if moxfield.is_empty() {
            info.clear_moxfield_id();
        } else {
            info.set_moxfield_id(moxfield.to_owned());
        }

        info
    }
}

impl Model for DeckEditor {
    type Message = DeckEditorMsg;
    type OutMessage = DeckEditorOut;
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
            DeckEditorMsg::NameChanged(name) => {
                self.name = name;
                self.error = None;
                Signal::done()
            }
            DeckEditorMsg::DescriptionChanged(desc) => {
                self.description = desc;
                Signal::done()
            }
            DeckEditorMsg::ToggleColor(color) => {
                if self.color_identity.has_color(color) {
                    self.color_identity.remove_color(color);
                } else {
                    self.color_identity.add_color(color);
                }
                Signal::done()
            }
            DeckEditorMsg::BracketChanged(bracket) => {
                self.bracket = bracket;
                Signal::done()
            }
            DeckEditorMsg::MoxfieldChanged(input) => {
                self.moxfield_input = input;
                Signal::done()
            }
            DeckEditorMsg::TogglePrecon(is_precon) => {
                self.is_precon = is_precon;
                Signal::done()
            }
            DeckEditorMsg::ToggleArchived(is_archived) => {
                self.is_archived = is_archived;
                Signal::done()
            }
            DeckEditorMsg::MergePicker(msg) => {
                let _ = self.merge_picker.update(msg, context)?;
                Signal::done()
            }
            DeckEditorMsg::ConfirmMerge => {
                if let (Some(target), Some(other)) = (self.editing_id, self.merge_picker.selected())
                    && target != other
                {
                    self.merge_picker.clear();
                    return Signal::out(DeckEditorOut::Merge { target, other }).ok();
                }
                Signal::done()
            }
            DeckEditorMsg::RequestDelete => {
                self.confirm_delete = true;
                Signal::done()
            }
            DeckEditorMsg::CancelDelete => {
                self.confirm_delete = false;
                Signal::done()
            }
            DeckEditorMsg::ConfirmDelete => {
                self.confirm_delete = false;
                if let Some(id) = self.editing_id {
                    return Signal::out(DeckEditorOut::Delete(id)).ok();
                }
                Signal::done()
            }
            DeckEditorMsg::Close => Signal::out(DeckEditorOut::Close).ok(),
            DeckEditorMsg::Submit => {
                let trimmed_name = self.name.trim().to_owned();
                if trimmed_name.is_empty() {
                    self.error = Some("Deck name cannot be empty.".to_owned());
                    return Signal::done();
                }

                if let Some(existing_id) = context.get_player_id(&trimmed_name)
                    && Some(existing_id) != self.editing_id
                {
                    self.error = Some(format!("A deck named \"{trimmed_name}\" already exists."));
                    return Signal::done();
                }

                Signal::out(DeckEditorOut::Save {
                    editing_id: self.editing_id,
                    info: self.to_player_info(),
                })
                .ok()
            }
        }
    }
}

impl Component for DeckEditor {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let heading = if self.editing_id.is_some() {
            "Edit Deck"
        } else {
            "Add New Deck"
        };

        let header = row![
            text(heading).font(FONT_BOLD).size(16).color(COLORS.text),
            space::horizontal(),
            button(text(md::MD_CLOSE).size(16))
                .padding([4, 6])
                .style(style::icon_button)
                .on_press(DeckEditorMsg::Close),
        ]
        .align_y(Vertical::Center);

        let name_field = column![
            text("Deck / Commander Name").size(12).color(COLORS.subtext1),
            text_input("e.g. Anim Pakal, Thousandth Moon", &self.name)
                .on_input(DeckEditorMsg::NameChanged)
                .size(13)
                .padding([8, 12])
                .style(style::search_input),
        ]
        .spacing(4)
        .width(Length::FillPortion(2));

        let moxfield_field = column![
            text("Moxfield URL or ID (optional)")
                .size(12)
                .color(COLORS.subtext1),
            text_input("https://moxfield.com/decks/...", &self.moxfield_input)
                .on_input(DeckEditorMsg::MoxfieldChanged)
                .size(13)
                .padding([8, 12])
                .style(style::search_input),
        ]
        .spacing(4)
        .width(Length::FillPortion(2));

        let desc_field = column![
            text("Description / Notes (optional)")
                .size(12)
                .color(COLORS.subtext1),
            text_input("Strategy, archetype, or notes...", &self.description)
                .on_input(DeckEditorMsg::DescriptionChanged)
                .size(13)
                .padding([8, 12])
                .style(style::search_input),
        ]
        .spacing(4)
        .width(Length::Fill);

        let color_picker =
            MtgColor::COLORS
                .into_iter()
                .fold(row![].spacing(6).align_y(Vertical::Center), |r, color| {
                    let selected = self.color_identity.has_color(color);
                    r.push(
                        button(color_icon(color))
                            .padding([6, 8])
                            .style(style::segment_button(selected))
                            .on_press(DeckEditorMsg::ToggleColor(color)),
                    )
                });

        let color_section = column![
            text(format!("Color Identity: {}", self.color_identity))
                .size(12)
                .color(COLORS.subtext1),
            color_picker,
        ]
        .spacing(6);

        let mut bracket_buttons = row![
            button(text("None").size(12))
                .padding([6, 10])
                .style(style::segment_button(self.bracket.is_none()))
                .on_press(DeckEditorMsg::BracketChanged(None)),
        ]
        .spacing(6)
        .align_y(Vertical::Center);

        for &b in Bracket::VARIANTS {
            bracket_buttons = bracket_buttons.push(
                button(text(format!("B{} · {}", b.number(), b.name())).size(12))
                    .padding([6, 10])
                    .style(style::segment_button(self.bracket == Some(b)))
                    .on_press(DeckEditorMsg::BracketChanged(Some(b))),
            );
        }

        let bracket_section = column![
            text("Commander Bracket").size(12).color(COLORS.subtext1),
            bracket_buttons.wrap(),
        ]
        .spacing(6);

        let flags_row = row![
            checkbox(self.is_precon)
                .label("Precon")
                .text_size(13)
                .on_toggle(DeckEditorMsg::TogglePrecon),
            checkbox(self.is_archived)
                .label("Archived (exclude from active pods)")
                .text_size(13)
                .on_toggle(DeckEditorMsg::ToggleArchived),
        ]
        .spacing(16)
        .align_y(Vertical::Center);

        let mut form = column![
            header,
            rule::horizontal(1),
            row![name_field, moxfield_field].spacing(12).width(Length::Fill),
            desc_field,
            row![color_section, space::horizontal().width(20), bracket_section]
                .spacing(12)
                .align_y(Vertical::Top)
                .wrap(),
            flags_row,
        ]
        .spacing(12)
        .width(Length::Fill);

        if let Some(err) = &self.error {
            form = form.push(text(err).size(12).color(COLORS.red));
        }

        if let Some(editing_id) = self.editing_id {
            let merge_source = self.merge_picker.selected();
            let can_merge = merge_source.is_some_and(|other| other != editing_id);

            let mut merge_btn = button(
                row![
                    text(md::MD_CALL_MERGE).size(14),
                    text("Merge Into This Deck").font(FONT_BOLD).size(12),
                ]
                .spacing(6)
                .align_y(Vertical::Center),
            )
            .padding([8, 12])
            .style(style::danger_button);

            if can_merge {
                merge_btn = merge_btn.on_press(DeckEditorMsg::ConfirmMerge);
            }

            let merge_hint: Element<'a, DeckEditorMsg> = match merge_source {
                Some(other) if other == editing_id => text("Choose a different deck to merge.")
                    .size(11)
                    .color(COLORS.yellow)
                    .into(),
                Some(other) => {
                    let other_name = context
                        .get_player_display_name(&other)
                        .unwrap_or_else(|| format!("Deck {other:?}"));
                    text(format!(
                        "Reassigns all matches from \"{other_name}\" into this deck and replays Elo chronologically."
                    ))
                    .size(11)
                    .color(COLORS.subtext1)
                    .into()
                }
                None => text("Absorb a duplicate or retired deck's match history into this deck.")
                    .size(11)
                    .color(COLORS.subtext0)
                    .into(),
            };

            let merge_section = column![
                rule::horizontal(1),
                text("Merge Another Deck Into This One")
                    .font(FONT_SEMIBOLD)
                    .size(13)
                    .color(COLORS.subtext1),
                row![
                    self.merge_picker.view(context).map(DeckEditorMsg::MergePicker),
                    merge_btn,
                ]
                .spacing(8)
                .align_y(Vertical::Center)
                .width(Length::Fill),
                merge_hint,
            ]
            .spacing(6)
            .width(Length::Fill);

            form = form.push(merge_section);
        }

        let save_label = if self.editing_id.is_some() {
            "Save Changes"
        } else {
            "Add Deck"
        };

        let mut footer = row![].spacing(8).align_y(Vertical::Center);

        if self.editing_id.is_some() {
            if self.confirm_delete {
                footer = footer
                    .push(text("Delete deck & its games?").size(12).color(COLORS.red))
                    .push(
                        button(text("Keep").size(12))
                            .padding([6, 10])
                            .style(style::secondary_button)
                            .on_press(DeckEditorMsg::CancelDelete),
                    )
                    .push(
                        button(text("Confirm Delete").font(FONT_BOLD).size(12))
                            .padding([6, 12])
                            .style(style::danger_button)
                            .on_press(DeckEditorMsg::ConfirmDelete),
                    );
            } else {
                footer = footer.push(
                    button(
                        row![
                            text(md::MD_TRASH_CAN_OUTLINE).size(14),
                            text("Delete Deck").size(13),
                        ]
                        .spacing(6)
                        .align_y(Vertical::Center),
                    )
                    .padding([8, 12])
                    .style(style::danger_button)
                    .on_press(DeckEditorMsg::RequestDelete),
                );
            }
        }

        let footer = footer
            .push(space::horizontal())
            .push(
                button(text("Cancel").size(13))
                    .padding([8, 14])
                    .style(style::secondary_button)
                    .on_press(DeckEditorMsg::Close),
            )
            .push(
                button(text(save_label).font(FONT_BOLD).size(13))
                    .padding([8, 14])
                    .style(style::primary_button)
                    .on_press(DeckEditorMsg::Submit),
            );

        container(form.push(footer))
            .width(Length::Fill)
            .padding(16)
            .style(style::card)
            .into()
    }
}

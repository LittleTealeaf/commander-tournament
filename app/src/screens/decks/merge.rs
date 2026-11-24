use commander_tournament_core::{player::PlayerId, tournament::Tournament};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, rule, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;

use crate::{
    components::{DeckPicker, DeckPickerMsg},
    fonts::{FONT_BOLD, FONT_SEMIBOLD},
    style::{self, COLORS},
};

#[derive(Debug, Clone)]
pub enum MergeDecksMsg {
    PrimaryPicker(DeckPickerMsg),
    SourcePicker(DeckPickerMsg),
    Confirm,
    Close,
}

#[derive(Debug, Clone)]
pub enum MergeDecksOut {
    Confirm { target: PlayerId, other: PlayerId },
    Close,
}

#[derive(Debug)]
pub struct MergeDecks {
    primary_picker: DeckPicker,
    source_picker: DeckPicker,
}

impl Default for MergeDecks {
    fn default() -> Self {
        Self {
            primary_picker: DeckPicker::all_decks("Search deck to keep..."),
            source_picker: DeckPicker::all_decks("Search duplicate deck..."),
        }
    }
}

impl MergeDecks {
    #[must_use]
    pub fn synced(tournament: &Tournament) -> Self {
        let mut merge = Self::default();
        merge.primary_picker.sync(tournament);
        merge.source_picker.sync(tournament);
        merge
    }
}

impl Model for MergeDecks {
    type Message = MergeDecksMsg;
    type OutMessage = MergeDecksOut;
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
            MergeDecksMsg::PrimaryPicker(msg) => {
                let _ = self.primary_picker.update(msg, context)?;
                Signal::done()
            }
            MergeDecksMsg::SourcePicker(msg) => {
                let _ = self.source_picker.update(msg, context)?;
                Signal::done()
            }
            MergeDecksMsg::Close => Signal::out(MergeDecksOut::Close).ok(),
            MergeDecksMsg::Confirm => {
                if let (Some(target), Some(other)) =
                    (self.primary_picker.selected(), self.source_picker.selected())
                    && target != other
                {
                    return Signal::out(MergeDecksOut::Confirm { target, other }).ok();
                }
                Signal::done()
            }
        }
    }
}

impl Component for MergeDecks {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let primary = self.primary_picker.selected();
        let source = self.source_picker.selected();

        let header = row![
            text("Merge Decks").font(FONT_BOLD).size(16).color(COLORS.text),
            space::horizontal(),
            button(text(md::MD_CLOSE).size(16))
                .padding([4, 6])
                .style(style::icon_button)
                .on_press(MergeDecksMsg::Close),
        ]
        .align_y(Vertical::Center);

        let primary_card = container(
            column![
                text("Primary Deck (Keep)")
                    .font(FONT_SEMIBOLD)
                    .size(13)
                    .color(COLORS.blue),
                self.primary_picker
                    .view(context)
                    .map(MergeDecksMsg::PrimaryPicker),
            ]
            .spacing(8),
        )
        .width(Length::FillPortion(1))
        .padding(12)
        .style(style::pod_seat_card(true, false));

        let source_card = container(
            column![
                text("Source Deck (Absorb & Remove)")
                    .font(FONT_SEMIBOLD)
                    .size(13)
                    .color(COLORS.red),
                self.source_picker.view(context).map(MergeDecksMsg::SourcePicker),
            ]
            .spacing(8),
        )
        .width(Length::FillPortion(1))
        .padding(12)
        .style(style::card);

        let can_merge = matches!((primary, source), (Some(a), Some(b)) if a != b);

        let preview: Element<'a, MergeDecksMsg> = if let (Some(p_id), Some(s_id)) = (primary, source) {
            if p_id == s_id {
                text("Select two different decks to merge.")
                    .size(12)
                    .color(COLORS.yellow)
                    .into()
            } else if let (Some(p), Some(s)) = (
                context.get_registered_player(p_id),
                context.get_registered_player(s_id),
            ) {
                let combined_games = p.stats().games() + s.stats().games();
                let combined_wins = p.stats().wins() + s.stats().wins();
                text(format!(
                    "Reassigns all matches from \"{}\" to \"{}\" ({combined_games} combined games, {combined_wins} combined wins) and replays Elo chronologically.",
                    s.info().display_name(),
                    p.info().display_name(),
                ))
                .size(12)
                .color(COLORS.subtext1)
                .into()
            } else {
                text("Selected deck no longer exists.")
                    .size(12)
                    .color(COLORS.red)
                    .into()
            }
        } else {
            text("All historical matches referencing the source deck will be reassigned to the primary deck and replayed.")
                .size(12)
                .color(COLORS.subtext0)
                .into()
        };

        let mut confirm_btn = button(text("Confirm Merge").font(FONT_BOLD).size(13))
            .padding([8, 14])
            .style(style::danger_button);
        if can_merge {
            confirm_btn = confirm_btn.on_press(MergeDecksMsg::Confirm);
        }

        let footer = row![
            space::horizontal(),
            button(text("Cancel").size(13))
                .padding([8, 14])
                .style(style::secondary_button)
                .on_press(MergeDecksMsg::Close),
            confirm_btn,
        ]
        .spacing(8)
        .align_y(Vertical::Center);

        container(
            column![
                header,
                rule::horizontal(1),
                row![primary_card, source_card].spacing(12).width(Length::Fill),
                preview,
                footer,
            ]
            .spacing(12)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(16)
        .style(style::card)
        .into()
    }
}

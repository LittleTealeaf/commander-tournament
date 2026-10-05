use commander_tournament_core::player::{bracket::Bracket, color::MtgColor};
use iced::{
    Length,
    alignment::Vertical,
    widget::{button, checkbox, column, container, pick_list, row, space, text, text_editor, text_input},
};
use nerd_font_symbols::md::MD_LINK_VARIANT;
use strum::VariantArray;

use crate::{
    icons::color_icon,
    views::player::{PlayerDetailsMsg, PlayerView},
};

pub fn view_info_panel(state: &PlayerView) -> iced::widget::Container<'_, super::PlayerDetailsMsg> {
    let edit_name = text_input("Player Name...", state.info.name()).on_input(PlayerDetailsMsg::SetName);

    let edit_description = text_editor(&state.description)
        .placeholder("Description...")
        .on_action(PlayerDetailsMsg::EditDescription);

    let edit_moxfieldid =
        text_input("Moxfield ID", &state.moxfield_id).on_input(PlayerDetailsMsg::SetMoxfieldId);

    let deck_colors = row(MtgColor::COLORS.map(|color| {
        let style = if state.info.color_identity().has_color(color) {
            button::primary
        } else {
            button::secondary
        };

        button(color_icon(color).width(20).height(20))
            .on_press(PlayerDetailsMsg::ToggleColor(color))
            .style(style)
            .into()
    }))
    .spacing(5);

    let checkbox_archived = checkbox(state.info.is_archived())
        .label("Archived")
        .on_toggle(PlayerDetailsMsg::SetArchived);

    let checkbox_precon = checkbox(state.info.is_precon())
        .label("Precon")
        .on_toggle(PlayerDetailsMsg::SetIsPrecon);

    let button_link =
        button(MD_LINK_VARIANT).on_press_maybe(state.info.moxfield_link().map(PlayerDetailsMsg::OpenLink));

    let text_identity = text(state.info.color_identity().to_string());

    let bracket = {
        let row = row![
            text("Bracket"),
            pick_list(
                Bracket::VARIANTS,
                state.info.bracket(),
                PlayerDetailsMsg::SetBracket,
            )
        ]
        .align_y(Vertical::Center)
        .spacing(10);

        if state.info.bracket().is_none() {
            row.push(button("Clear").on_press(PlayerDetailsMsg::ClearBracket))
        } else {
            row
        }
    };

    container(
        column![
            row![edit_name, checkbox_archived]
                .spacing(20)
                .align_y(Vertical::Center),
            row![edit_moxfieldid, button_link]
                .spacing(20)
                .align_y(Vertical::Center),
            row![
                deck_colors,
                text_identity,
                space().width(Length::Fill),
                checkbox_precon
            ]
            .spacing(20)
            .align_y(Vertical::Center),
            bracket,
            edit_description,
        ]
        .spacing(20),
    )
}

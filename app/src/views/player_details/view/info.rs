use edh_tourn::player::color::MtgColor;
use iced::widget::{button, column, container, row, text, text_editor, text_input};
use nerd_font_symbols::md::MD_LINK_VARIANT;

use crate::views::player_details::{Message, State};

pub fn view_info_panel(state: &State) -> iced::widget::Container<'_, super::Message> {
    let edit_name = text_input("Player Name...", state.info.name()).on_input(Message::SetName);

    let edit_description = text_editor(&state.description)
        .placeholder("Description...")
        .on_action(Message::EditDescription);

    let edit_moxfieldid =
        text_input("Moxfield ID", &state.moxfield_id).on_input(Message::SetMoxfieldId);

    let deck_colors = row(MtgColor::COLORS.map(|color| {
        let style = if state.info.color_identity().has_color(color) {
            button::primary
        } else {
            button::secondary
        };

        button(color.letter())
            .on_press(Message::ToggleColor(color))
            .style(style)
            .into()
    }))
    .spacing(5);

    let button_link =
        button(MD_LINK_VARIANT).on_press_maybe(state.info.moxfield_link().map(Message::OpenLink));

    let text_identity = text(state.info.color_identity().to_string());

    container(
        column![
            row![edit_name, text_identity].spacing(20),
            row![edit_moxfieldid, button_link, deck_colors].spacing(20),
            edit_description,
        ]
        .spacing(20),
    )
}

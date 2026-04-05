use edh_tourn::player::color::MtgColor;
use iced::{
    alignment::Horizontal,
    widget::{button, column, container, row, text, text_editor, text_input},
};
use nerd_font_symbols::md::{MD_LINK_VARIANT, MD_SWORD};

use crate::player_details::{PlayerDetails, PlayerDetailsMsg};

pub fn view_info_panel(
    state: &PlayerDetails,
) -> iced::widget::Container<'_, super::PlayerDetailsMsg> {
    let edit_name =
        text_input("Player Name...", state.info.name()).on_input(PlayerDetailsMsg::SetName);

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

        button(color.letter())
            .on_press(PlayerDetailsMsg::ToggleColor(color))
            .style(style)
            .into()
    }))
    .spacing(5);

    let button_link = button(MD_LINK_VARIANT)
        .on_press_maybe(state.info.moxfield_link().map(PlayerDetailsMsg::OpenLink));

    let text_identity = text(state.info.color_identity().to_string());

    container(
        column![
            row![edit_name, text_identity].spacing(20),
            row![edit_moxfieldid, button_link, deck_colors].spacing(20),
            edit_description,
            state.id.is_some().then(|| container(
                button(text(format!("{MD_SWORD} Open Next Match")))
                    .on_press(PlayerDetailsMsg::OpenNextPlayerMatch)
            )
            .align_x(Horizontal::Center))
        ]
        .spacing(20),
    )
}

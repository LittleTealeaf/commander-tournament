use edh_tourn::player::color::MtgColor;
use iced::widget::{button, column, container, row, text, text_editor, text_input};

use crate::{
    logic::Message,
    view::player::{ViewPlayerMessage, ViewPlayerScene},
};

pub fn view_info_panel(scene: &ViewPlayerScene) -> iced::widget::Container<'_, Message> {
    let edit_name = text_input("Player Name...", scene.info.name())
        .on_input(|text| ViewPlayerMessage::SetName(text).into());

    let edit_description = text_editor(&scene.edit_description)
        .placeholder("Description...")
        .on_action(|action| ViewPlayerMessage::EditDescription(action).into());

    let edit_moxfieldid = text_input("Moxfield ID", &scene.moxfield)
        .on_input(|text| ViewPlayerMessage::SetMoxfieldId(text).into());

    let deck_colors = row(MtgColor::COLORS.map(|color| {
        let style = if scene.info.color_identity().has_color(color) {
            button::primary
        } else {
            button::secondary
        };

        button(color.letter())
            .on_press(ViewPlayerMessage::ToggleColor(color).into())
            .style(style)
            .into()
    }))
    .spacing(5);

    let button_link = button("").on_press_maybe(scene.info.moxfield_link().map(Message::OpenLink));

    let text_identity = text(scene.info.color_identity().to_string());

    container(
        column![
            row![edit_name, text_identity].spacing(20),
            row![edit_moxfieldid, button_link, deck_colors].spacing(20),
            edit_description,
        ]
        .spacing(20),
    )
}

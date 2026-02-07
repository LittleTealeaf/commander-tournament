use iced::{
    Length, Padding,
    widget::{button, container, row, space},
};

use crate::ui::Message;

pub fn toolbar() -> iced::widget::Container<'static, Message> {
    let left_buttons = row![
        button(" ⚙ Config").on_press(Message::ShowConfig(true)),
        button(" ➕ New Player")
            .on_press(Message::SetChangePlayerName(Some((None, String::new())))),
        button(" 🔄 Reload").on_press(Message::Reload),
    ]
    .spacing(10);

    let right_buttons = row![
        button(" 📥 Ingest").on_press(Message::Ingest),
        button(" ✨ New").on_press(Message::New),
        button(" 📂 Load").on_press(Message::Load(String::from("game.ron"))),
        button(" 💾 Save").on_press(Message::Save(String::from("game.ron"))),
    ]
    .spacing(10);

    let toolbar_row = row![
        left_buttons,
        space().width(Length::Fill),
        right_buttons,
    ]
    .width(Length::Fill)
    .spacing(15);

    container(toolbar_row)
        .padding(Padding::new(10f32))
}

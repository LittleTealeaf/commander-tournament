use iced::{
    Length,
    alignment::Horizontal,
    widget::{button, center, column, row, space, text},
};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::Scene,
};

#[derive(Debug)]
pub struct ConfirmPrompt {
    title: String,
    text: String,
    on_confirm: Message,
}

impl ConfirmPrompt {
    #[must_use]
    pub const fn new(title: String, text: String, on_confirm: Message) -> Self {
        Self { title, text, on_confirm }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmPromptMessage {
    Confirm,
    Deny,
}

impl From<ConfirmPromptMessage> for Message {
    fn from(value: ConfirmPromptMessage) -> Self {
        Self::ConfirmationPrompt(value)
    }
}

impl HandleMessage<ConfirmPromptMessage> for App {
    fn update(&mut self, msg: ConfirmPromptMessage) -> anyhow::Result<iced::Task<Message>> {
        let Some(Scene::Confirm(confirm)) = self.scenes.last() else {
            return Message::done();
        };

        match msg {
            ConfirmPromptMessage::Confirm => {
                let msg = confirm.on_confirm.clone();
                self.scenes.pop();
                self.update(msg)
            }
            ConfirmPromptMessage::Deny => {
                self.scenes.pop();
                Message::done()
            }
        }
    }
}

impl View<ConfirmPrompt> for App {
    fn view<'a>(&'a self, scene: &'a ConfirmPrompt) -> iced::Element<'a, Message> {
        center(
            column![
                text(&scene.title).size(25),
                text(&scene.text),
                row![
                    button(text("No").align_x(Horizontal::Center))
                        .on_press(ConfirmPromptMessage::Deny.into())
                        .width(200),
                    space().width(Length::Fill),
                    button(text("Yes").align_x(Horizontal::Center))
                        .on_press(ConfirmPromptMessage::Confirm.into())
                        .width(200)
                ]
                .width(Length::Fill)
            ]
            .spacing(20)
            .width(Length::Shrink),
        )
        .into()
    }
}

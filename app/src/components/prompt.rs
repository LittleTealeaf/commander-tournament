use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text},
};

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

#[derive(Clone, Debug, derive_more::Constructor)]
pub struct DialogPrompt {
    title: String,
    details: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Accept,
    Cancel,
}

impl Component for DialogPrompt {
    type Message = Message;
    type OutMessage = Message;
}

impl ComponentUpdate for DialogPrompt {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        Effect::Out(message).ok()
    }
}

impl ComponentView for DialogPrompt {
    type ViewContext<'a> = ();
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let title = text(&self.title).size(24);
        let details = text(&self.details);

        let buttons = row![
            button("Cancel").on_press(Message::Cancel),
            button("Accept").on_press(Message::Accept),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        let content = column![title, details, buttons]
            .spacing(20)
            .align_x(Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

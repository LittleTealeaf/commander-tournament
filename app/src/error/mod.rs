use iced::{
    Alignment, Length,
    alignment::Horizontal,
    widget::{button, column, container, text},
};

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct State {
    message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CloseError,
}

impl Component for State {
    type OutMessage = Message;
    type Message = Message;
}

impl ComponentUpdate for State {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(message)
    }
}

impl ComponentView for State {
    type ViewContext<'a> = ();
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let title = text("An Error Occurred").size(24);
        let message = text(&self.message).size(16).align_x(Horizontal::Center);

        let close_button = button(text("Close").align_x(Horizontal::Center))
            .padding([8, 16])
            .on_press(Message::CloseError);

        container(
            column![title, message, close_button]
                .spacing(20)
                .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding(20)
        .into()
    }
}

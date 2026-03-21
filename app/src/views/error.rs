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

#[derive(Debug)]
pub enum OutMessage {
    Close,
}

impl Component for State {
    type OutMessage = OutMessage;
    type Context<'a> = ();
    type Message = Message;
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::CloseError => Effect::out(OutMessage::Close),
        }
    }
}

impl ComponentView for State {
    fn view<'a>(&'a self, (): Self::Context<'a>) -> iced::Element<'a, Self::Message> {
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

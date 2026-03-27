use iced::{
    Alignment, Length,
    alignment::Horizontal,
    widget::{button, column, container, text},
};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Error {
    message: String,
}

#[derive(Debug, Clone)]
pub enum ErrorMsg {
    CloseError,
}

impl Component for Error {
    type OutMessage = ErrorMsg;
    type Message = ErrorMsg;
}

impl ComponentUpdate for Error {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        Effect::Out(message).ok()
    }
}

impl ComponentView for Error {
    type ViewContext<'a> = ();
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let title = text("An Error Occurred").size(24);
        let message = text(&self.message).size(16).align_x(Horizontal::Center);

        let close_button = button(text("Close").align_x(Horizontal::Center))
            .padding([8, 16])
            .on_press(ErrorMsg::CloseError);

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

use iced::{
    Length,
    widget::{container, text},
};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView, ViewScreen},
};

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct ErrorView {
    message: String,
}

#[derive(Clone, Debug)]
pub enum ErrorMsg {
    Close,
}

impl Component for ErrorView {
    type OutMessage = ErrorMsg;
    type Message = ErrorMsg;
}

impl ComponentUpdate for ErrorView {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        _: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        Effect::done()
    }
}

impl ComponentView for ErrorView {
    type ViewContext<'a> = ();
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        container(text(&self.message)).width(Length::Fill).into()
    }
}

impl ViewScreen for ErrorView {
    const CLOSE_MESSAGE: Self::Message = ErrorMsg::Close;
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Application Error".to_owned()
    }
}

use iced::{
    Length,
    widget::{container, text},
};

use crate::{
    app::ViewMsg,
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView, ViewScreen},
};

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct ErrorView {
    message: String,
}

#[derive(Clone, Debug)]
pub struct ErrorMsg;

impl From<ErrorMsg> for ViewMsg {
    fn from(_: ErrorMsg) -> Self {
        Self::Close
    }
}

impl Component for ErrorView {
    type OutMessage = ();
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
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Application Error".to_owned()
    }
}

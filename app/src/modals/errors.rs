use core::iter::once;

use iced::widget::{button, text};

use crate::{
    effect::Effect,
    modals::ModalType,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Debug)]
pub struct ErrorModal {
    error: String,
}

#[derive(Debug, Clone)]
pub struct ErrorMsg;

#[derive(Debug, Clone)]
pub struct ErrorOut;

impl ErrorModal {
    pub fn new<E>(error: E) -> Self
    where
        E: Into<String>,
    {
        Self { error: error.into() }
    }
}

impl Component for ErrorModal {
    type Message = ErrorMsg;
    type OutMessage = ();
}

impl ComponentUpdate for ErrorModal {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        _: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(()).ok()
    }
}

impl ComponentView for ErrorModal {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;

    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        text(&self.error).into()
    }
}

impl ModalType for ErrorModal {
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        "Error".to_owned()
    }

    fn options<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl Iterator<Item = iced::widget::Button<'a, Self::Message>> {
        once(button("Close").on_press(ErrorMsg))
    }
}

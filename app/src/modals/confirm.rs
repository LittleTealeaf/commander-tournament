use iced::widget::{button, text};

use crate::{
    app::Message,
    effect::Effect,
    modals::ModalType,
    traits::{Component, ComponentUpdate, ComponentView, Mapped},
};

#[derive(Debug)]
pub struct ConfirmModal<M> {
    title: String,
    details: String,
    on_confirm: M,
    on_cancel: Option<M>,
}

impl<M> ConfirmModal<M> {
    pub fn new<T, D, C>(title: T, details: D, confirm: M, cancel: C) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        C: Into<Option<M>>,
    {
        Self {
            title: title.into(),
            details: details.into(),
            on_confirm: confirm,
            on_cancel: cancel.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConfirmMsg {
    Confirm,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum ConfirmOut<M>
where
    M: Clone,
{
    Close(Option<M>),
}

impl<M, N> Mapped<ConfirmModal<N>> for ConfirmModal<M>
where
    M: Into<N>,
{
    fn map(self) -> ConfirmModal<N> {
        ConfirmModal {
            title: self.title,
            details: self.details,
            on_confirm: self.on_confirm.into(),
            on_cancel: self.on_cancel.map(Into::into),
        }
    }
}

impl<M> Component for ConfirmModal<M>
where
    M: Clone,
{
    type Message = ConfirmMsg;
    type OutMessage = Option<M>;
}

impl<M> ComponentUpdate for ConfirmModal<M>
where
    M: Clone,
{
    type UpdateContext<'a> = ();

    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(match message {
            ConfirmMsg::Confirm => Some(self.on_confirm.clone()),
            ConfirmMsg::Cancel => self.on_cancel.clone(),
        })
        .ok()
    }
}

impl ModalType for ConfirmModal<Message> {
    fn title<'a>(&'a self, (): Self::ViewContext<'a>) -> String {
        self.title.clone()
    }

    fn options<'a>(
        &'a self,
        (): Self::ViewContext<'a>,
    ) -> impl Iterator<Item = iced::widget::Button<'a, Self::Message>> {
        [
            button("Cancel").on_press(ConfirmMsg::Cancel),
            button("Confirm").on_press(ConfirmMsg::Confirm),
        ]
        .into_iter()
    }
}

impl ComponentView for ConfirmModal<Message> {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        text(&self.details).into()
    }
}

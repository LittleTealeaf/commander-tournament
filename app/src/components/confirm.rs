use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text},
};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Clone, Debug, derive_more::Constructor)]
pub struct ConfirmDialog<T> {
    title: String,
    description: String,
    confirm: T,
    cancel: Option<T>,
}

impl<T> ConfirmDialog<T> {
    pub fn map<I>(self) -> ConfirmDialog<I>
    where
        T: Into<I>,
    {
        ConfirmDialog {
            title: self.title,
            description: self.description,
            confirm: self.confirm.into(),
            cancel: self.cancel.map(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConfirmDialogMsg {
    Confirm,
    Cancel,
}

#[derive(Debug)]
pub enum ConfirmDialogOut<T> {
    Message(T),
    Close,
}

impl<T> Component for ConfirmDialog<T> {
    type Message = ConfirmDialogMsg;
    type OutMessage = ConfirmDialogOut<T>;
}

impl<T> ComponentUpdate for ConfirmDialog<T>
where
    T: Clone,
{
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        // Close needs to happen first so that the resulting messages are not caught up in this
        // view
        Effect::out(ConfirmDialogOut::Close)
            .chain(match message {
                ConfirmDialogMsg::Confirm => {
                    Effect::out(ConfirmDialogOut::Message(self.confirm.clone()))
                }
                ConfirmDialogMsg::Cancel => self
                    .cancel
                    .clone()
                    .map(ConfirmDialogOut::Message)
                    .map(Effect::out)
                    .unwrap_or_default(),
            })
            .ok()
    }
}

impl<T> ComponentView for ConfirmDialog<T> {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;

    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let title = text(&self.title).size(24);
        let details = text(&self.description);

        let buttons = row![
            button("Cancel").on_press(ConfirmDialogMsg::Cancel),
            button("Accept").on_press(ConfirmDialogMsg::Confirm),
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

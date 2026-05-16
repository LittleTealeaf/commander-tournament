pub mod confirm;
pub mod errors;
pub mod player_filter;

use core::iter::once;

use edh_tourn::tournament::Tournament;
use iced::{
    Color, Element, Length,
    widget::{Button, center, column, container, opaque, row, space, stack, text},
};

use crate::{
    app::Message,
    effect::Effect,
    fonts::FONT_BOLD,
    modals::{
        confirm::{ConfirmModal, ConfirmMsg},
        errors::{ErrorModal, ErrorMsg},
    },
    traits::{Component, ComponentUpdate, ComponentView, Mapped},
};

#[derive(Debug, derive_more::From)]
pub enum Modal<M> {
    Confirm(ConfirmModal<M>),
    Error(ErrorModal),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ModalMsg {
    Confirm(ConfirmMsg),
    Error(ErrorMsg),
}

impl<M, N> Mapped<Modal<N>> for Modal<M>
where
    M: Into<N>,
{
    fn map(self) -> Modal<N> {
        match self {
            Self::Confirm(confirm) => Modal::Confirm(confirm.map()),
            Self::Error(modal) => Modal::Error(modal),
        }
    }
}

impl Component for Modal<Message> {
    type OutMessage = Message;
    type Message = ModalMsg;
}

impl ComponentUpdate for Modal<Message> {
    type UpdateContext<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        _: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match (self, message) {
            (Self::Confirm(modal), ModalMsg::Confirm(msg)) => modal.map_update(msg, (), |msg| {
                Effect::out(Message::CloseModal)
                    .chain(msg.map(Effect::out).unwrap_or_default())
                    .ok()
            }),
            (Self::Error(_), ModalMsg::Error(_)) => Effect::out(Message::CloseModal).ok(),
            (_, _) => Effect::done(),
        }
    }
}

impl Modal<Message> {
    pub fn overlay<'a, Content>(&'a self, app_content: Content, _: &'a Tournament) -> Element<'a, Message>
    where
        Content: Into<Element<'a, Message>>,
    {
        let content = match self {
            Self::Confirm(modal) => modal.render(()).map(ModalMsg::from),
            Self::Error(modal) => modal.render(()).map(ModalMsg::from),
        }
        .map(Into::into);

        let content_box = container(content);
        let bounding_box = container(content_box)
            .width(Length::Shrink)
            .padding(10)
            .style(container::rounded_box);

        let shadow = center(bounding_box).style(|style| container::Style {
            background: Some(
                Color {
                    a: 0.5,
                    ..style.palette().background
                }
                .into(),
            ),
            ..container::Style::default()
        });

        let overlay = opaque(shadow);

        stack![app_content.into(), overlay].into()
    }
}

trait ModalType: ComponentView + Component {
    fn title<'a>(&'a self, context: Self::ViewContext<'a>) -> String;
    fn options<'a>(
        &'a self,
        context: Self::ViewContext<'a>,
    ) -> impl Iterator<Item = Button<'a, Self::Message>>;

    fn render<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::Message>
    where
        Self::ViewContext<'a>: Clone,
    {
        let content = self.view(context.clone());
        let title = text(self.title(context.clone())).size(30).font(FONT_BOLD);
        let buttons =
            row(once(space().width(Length::Fill).into()).chain(self.options(context).map(Into::into)))
                .spacing(10);

        column![title, content, buttons].spacing(10).into()
    }
}

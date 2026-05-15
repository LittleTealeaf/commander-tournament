pub mod confirm;
pub mod errors;

use edh_tourn::tournament::Tournament;
use iced::{
    Element, Padding,
    widget::{center, opaque, stack, text},
};

use crate::{
    App,
    app::Message,
    modals::confirm::{ConfirmModal, ConfirmMsg},
    traits::{Component, ComponentUpdate, ComponentView, Mapped},
};

#[derive(Debug, derive_more::From)]
pub enum Modal<M> {
    Confirm(ConfirmModal<M>),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ModalMsg {
    Confirm(ConfirmMsg),
}

impl<M, N> Mapped<Modal<N>> for Modal<M>
where
    M: Into<N>,
{
    fn map(self) -> Modal<N> {
        match self {
            Self::Confirm(confirm) => Modal::Confirm(confirm.map()),
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
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        todo!()
    }
}

impl Modal<Message> {
    pub fn overlay<'a, Content>(
        &'a self,
        app_content: Content,
        context: &'a Tournament,
    ) -> Element<'a, Message>
    where
        Content: Into<Element<'a, Message>>,
    {
        let overlay = opaque(
            center(match self {
                Self::Confirm(modal) => modal.view(()),
            })
            .padding(Padding::new(10.0)),
        )
        .map(ModalMsg::from)
        .map(Into::into);

        stack![app_content.into(), overlay].into()
    }
}

// impl<M> ComponentUpdate<M> for Modal<M> where M: Clone {
//     type UpdateContext<'a> = &'a App;
//
//     fn update(
//         &mut self,
//         message: Self::Message,
//         context: Self::UpdateContext<'_>,
//     ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>>
//     {
//         match (self, message) {
//             (Self::Confirm(modal), ModalMessage::Confirm(msg)) => modal.map_update(msg, (), |msg| {
//
//             })
//
//             _ => Effect::done()
//         }
//     }
// }
//
//
// #[derive(Debug)]
// pub enum ModalOld<M> {
//     Confirm {
//         title: String,
//         details: String,
//         on_confirm: M,
//         on_cancel: Option<M>,
//     },
//     Error {
//         error: String,
//     },
// }
//
// impl<M> ModalOld<M> {
//     pub fn confirm<C>(title: String, details: String, on_confirm: M, on_cancel: C) -> Self
//     where
//         C: Into<Option<M>>,
//     {
//         Self::Confirm {
//             title,
//             details,
//             on_confirm,
//             on_cancel: on_cancel.into(),
//         }
//     }
//
//     pub fn map<MT>(self) -> ModalOld<MT>
//     where
//         M: Into<MT>,
//     {
//         match self {
//             Self::Confirm {
//                 title,
//                 details,
//                 on_confirm,
//                 on_cancel,
//             } => ModalOld::Confirm {
//                 title,
//                 details,
//                 on_confirm: on_confirm.into(),
//                 on_cancel: on_cancel.map(Into::into),
//             },
//             Self::Error { error } => ModalOld::Error { error },
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// pub enum ModalOldMsg {
//     Confirm,
//     Close,
//     Cancel,
// }
//
// impl Component for ModalOld<Message> {
//     type Message = ModalOldMsg;
//     type OutMessage = Message;
// }
//
// impl ComponentUpdate for ModalOld<Message> {
//     type UpdateContext<'a> = ();
//     fn update(
//         &mut self,
//         message: Self::Message,
//         (): Self::UpdateContext<'_>,
//     ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
//         match (self, message) {
//             (Self::Confirm { on_confirm, .. }, ModalOldMsg::Confirm) => Effect::out(on_confirm.clone())
//                 .chain(Effect::out(Message::CloseModal))
//                 .ok(),
//             (Self::Confirm { on_cancel, .. }, ModalOldMsg::Cancel) => on_cancel
//                 .clone()
//                 .map_or(Effect::Done, Effect::out)
//                 .chain(Effect::out(Message::CloseModal))
//                 .ok(),
//
//             (_, ModalOldMsg::Close) => Effect::out(Message::CloseModal).ok(),
//             (_, _) => Effect::done(),
//         }
//     }
// }

// impl<M> Modal<M> {
//     pub fn overlay<'a, Content>(&'a self, content: Content) -> Element<'a, M>
//     where
//         M: 'a + Clone,
//         Content: Into<Element<'a, M>>,
//         Modal: Into<M>,
//     {
//         todo!()
//         // stack![
//         //     content.into(),
//         //     opaque(
//         //         center(
//         //             container(
//         //                 container(match self {
//         //                     Self::Confirm { title, details, .. } => {
//         //                         column![
//         //                             text(title).font(FONT_BOLD).size(20),
//         //                             text(details).wrapping(text::Wrapping::Word).width(Length::Fill),
//         //                             row![
//         //                                 space().width(Length::Fill),
//         //                                 button("Cancel").on_press(ModalOldMsg::Cancel.into()),
//         //                                 button("Confirm").on_press(ModalOldMsg::Confirm.into()),
//         //                             ]
//         //                             .spacing(10)
//         //                         ]
//         //                         .spacing(10)
//         //                         .padding(10)
//         //                     }
//         //                     Self::Error { error } => {
//         //                         column![
//         //                             text("Application Error").font(FONT_BOLD).size(20),
//         //                             text(error).wrapping(text::Wrapping::Word).width(Length::Fill),
//         //                             container(button("Close").on_press(ModalOldMsg::Close.into()))
//         //                                 .align_x(Horizontal::Right)
//         //                                 .width(Length::Fill)
//         //                         ]
//         //                         .spacing(10)
//         //                         .padding(10)
//         //                     }
//         //                 })
//         //                 .width(250)
//         //                 .max_width(500)
//         //             )
//         //             .width(Length::Shrink)
//         //             .padding(10)
//         //             .style(container::rounded_box)
//         //         )
//         //         .style(|style| {
//         //             container::Style {
//         //                 background: Some(
//         //                     Color {
//         //                         a: 0.5,
//         //                         ..style.palette().background
//         //                     }
//         //                     .into(),
//         //                 ),
//         //                 ..container::Style::default()
//         //             }
//         //         })
//         //     )
//         // ]
//         // .into()
//     }
// }

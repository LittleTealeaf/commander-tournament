use iced::{
    Color, Element, Length,
    alignment::Horizontal,
    font,
    widget::{button, center, column, container, opaque, row, space, stack, text},
};

use crate::{
    app::Message,
    effect::Effect,
    style::font_default,
    traits::{Component, ComponentUpdate},
};

#[derive(Debug)]
pub enum Modal<M> {
    Confirm {
        title: String,
        details: String,
        on_confirm: M,
        on_cancel: Option<M>,
    },
    Error {
        error: String,
    },
}

impl<M> Modal<M> {
    pub fn confirm<T, D, C>(title: &T, details: &D, on_confirm: M, on_cancel: C) -> Self
    where
        T: ToString,
        D: ToString,
        C: Into<Option<M>>,
    {
        Self::Confirm {
            title: title.to_string(),
            details: details.to_string(),
            on_confirm,
            on_cancel: on_cancel.into(),
        }
    }

    pub fn map<MT>(self) -> Modal<MT>
    where
        M: Into<MT>,
    {
        match self {
            Self::Confirm {
                title,
                details,
                on_confirm,
                on_cancel,
            } => Modal::Confirm {
                title,
                details,
                on_confirm: on_confirm.into(),
                on_cancel: on_cancel.map(Into::into),
            },
            Self::Error { error } => Modal::Error { error },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModalMsg {
    Confirm,
    Close,
    Cancel,
}

impl Component for Modal<Message> {
    type Message = ModalMsg;
    type OutMessage = Message;
}

impl ComponentUpdate for Modal<Message> {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match (self, message) {
            (Self::Confirm { on_confirm, .. }, ModalMsg::Confirm) => {
                Effect::out(on_confirm.clone())
                    .chain(Effect::out(Message::CloseModal))
                    .ok()
            }
            (Self::Confirm { on_cancel, .. }, ModalMsg::Cancel) => on_cancel
                .clone()
                .map_or(Effect::Done, Effect::out)
                .chain(Effect::out(Message::CloseModal))
                .ok(),

            (_, ModalMsg::Close) => Effect::out(Message::CloseModal).ok(),
            (_, _) => Effect::done(),
        }
    }
}

impl<M> Modal<M> {
    pub fn overlay<'a, Content>(&'a self, content: Content) -> Element<'a, M>
    where
        M: 'a + Clone,
        Content: Into<Element<'a, M>>,
        ModalMsg: Into<M>,
    {
        stack![
            content.into(),
            opaque(
                center(
                    container(
                        container(match self {
                            Self::Confirm { title, details, .. } => {
                                column![
                                    text(title)
                                        .font(font::Font {
                                            weight: font::Weight::Bold,
                                            ..font_default()
                                        })
                                        .size(20),
                                    text(details)
                                        .wrapping(text::Wrapping::Word)
                                        .width(Length::Fill),
                                    row![
                                        space().width(Length::Fill),
                                        button("Cancel").on_press(ModalMsg::Cancel.into()),
                                        button("Confirm").on_press(ModalMsg::Confirm.into()),
                                    ]
                                    .spacing(10)
                                ]
                            }
                            Self::Error { error } => {
                                column![
                                    text("Application Error")
                                        .font(font::Font {
                                            weight: font::Weight::Bold,
                                            ..font_default()
                                        })
                                        .size(20),
                                    text(error)
                                        .wrapping(text::Wrapping::Word)
                                        .width(Length::Fill),
                                    container(button("Close").on_press(ModalMsg::Close.into()))
                                        .align_x(Horizontal::Right)
                                        .width(Length::Fill)
                                ]
                                .spacing(10)
                            }
                        })
                        .width(250)
                        .max_width(500)
                    )
                    .width(Length::Shrink)
                    .padding(10)
                    .style(container::rounded_box)
                )
                .style(|style| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.5,
                                ..style.palette().background
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                })
            )
        ]
        .into()
    }
}

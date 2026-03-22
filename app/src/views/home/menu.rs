use std::path::PathBuf;

use iced::widget::{button, row};

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

#[derive(Debug, Clone, Default)]
pub struct State;

#[derive(Debug, Clone)]
pub enum Message {
    New,
    Open,
    Save,
    SaveAs,
}

impl Component for State {
    type Message = Message;
    type OutMessage = Message;
    type Context<'a> = &'a Option<PathBuf>;
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        message: Self::Message,
        _: Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(message)
    }
}

impl ComponentView for State {
    fn view<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        row![
            button("New").on_press(Message::New),
            button("Open").on_press(Message::Open),
            button("SaveAs").on_press(Message::SaveAs),
            button("Save").on_press_maybe(context.is_some().then_some(Message::Save)),
        ]
        .spacing(5)
        .into()
    }
}

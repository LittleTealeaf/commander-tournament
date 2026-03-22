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
}

impl ComponentUpdate for State {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(message)
    }
}

impl ComponentView for State {
    type ViewContext<'a>
        = &'a Option<PathBuf>
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
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

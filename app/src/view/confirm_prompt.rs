use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::Scene,
};

pub struct ConfirmPrompt {
    text: String,
    on_confirm: Message,
}

impl ConfirmPrompt {
    pub fn new(text: String, on_confirm: Message) -> Self {
        Self { text, on_confirm }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmPromptMessage {
    Confirm,
    Deny,
}

impl From<ConfirmPromptMessage> for Message {
    fn from(value: ConfirmPromptMessage) -> Self {
        Self::ConfirmationPrompt(value)
    }
}

impl HandleMessage<ConfirmPromptMessage> for App {
    fn update(&mut self, msg: ConfirmPromptMessage) -> anyhow::Result<iced::Task<Message>> {
        let Some(Scene::Confirm(confirm)) = self.scenes.last() else {
            return Message::done();
        };

        match msg {
            ConfirmPromptMessage::Confirm => {
                let msg = confirm.on_confirm.clone();
                self.scenes.pop();
                self.update(msg)
            }
            ConfirmPromptMessage::Deny => {
                self.scenes.pop();
                Message::done()
            }
        }
    }
}

impl View<ConfirmPrompt> for App {
    fn view<'a>(&'a self, scene: &'a ConfirmPrompt) -> iced::Element<'a, Message> {
        todo!()
    }
}

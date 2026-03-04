use crate::{logic::Message, traits::HandleMessage};

pub struct ConfirmationPrompt {
    text: String,
    on_confirm: Message,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationPromptMessage {
    Confirm,
    Deny,
}

impl From<ConfirmationPromptMessage> for Message {
    fn from(value: ConfirmationPromptMessage) -> Self {
        
    }
}

impl HandleMessage<ConfirmationPromptMessage> for Message {
    fn update(&mut self, msg: ConfirmationPromptMessage) -> anyhow::Result<iced::Task<Message>> {
        
    }
}

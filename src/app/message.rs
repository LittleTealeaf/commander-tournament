use super::App;
use super::traits::HandleMessage;
use iced::Task;
use opener::open_browser;

#[derive(Clone)]
pub enum Message {
    ClearError,
    OpenLink(String),
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<Option<Task<Message>>> {
        match msg {
            Message::ClearError => {
                self.error = None;
                Ok(None)
            }
            Message::OpenLink(link) => {
                open_browser(link)?;
                Ok(None)
            }
        }
    }
}

use iced::{Element, Task};

use crate::app::{App, Message};

pub trait HandleMessage<T> {
    fn update(&mut self, msg: T) -> anyhow::Result<Option<Task<Message>>>;
}

pub trait View {
    fn view(app: &App) -> Element<'_, Message>;
}

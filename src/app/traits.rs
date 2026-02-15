use iced::{Element, Task};

use crate::app::{App, Message};

pub trait HandleMessage<T> {
    fn done() -> anyhow::Result<Task<Message>> {
        Ok(Task::none())
    }

    fn update(&mut self, msg: T) -> anyhow::Result<Task<Message>>;
}

pub trait View {
    fn view(app: &App) -> Element<'_, Message>;
}

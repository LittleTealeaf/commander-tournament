use iced::Element;

use crate::app::{App, Message};

pub trait HandleMessage<T> {
    fn update(&mut self, msg: T) -> anyhow::Result<()>;
}


pub trait View {
    fn view(app: &App) -> Element<'_, Message>;
}

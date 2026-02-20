use iced::{Element, Task};

use crate::App;
use crate::logic::Message;

pub trait HandleMessage<T> {
    fn update(&mut self, msg: T) -> anyhow::Result<Task<Message>>;

    #[cfg(test)]
    fn test_update(&mut self, msg: T) {
        let _ = self.update(msg).unwrap();
    }
}

pub trait View {
    fn view(app: &App) -> Element<'_, Message>;
}

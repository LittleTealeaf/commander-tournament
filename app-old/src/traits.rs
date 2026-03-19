use iced::{Element, Task};

use crate::logic::Message;

pub trait HandleMessage<T> {
    fn update(&mut self, msg: T) -> anyhow::Result<Task<Message>>;
}

pub trait View<S> {
    fn view<'a>(&'a self, scene: &'a S) -> Element<'a, Message>;
}

pub trait PushMaybe<'a, M, T, R> {
    #[must_use]
    fn push_maybe(self, item: Option<impl Into<Element<'a, M, T, R>>>) -> Self;
}

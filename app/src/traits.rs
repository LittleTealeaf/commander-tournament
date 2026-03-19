use iced::{Element, Task};

#[derive(Debug)]
pub struct Effect<M, O> {
    pub task: Task<M>,
    pub out: Option<O>,
}

impl<M, O> Effect<M, O> {
    pub fn ok() -> anyhow::Result<Self> {
        Ok(Self {
            task: Task::none(),
            out: None,
        })
    }

    pub fn out(out_message: O) -> anyhow::Result<Self> {
        Ok(Self {
            task: Task::none(),
            out: Some(out_message),
        })
    }

    pub const fn task(task: Task<M>) -> anyhow::Result<Self> {
        Ok(Self { task, out: None })
    }

    pub const fn both(out_message: O, task: Task<M>) -> anyhow::Result<Self> {
        Ok(Self {
            task,
            out: Some(out_message),
        })
    }
}

pub trait Component {
    type OutMessage;
    type Message: Clone + Send + 'static;

    type Context<'a>;

    fn update(
        &mut self,
        message: Self::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;

    fn view(&self, context: Self::Context<'_>) -> Element<'_, Self::Message>;
}

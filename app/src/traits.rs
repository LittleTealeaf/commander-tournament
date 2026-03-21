use iced::{Element, Task};
use iced_futures::MaybeSend;

#[derive(Debug)]
pub struct Effect<M, O> {
    pub task: Task<M>,
    pub out: Vec<O>,
}

impl<M, O> Effect<M, O>
where
    M: 'static,
{
    pub fn ok() -> anyhow::Result<Self> {
        Ok(Self {
            task: Task::none(),
            out: Vec::new(),
        })
    }

    pub fn out(out_message: O) -> anyhow::Result<Self> {
        Ok(Self {
            task: Task::none(),
            out: vec![out_message],
        })
    }

    pub const fn task(task: Task<M>) -> anyhow::Result<Self> {
        Ok(Self {
            task,
            out: Vec::new(),
        })
    }

    pub fn both(out_message: O, task: Task<M>) -> anyhow::Result<Self> {
        Ok(Self {
            task,
            out: vec![out_message],
        })
    }

    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        Self {
            task: self.task.chain(other.task),
            out: self.out.into_iter().chain(other.out).collect(),
        }
    }

    pub fn map<MN, ON, F>(self, mut map_out: F) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: FnMut(O) -> anyhow::Result<Effect<MN, ON>>,
    {
        let mut task = self.task.map(Into::into);
        let mut out = Vec::new();

        for o in self.out {
            let effect = map_out(o)?;
            out.extend(effect.out);
            task = task.chain(effect.task);
        }

        Ok(Effect { task, out })
    }
}

pub trait Component {
    type OutMessage;
    type Message: Clone + Send + 'static;

    type Context<'a>;
}

pub trait ComponentView: Component {
    fn view<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message>;

    fn view_into<'a, M>(&'a self, context: Self::Context<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a,
    {
        self.view(context).map(Into::into)
    }
}

pub trait ComponentUpdate: Component {
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}

pub trait HandleMessage<M>: Component {
    fn handle_message(
        &mut self,
        message: M,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}

impl<T: Component + ComponentUpdate> HandleMessage<T::Message> for T {
    fn handle_message(
        &mut self,
        message: T::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.update(message, context)
    }
}

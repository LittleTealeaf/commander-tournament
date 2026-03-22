use iced::{Element, Task};
use iced_futures::MaybeSend;

use crate::core::message::Message;

#[derive(Debug)]
pub enum Effect<M, O> {
    Global(Message),
    Out(O),
    Task(Task<M>),
    Chain(Box<Self>, Box<Self>),
    Done,
}

impl<M, O> Effect<M, O>
where
    M: 'static,
{
    pub const fn done() -> anyhow::Result<Self> {
        Ok(Self::Done)
    }

    pub fn global<G>(message: G) -> anyhow::Result<Self>
    where
        G: Into<Message>,
    {
        Ok(Self::Global(message.into()))
    }

    pub const fn out(message: O) -> anyhow::Result<Self> {
        Ok(Self::Out(message))
    }

    pub const fn task(task: Task<M>) -> anyhow::Result<Self> {
        Ok(Self::Task(task))
    }

    fn inner_map<MN, ON, F>(self, map_out: &mut F) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: FnMut(O) -> anyhow::Result<Effect<MN, ON>>,
    {
        match self {
            Self::Done => Ok(Effect::Done),
            Self::Global(message) => Ok(Effect::Global(message)),
            Self::Out(message) => map_out(message),
            Self::Task(task) => Ok(Effect::Task(task.map(Into::into))),
            Self::Chain(a, b) => {
                let a = a.inner_map(map_out)?;
                let b = b.inner_map(map_out)?;
                Ok(Effect::Chain(a.into(), b.into()))
            }
        }
    }

    pub fn map<MN, ON, F>(self, mut map_out: F) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: FnMut(O) -> anyhow::Result<Effect<MN, ON>>,
    {
        self.inner_map(&mut map_out)
    }
}

impl<M> Effect<M, ()>
where
    M: MaybeSend + 'static,
{
    pub fn map_empty<MN, ON>(self) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: Into<MN>,
    {
        self.map(|()| Effect::done())
    }
}

pub trait Component {
    type OutMessage;
    type Message: Clone + Send + 'static;
}

pub trait ComponentView: Component {
    type ViewContext<'a>
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::Message>;

    fn view_into<'a, M>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a,
    {
        self.view(context).map(Into::into)
    }
}

pub trait ComponentUpdate: Component {
    type UpdateContext<'a>;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}

pub trait HandleMessage<M>: ComponentUpdate {
    fn handle_message(
        &mut self,
        message: M,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}

impl<T: Component + ComponentUpdate> HandleMessage<T::Message> for T {
    fn handle_message(
        &mut self,
        message: T::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.update(message, context)
    }
}

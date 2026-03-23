use core::iter::once;

use iced::{Element, Task};
use iced_futures::MaybeSend;

use crate::core::message::Message;

#[derive(Debug)]
pub enum Effect<M, O> {
    Global(Message),
    Out(O),
    Task(Task<M>),
    Batch(Vec<Self>),
    Done,
}

impl<M, O> Effect<M, O>
where
    M: 'static + MaybeSend,
{
    pub const fn ok(self) -> anyhow::Result<Self> {
        Ok(self)
    }

    pub const fn done() -> anyhow::Result<Self> {
        Self::Done.ok()
    }

    pub fn global<Msg>(message: Msg) -> Self
    where
        Msg: Into<Message>,
    {
        Self::Global(message.into())
    }

    pub fn batch<I>(effects: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (tasks, mut other_effects) = effects.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut tasks, mut effects), effect| {
                match effect {
                    Self::Done => {}
                    Self::Task(task) => tasks.push(task),
                    other => effects.push(other),
                }
                (tasks, effects)
            },
        );

        if other_effects.is_empty() {
            if tasks.is_empty() {
                Self::Done
            } else {
                Self::Task(Task::batch(tasks))
            }
        } else {
            if !tasks.is_empty() {
                let task = Task::batch(tasks);
                let len = other_effects.len();
                other_effects.push(Self::Task(task));
                other_effects.swap(0, len);
            }
            Self::Batch(other_effects)
        }
    }

    pub fn sequence<I>(effects: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (task, mut other_effects) = effects.into_iter().fold(
            (Task::none(), Vec::new()),
            |(base_task, mut effects), effect| match effect {
                Self::Done => (base_task, effects),
                Self::Task(task) => (base_task.chain(task), effects),
                other => {
                    effects.push(other);
                    (base_task, effects)
                }
            },
        );
        if other_effects.is_empty() {
            Self::Task(task)
        } else {
            other_effects.push(Self::Task(task));
            Self::Batch(other_effects)
        }
    }

    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Task(left), Self::Task(right)) => Self::Task(left.chain(right)),
            (Self::Batch(left), Self::Batch(right)) => {
                Self::sequence(left.into_iter().chain(right))
            }
            (effect, Self::Batch(effects)) => Self::sequence(once(effect).chain(effects)),
            (Self::Batch(effects), effect) => {
                Self::sequence(effects.into_iter().chain(once(effect)))
            }
            (left, right) => Self::Batch(vec![left, right]),
        }
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Task(left), Self::Task(right)) => Self::Task(Task::batch([left, right])),
            (Self::Batch(left), Self::Batch(right)) => Self::batch(left.into_iter().chain(right)),
            (effect, Self::Batch(effects)) | (Self::Batch(effects), effect) => {
                Self::batch(once(effect).chain(effects))
            }
            (left, right) => Self::Batch(vec![left, right]),
        }
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
            Self::Batch(batch) => {
                let mut effects = Vec::new();
                for effect in batch {
                    effects.push(effect.inner_map(map_out)?);
                }
                Ok(Effect::Batch(effects))
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

    pub const fn is_done(&self) -> bool {
        matches!(self, Self::Done)
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

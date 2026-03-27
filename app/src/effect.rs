use core::iter::once;

use iced::Task;
use iced_futures::MaybeSend;

use crate::core::message::Message;

#[derive(Debug, Default)]
pub enum Effect<M, O> {
    Global(Message),
    Out(O),
    Task(Task<M>),
    Batch(Vec<Self>),
    Sequence(Vec<Self>),
    #[default]
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

    pub fn future<F>(future: F) -> Self
    where
        F: core::future::Future<Output = M> + Send + 'static,
    {
        Self::Task(Task::future(future))
    }

    pub fn batch<I>(effects: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        effects.into_iter().fold(Self::Done, Self::merge)
    }

    /// Notes: tasks are not spawned if any message fails to complete
    pub fn sequence<I>(effects: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        effects.into_iter().fold(Self::Done, Self::chain)
    }

    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Sequence(left), Self::Sequence(right)) => {
                Self::Sequence(left.into_iter().chain(right).collect())
            }
            (effect, Self::Sequence(effects)) => {
                Self::Sequence(once(effect).chain(effects).collect())
            }
            (Self::Sequence(effects), effect) => {
                Self::Sequence(effects.into_iter().chain(once(effect)).collect())
            }
            (left, right) => Self::Sequence(vec![left, right]),
        }
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Batch(left), Self::Batch(right)) => {
                Self::Batch(left.into_iter().chain(right).collect())
            }
            (effect, Self::Batch(mut effects)) | (Self::Batch(mut effects), effect) => {
                effects.push(effect);
                Self::Batch(effects)
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
            Self::Sequence(sequence) => {
                let mut effects = Vec::new();
                for effect in sequence {
                    effects.push(effect.inner_map(map_out)?);
                }
                Ok(Effect::Sequence(effects))
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

#[cfg(test)]
mod tests {

    use super::*;
    #[derive(Debug)]
    struct Message(usize);

    #[derive(Debug, PartialEq, Eq)]
    struct Out(usize);

    type Eff = Effect<Message, Out>;

    #[test]
    fn sequence_dones_returns_done() {
        let effect: Eff = Effect::Done.chain(Effect::Done);
        assert!(matches!(effect, Effect::Done));

        let sequence = [Effect::Done, Effect::Done, Effect::Done];
        let effect: Eff = Effect::sequence(sequence);
        assert!(matches!(effect, Effect::Done));
    }

    #[test]
    fn sequence_chain_sequence_merges() {
        let sequence_a = Effect::sequence([Out(0), Out(1)].map(Effect::Out));
        let sequence_b = Effect::sequence([Out(2), Out(3)].map(Effect::Out));
        let effect: Eff = sequence_a.chain(sequence_b);
        let Effect::Sequence(sequence) = effect else {
            panic!("Expected output to be a sequence");
        };
        let mut iter = sequence.iter();
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(0))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(1))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(2))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(3))));
        assert!(iter.next().is_none());
    }

    #[test]
    fn batch_dones_returns_done() {
        let effect: Eff = Effect::Done.merge(Effect::Done);
        assert!(matches!(effect, Effect::Done));

        let batch = [Effect::Done, Effect::Done, Effect::Done, Effect::Done];
        let effect: Eff = Effect::batch(batch);
        assert!(matches!(effect, Effect::Done));
    }
}

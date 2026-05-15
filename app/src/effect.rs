use core::iter::once;

use iced::Task;
use iced_futures::MaybeSend;

use crate::{
    modals::{Modal, confirm::ConfirmModal},
    traits::Mapped,
};

#[derive(Debug, Default)]
pub enum Effect<M, O> {
    Msg(M),
    Out(O),
    Task(Task<M>),
    Batch(Vec<Self>),
    Sequence(Vec<Self>),
    Modal(Modal<M>),
    OnError(Box<Self>, Option<M>),
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

    pub fn out<Out>(message: Out) -> Self
    where
        Out: Into<O>,
    {
        Self::Out(message.into())
    }

    pub fn msg<Msg>(message: Msg) -> Self
    where
        Msg: Into<M>,
    {
        Self::Msg(message.into())
    }

    pub fn delayed<Msg>(message: Msg) -> Self
    where
        Msg: Into<M>,
    {
        Self::Task(Task::done(message.into()))
    }

    #[must_use]
    pub fn on_error<Msg>(self, effect: Msg) -> Self
    where
        Msg: Into<M>,
    {
        Self::OnError(Box::new(self), Some(effect.into()))
    }

    #[must_use]
    pub fn ignore_error(self) -> Self {
        Self::OnError(Box::new(self), None)
    }

    #[must_use]
    pub fn maybe_on_error<Msg>(self, effect: Option<Msg>) -> Self
    where
        Msg: Into<M>,
    {
        match effect {
            Some(msg) => self.on_error(msg),
            None => self.ignore_error(),
        }
    }

    pub fn modal<A>(modal: A) -> Self
    where
        A: Into<Modal<M>>,
    {
        Self::Modal(modal.into())
    }

    pub fn confirm(title: String, details: String, on_confirm: M, on_cancel: Option<M>) -> Self {
        Self::modal(ConfirmModal::new(title, details, on_confirm, on_cancel))
    }

    pub fn perform<Fun, Out, H>(future: Fun, handler: H) -> Self
    where
        Fun: core::future::Future<Output = Out> + Send + 'static,
        H: Fn(Out) -> M + iced_futures::MaybeSend + 'static,
        Out: iced_futures::MaybeSend + 'static,
    {
        Self::Task(Task::perform(future, handler))
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
        let iter = effects.into_iter();
        // Pre-allocate memory based on the iterator's size hint
        let mut flat = Vec::with_capacity(iter.size_hint().0);

        for effect in iter {
            match effect {
                Self::Done => {}
                Self::Batch(mut items) => flat.append(&mut items),
                other => flat.push(other),
            }
        }

        match flat.len() {
            0 => Self::Done,
            1 => flat.pop().unwrap(), // Avoid wrapping a single item
            _ => Self::Batch(flat),
        }
    }

    pub fn sequence<I>(effects: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let iter = effects.into_iter();
        // Pre-allocate memory based on the iterator's size hint
        let mut flat = Vec::with_capacity(iter.size_hint().0);

        for effect in iter {
            match effect {
                Self::Done => {}
                Self::Sequence(mut items) => flat.append(&mut items),
                other => flat.push(other),
            }
        }

        match flat.len() {
            0 => Self::Done,
            1 => flat.pop().unwrap(), // Avoid wrapping a single item
            _ => Self::Sequence(flat),
        }
    }

    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Sequence(left), Self::Sequence(right)) => {
                Self::Sequence(left.into_iter().chain(right).collect())
            }
            (effect, Self::Sequence(effects)) => Self::Sequence(once(effect).chain(effects).collect()),
            (Self::Sequence(mut effects), effect) => {
                effects.push(effect);
                Self::Sequence(effects)
            }
            (left, right) => Self::Sequence(vec![left, right]),
        }
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Batch(left), Self::Batch(right)) => Self::Batch(left.into_iter().chain(right).collect()),
            (effect, Self::Batch(mut effects)) | (Self::Batch(mut effects), effect) => {
                effects.push(effect);
                Self::Batch(effects)
            }
            (left, right) => Self::Batch(vec![left, right]),
        }
    }

    fn inner_map<MN, ON, F>(self, map_out: &F) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: Fn(O) -> anyhow::Result<Effect<MN, ON>>,
    {
        match self {
            Self::OnError(effect, on_error) => {
                Effect::OnError(Box::new(effect.inner_map(map_out)?), on_error.map(Into::into)).ok()
            }
            Self::Done => Ok(Effect::Done),
            Self::Out(message) => map_out(message),
            Self::Modal(modal) => Ok(Effect::Modal(modal.map())),
            Self::Task(task) => Ok(Effect::Task(task.map(Into::into))),
            Self::Msg(message) => Effect::Msg(message.into()).ok(),
            Self::Batch(batch) => Ok(Effect::Batch(
                batch
                    .into_iter()
                    .map(|effect| effect.inner_map(map_out))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )),
            Self::Sequence(sequence) => Ok(Effect::Sequence(
                sequence
                    .into_iter()
                    .map(|effect| effect.inner_map(map_out))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )),
        }
    }

    pub fn map<MN, ON, F>(self, map_out: F) -> anyhow::Result<Effect<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: Fn(O) -> anyhow::Result<Effect<MN, ON>>,
    {
        self.inner_map(&map_out)
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

    use itertools::Itertools;

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
    fn chain_appends_at_beginning() {
        let item = Effect::Out(Out(0));
        let sequence = Effect::sequence([Out(1), Out(2)].map(Effect::Out));
        let effect: Eff = item.chain(sequence);
        let Effect::Sequence(sequence) = effect else {
            panic!("Expected output to be a sequence");
        };

        let mut iter = sequence.iter();
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(0))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(1))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(2))));
        assert!(iter.next().is_none());
    }

    #[test]
    fn chain_appends_at_end() {
        let sequence = Effect::sequence([Out(1), Out(2)].map(Effect::Out));
        let item = Effect::Out(Out(0));
        let effect: Eff = sequence.chain(item);
        let Effect::Sequence(sequence) = effect else {
            panic!("Expected output to be a sequence");
        };

        let mut iter = sequence.iter();
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(1))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(2))));
        assert!(matches!(iter.next().unwrap(), Effect::Out(Out(0))));
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

    #[test]
    fn batch_merges_into_batch() {
        let batch_a = Effect::batch([Out(0), Out(1)].map(Effect::Out));
        let batch_b = Effect::batch([Out(2), Out(3)].map(Effect::Out));
        let effect: Eff = batch_a.merge(batch_b);

        let Effect::Batch(batch) = effect else {
            panic!("Expected output to be a batch");
        };

        let outs = batch
            .into_iter()
            .filter_map(|item| match item {
                Effect::Out(out) => Some(out),
                _ => None,
            })
            .collect_vec();

        assert!(outs.contains(&Out(0)));
        assert!(outs.contains(&Out(1)));
        assert!(outs.contains(&Out(2)));
        assert!(outs.contains(&Out(3)));
    }
}

use crate::tournament::Tournament;

mod aggregates;
mod winloss;

#[derive(Debug, Copy, Clone)]
pub struct Analytics<'a> {
    tourn: &'a Tournament,
}

impl Tournament {
    #[must_use]
    pub const fn analytics(&self) -> Analytics<'_> {
        Analytics { tourn: self }
    }
}

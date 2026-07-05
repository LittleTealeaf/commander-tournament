use crate::tournament::Tournament;

mod aggregates;
mod winloss;

#[derive(Debug, Copy, Clone)]
pub struct Analytics<'a> {
    tourn: &'a Tournament,
    include_precons: bool,
}

impl Tournament {
    #[must_use]
    pub const fn analytics(&self) -> Analytics<'_> {
        Analytics {
            tourn: self,
            include_precons: true,
        }
    }
}

impl Analytics<'_> {
    pub const fn with_precons(self, include_precons: bool) -> Self {
        Self {
            include_precons,
            ..self
        }
    }
}

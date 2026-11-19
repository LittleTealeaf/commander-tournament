use crate::{player::PlayerId, tournament::Tournament};

mod aggregates;
mod winloss;

#[derive(Debug, Clone)]
pub struct Analytics<'a, F>
where
    F: Fn(PlayerId) -> bool,
{
    tourn: &'a Tournament,
    pool: F,
}

impl Tournament {
    #[must_use]
    pub const fn analytics(&self) -> Analytics<'_, impl Fn(PlayerId) -> bool> {
        Analytics {
            tourn: self,
            pool: |_| true,
        }
    }
}

impl<'a, F> Analytics<'a, F>
where
    F: Fn(PlayerId) -> bool,
{
    pub fn with_pool<F2>(self, pool: F2) -> Analytics<'a, F2>
    where
        F2: Fn(PlayerId) -> bool,
    {
        Analytics {
            tourn: self.tourn,
            pool,
        }
    }

    pub fn filter_pool<F2>(self, filter: F2) -> Analytics<'a, impl Fn(PlayerId) -> bool>
    where
        F2: Fn(PlayerId) -> bool,
    {
        Analytics {
            tourn: self.tourn,
            pool: move |id| (self.pool)(id) && filter(id),
        }
    }

    pub fn filter_precons(self, include: bool) -> Analytics<'a, impl Fn(PlayerId) -> bool> {
        let precons = {
            if include {
                im::HashSet::new()
            } else {
                self.tourn
                    .players()
                    .iter()
                    .filter_map(|(id, info)| info.is_precon().then_some(*id))
                    .collect::<im::HashSet<PlayerId>>()
            }
        };

        self.filter_pool(move |id| !precons.contains(&id))
    }

    pub fn validated_pool(&self) -> impl Iterator<Item = PlayerId> {
        self.tourn.players().keys().copied().filter(|i| (self.pool)(*i))
    }
}

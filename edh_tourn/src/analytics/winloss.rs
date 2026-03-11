pub mod color;
pub mod player;

use crate::{error::TournamentError};

#[derive(Debug)]
pub struct AggregatePerformance<T> {
    value: T,
    games_played: usize,
    games_won: usize,
    games_lost: usize,
}

impl<T> AggregatePerformance<T> {
    const fn new(value: T) -> Self {
        Self {
            value,
            games_lost: 0,
            games_won: 0,
            games_played: 0,
        }
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub const fn games_played(&self) -> usize {
        self.games_played
    }

    #[must_use]
    pub const fn games_won(&self) -> usize {
        self.games_won
    }

    #[must_use]
    pub const fn games_lost(&self) -> usize {
        self.games_lost
    }
}

pub trait GetAggregatePerformance<T> {
    fn get_player_aggregate_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = AggregatePerformance<T>>, TournamentError>;
}

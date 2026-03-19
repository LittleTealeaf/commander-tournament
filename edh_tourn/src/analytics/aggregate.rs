use core::ops::{Add, AddAssign};

use crate::player::stats::PlayerStats;

#[derive(Debug, Clone, Default)]
pub struct AggregateStats {
    sum_elos: f64,
    count: u32,
    games: u32,
    wins: u32,
}

impl AggregateStats {
    #[must_use]
    pub const fn games(&self) -> u32 {
        self.games
    }

    #[must_use]
    pub const fn wins(&self) -> u32 {
        self.wins
    }

    #[must_use]
    pub const fn losses(&self) -> u32 {
        self.games - self.wins
    }

    #[must_use]
    pub fn avg_elo(&self) -> Option<f64> {
        (self.count > 0).then(|| self.sum_elos / f64::from(self.count))
    }

    #[must_use]
    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| f64::from(self.wins) / f64::from(self.games))
    }
}

impl From<PlayerStats> for AggregateStats {
    fn from(value: PlayerStats) -> Self {
        (&value).into()
    }
}

impl From<&PlayerStats> for AggregateStats {
    fn from(value: &PlayerStats) -> Self {
        Self {
            sum_elos: value.elo(),
            count: 1,
            games: value.games(),
            wins: value.wins(),
        }
    }
}

impl AddAssign for AggregateStats {
    fn add_assign(&mut self, rhs: Self) {
        self.sum_elos += rhs.sum_elos;
        self.count += rhs.count;
        self.wins += rhs.wins;
        self.games += rhs.games;
    }
}

impl Add for AggregateStats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sum_elos: self.sum_elos + rhs.sum_elos,
            count: self.count + rhs.count,
            games: self.games + rhs.games,
            wins: self.wins + rhs.wins,
        }
    }
}

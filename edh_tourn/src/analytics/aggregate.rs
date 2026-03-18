use core::ops::{Add, AddAssign};

use crate::player::stats::PlayerStats;

#[derive(Debug, Clone)]
pub struct AggregateStats {
    sum_elos: f64,
    record_count_f64: f64,
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
    pub fn avg_elo(&self) -> f64 {
        self.sum_elos / self.record_count_f64
    }

    #[must_use]
    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| f64::from(self.wins) / f64::from(self.games))
    }
}

impl From<&PlayerStats> for AggregateStats {
    fn from(value: &PlayerStats) -> Self {
        Self {
            sum_elos: value.elo(),
            record_count_f64: 1f64,
            games: value.games(),
            wins: value.wins(),
        }
    }
}

impl AddAssign for AggregateStats {
    fn add_assign(&mut self, rhs: Self) {
        self.sum_elos += rhs.sum_elos;
        self.record_count_f64 += rhs.record_count_f64;
        self.wins += rhs.wins;
        self.games += rhs.games;
    }
}

impl Add for AggregateStats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sum_elos: self.sum_elos + rhs.sum_elos,
            record_count_f64: self.record_count_f64 + rhs.record_count_f64,
            games: self.games + rhs.games,
            wins: self.wins + rhs.wins,
        }
    }
}

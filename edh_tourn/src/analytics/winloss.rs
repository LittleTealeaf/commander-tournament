use core::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign},
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
)]
pub struct MatchPerformance {
    played: usize,
    won: usize,
    lost: usize,
}

impl MatchPerformance {
    #[must_use]
    pub(crate) const fn new(played: usize, won: usize, lost: usize) -> Self {
        Self { played, won, lost }
    }

    pub const WIN: Self = Self {
        played: 1,
        won: 1,
        lost: 0,
    };

    pub const LOSS: Self = Self {
        played: 1,
        lost: 1,
        won: 0,
    };

    pub const DRAW: Self = Self {
        played: 1,
        won: 0,
        lost: 0,
    };
}

impl PartialOrd for MatchPerformance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MatchPerformance {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, person with the highest win to loss difference
        let baseline = self.played().max(other.played());
        let left = (baseline + self.wins()) - self.losses();
        let right = (baseline + other.wins()) - other.losses();
        let cmp = left.cmp(&right);

        cmp.then_with(|| {
            // Then, the person with the lowest non-win games
            let left = self.played() - self.wins();
            let right = other.played() - other.wins();

            let cmp = left.cmp(&right).reverse();

            cmp.then_with(|| self.wins().cmp(&other.wins()))
        })
    }
}

impl Add<Self> for MatchPerformance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            played: self.played + rhs.played,
            won: self.won + rhs.won,
            lost: self.lost + rhs.lost,
        }
    }
}

impl AddAssign<Self> for MatchPerformance {
    fn add_assign(&mut self, rhs: Self) {
        self.played += rhs.played;
        self.won += rhs.won;
        self.lost += rhs.lost;
    }
}

impl Sum for MatchPerformance {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl MatchPerformance {
    #[must_use]
    pub const fn played(&self) -> usize {
        self.played
    }

    #[must_use]
    pub const fn wins(&self) -> usize {
        self.won
    }

    #[must_use]
    pub const fn losses(&self) -> usize {
        self.lost
    }

    #[must_use]
    pub const fn draws(&self) -> usize {
        self.played - (self.won + self.lost)
    }

    pub(crate) const fn add_draw(&mut self) {
        self.played += 1;
    }

    pub(crate) const fn add_win(&mut self) {
        self.played += 1;
        self.won += 1;
    }

    pub(crate) const fn add_loss(&mut self) {
        self.played += 1;
        self.lost += 1;
    }
}

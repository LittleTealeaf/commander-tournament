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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_and_accessors() {
        let win = MatchPerformance::WIN;
        assert_eq!(win.played(), 1);
        assert_eq!(win.wins(), 1);
        assert_eq!(win.losses(), 0);
        assert_eq!(win.draws(), 0);

        let loss = MatchPerformance::LOSS;
        assert_eq!(loss.played(), 1);
        assert_eq!(loss.wins(), 0);
        assert_eq!(loss.losses(), 1);
        assert_eq!(loss.draws(), 0);

        let draw = MatchPerformance::DRAW;
        assert_eq!(draw.played(), 1);
        assert_eq!(draw.wins(), 0);
        assert_eq!(draw.losses(), 0);
        assert_eq!(draw.draws(), 1);
    }

    #[test]
    fn test_mutators() {
        let mut perf = MatchPerformance::default();
        perf.add_win();
        perf.add_loss();
        perf.add_draw();

        assert_eq!(perf.played(), 3);
        assert_eq!(perf.wins(), 1);
        assert_eq!(perf.losses(), 1);
        assert_eq!(perf.draws(), 1);
    }

    #[test]
    fn test_math_operations() {
        let p1 = MatchPerformance::new(3, 2, 1);
        let p2 = MatchPerformance::new(2, 0, 1);

        // Test Add
        let p3 = p1 + p2;
        assert_eq!(p3.played(), 5);
        assert_eq!(p3.wins(), 2);
        assert_eq!(p3.losses(), 2);

        // Test AddAssign
        let mut p4 = p1;
        p4 += p2;
        assert_eq!(p4, p3);

        // Test Sum
        let performances = vec![
            MatchPerformance::WIN,
            MatchPerformance::LOSS,
            MatchPerformance::DRAW,
        ];
        let sum: MatchPerformance = performances.into_iter().sum();
        assert_eq!(sum.played(), 3);
        assert_eq!(sum.wins(), 1);
        assert_eq!(sum.losses(), 1);
        assert_eq!(sum.draws(), 1);
    }

    #[test]
    fn test_ordering_primary() {
        // Highest win to loss difference wins
        let p_good = MatchPerformance::new(5, 4, 1); // diff baseline is heavily positive
        let p_bad = MatchPerformance::new(5, 1, 4); // diff baseline is negative
        assert!(p_good > p_bad);
        assert_eq!(p_good.cmp(&p_bad), Ordering::Greater);
    }

    #[test]
    fn test_ordering_secondary() {
        // Tie on primary, lowest non-win games wins
        // Both diffs will be the same when calculated with baseline 5:
        // p1: wins 2, losses 1. diff = (5 + 2) - 1 = 6. non-wins = 3 - 2 = 1.
        // p2: wins 3, losses 2. diff = (5 + 3) - 2 = 6. non-wins = 5 - 3 = 2.
        let p_few_non_wins = MatchPerformance::new(3, 2, 1);
        let p_many_non_wins = MatchPerformance::new(5, 3, 2);

        assert!(p_few_non_wins > p_many_non_wins);
    }

    #[test]
    fn test_ordering_tertiary() {
        // Tie on primary and secondary, highest total wins wins
        // Baseline 5.
        // p1: 5 played, 3 wins, 1 loss (1 draw). diff = (5 + 3) - 1 = 7. non-wins = 5 - 3 = 2.
        // p2: 4 played, 2 wins, 0 losses (2 draws). diff = (5 + 2) - 0 = 7. non-wins = 4 - 2 = 2.
        let p_more_wins = MatchPerformance::new(5, 3, 1);
        let p_fewer_wins = MatchPerformance::new(4, 2, 0);

        assert!(p_more_wins > p_fewer_wins);
    }

    #[test]
    fn test_ordering_equality() {
        let p_eq1 = MatchPerformance::new(4, 2, 1);
        let p_eq2 = MatchPerformance::new(4, 2, 1);
        assert_eq!(p_eq1, p_eq2);
        assert_eq!(p_eq1.cmp(&p_eq2), Ordering::Equal);
    }
}

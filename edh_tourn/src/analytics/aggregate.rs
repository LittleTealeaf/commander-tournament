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

impl<I> AddAssign<I> for AggregateStats
where
    I: Into<Self>,
{
    fn add_assign(&mut self, rhs: I) {
        let agg = rhs.into();
        self.sum_elos += agg.sum_elos;
        self.count += agg.count;
        self.wins += agg.wins;
        self.games += agg.games;
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    // Adjust this import/instantiation based on how `PlayerStats` is constructed in your codebase.
    use crate::player::stats::PlayerStats;

    #[test]
    fn test_games() {
        let stats = AggregateStats {
            games: 15,
            ..Default::default()
        };
        assert_eq!(stats.games(), 15);
    }

    #[test]
    fn test_wins() {
        let stats = AggregateStats {
            wins: 7,
            ..Default::default()
        };
        assert_eq!(stats.wins(), 7);
    }

    #[test]
    fn test_losses() {
        let stats = AggregateStats {
            games: 15,
            wins: 7,
            ..Default::default()
        };
        assert_eq!(stats.losses(), 8);
    }

    #[test]
    fn avg_elo_returns_none_when_count_is_zero() {
        let stats = AggregateStats::default();
        assert!(stats.avg_elo().is_none());
    }

    #[test]
    fn avg_elo_calculates_correctly() {
        let stats = AggregateStats {
            sum_elos: 3000.0,
            count: 2,
            ..Default::default()
        };
        assert_eq!(stats.avg_elo(), Some(1500.0));
    }

    #[test]
    fn wr_returns_none_when_games_is_zero() {
        let stats = AggregateStats::default();
        assert!(stats.wr().is_none());
    }

    #[test]
    fn wr_calculates_correctly() {
        let stats = AggregateStats {
            games: 10,
            wins: 4,
            ..Default::default()
        };
        assert_eq!(stats.wr(), Some(0.4));
    }

    #[test]
    fn from_player_stats_reference() {
        let ps = PlayerStats::new(100.0);

        let agg = AggregateStats::from(&ps);

        assert_relative_eq!(agg.sum_elos, ps.elo());
        assert_eq!(agg.count, 1);
        assert_eq!(agg.games, ps.games());
        assert_eq!(agg.wins, ps.wins());
    }

    #[test]
    fn from_player_stats_owned() {
        let ps = PlayerStats::new(100.0);
        let elo = ps.elo();
        let games = ps.games();
        let wins = ps.wins();

        let agg = AggregateStats::from(ps);

        assert_relative_eq!(agg.sum_elos, elo);
        assert_eq!(agg.count, 1);
        assert_eq!(agg.games, games);
        assert_eq!(agg.wins, wins);
    }

    #[test]
    fn test_add() {
        let a = AggregateStats {
            sum_elos: 1200.0,
            count: 1,
            games: 5,
            wins: 3,
        };
        let b = AggregateStats {
            sum_elos: 1400.0,
            count: 2,
            games: 10,
            wins: 4,
        };

        let c = a + b;

        assert_relative_eq!(c.sum_elos, 2600.0);
        assert_eq!(c.count, 3);
        assert_eq!(c.games, 15);
        assert_eq!(c.wins, 7);
    }

    #[test]
    fn test_add_assign() {
        let mut a = AggregateStats {
            sum_elos: 1200.0,
            count: 1,
            games: 5,
            wins: 3,
        };
        let b = AggregateStats {
            sum_elos: 1400.0,
            count: 2,
            games: 10,
            wins: 4,
        };

        a += b;

        assert_relative_eq!(a.sum_elos, 2600.0);
        assert_eq!(a.count, 3);
        assert_eq!(a.games, 15);
        assert_eq!(a.wins, 7);
    }

    #[test]
    fn test_default() {
        let stats = AggregateStats::default();
        assert_relative_eq!(stats.sum_elos, 0.0);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.games, 0);
        assert_eq!(stats.wins, 0);
    }

    #[test]
    fn test_clone() {
        let stats = AggregateStats {
            sum_elos: 1500.0,
            count: 1,
            games: 10,
            wins: 5,
        };
        let cloned = stats.clone();

        assert_relative_eq!(stats.sum_elos, cloned.sum_elos);
        assert_eq!(stats.count, cloned.count);
        assert_eq!(stats.games, cloned.games);
        assert_eq!(stats.wins, cloned.wins);
    }
}

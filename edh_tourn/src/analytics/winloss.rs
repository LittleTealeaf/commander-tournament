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
//
// #[cfg(test)]
// mod tests {
//     use crate::{game::entry::GameEntry, player::info::PlayerInfo};
//
//     use super::*;
//
//     mod match_performance {
//         use super::*;
//
//         #[test]
//         fn test_primary_comparison_win_loss_diff() {
//             // A has diff of +2, B has diff of 0
//             let player_a = MatchPerformance::new(2, 2, 0);
//             let player_b = MatchPerformance::new(2, 1, 1);
//
//             assert_eq!(player_a.cmp(&player_b), Ordering::Greater);
//             assert_eq!(player_b.cmp(&player_a), Ordering::Less);
//
//             // Negative differentials: A has -2, B has -1
//             let player_c = MatchPerformance::new(2, 0, 2);
//             let player_d = MatchPerformance::new(2, 0, 1); // e.g., 1 draw
//
//             assert_eq!(player_c.cmp(&player_d), Ordering::Less);
//         }
//
//         #[test]
//         fn test_secondary_comparison_lowest_non_wins() {
//             // Both have a win/loss diff of 0
//             // A non-wins: 2 played - 1 win = 1
//             // B non-wins: 4 played - 2 wins = 2
//             // A should win because they have fewer non-win games
//             let player_a = MatchPerformance::new(2, 1, 1);
//             let player_b = MatchPerformance::new(4, 2, 2);
//
//             assert_eq!(player_a.cmp(&player_b), Ordering::Greater);
//             assert_eq!(player_b.cmp(&player_a), Ordering::Less);
//         }
//
//         #[test]
//         fn test_tertiary_comparison_most_total_wins() {
//             // Win/loss diff is the same: A (+2), B (+2)
//             // Non-win games is the same: A (5-3=2), B (4-2=2)
//             // A has more total wins (3 > 2), so A should be greater
//             let player_a = MatchPerformance::new(5, 3, 1); // 3 wins, 1 loss, 1 draw
//             let player_b = MatchPerformance::new(4, 2, 0); // 2 wins, 0 losses, 2 draws
//
//             assert_eq!(player_a.cmp(&player_b), Ordering::Greater);
//             assert_eq!(player_b.cmp(&player_a), Ordering::Less);
//         }
//
//         #[test]
//         fn test_total_equality() {
//             // Exactly the same stats
//             let player_a = MatchPerformance::new(3, 1, 1);
//             let player_b = MatchPerformance::new(3, 1, 1);
//
//             assert_eq!(player_a.cmp(&player_b), Ordering::Equal);
//         }
//     }
//
//     #[test]
//     fn player_player_does_not_return_self() {
//         for tourn in Tournament::test_tournaments() {
//             for id in tourn.players().keys() {
//                 let values = tourn.get_player_player_match_performance(*id).unwrap();
//                 for (player, _) in values {
//                     assert_ne!(*id, player.id());
//                 }
//             }
//         }
//     }
//
//     #[test]
//     fn player_identities_includes_self() {
//         let mut tourn = Tournament::new();
//         let id = tourn.register_debug_player().unwrap();
//         tourn
//             .register_entry(GameEntry::new([id; 4], id).unwrap())
//             .unwrap();
//         assert!(
//             !tourn
//                 .get_player_identity_match_performance(id)
//                 .unwrap()
//                 .is_empty()
//         );
//     }
//
//     #[test]
//     fn player_colors_include_self() {
//         let mut tourn = Tournament::new();
//         let id = tourn
//             .register_player_with_info(
//                 PlayerInfo::new("debug".to_owned()).with_color_identity(ColorIdentity::AZORIUS),
//             )
//             .unwrap();
//         tourn
//             .register_entry(GameEntry::new([id; 4], id).unwrap())
//             .unwrap();
//         let ranking = tourn.get_player_color_match_performance(id).unwrap();
//         assert!(!ranking.is_empty());
//     }
// }

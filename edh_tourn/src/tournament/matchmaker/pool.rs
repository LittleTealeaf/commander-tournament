use core::cmp::Ordering;
use std::collections::BinaryHeap;

use ordered_float::NotNan;

use crate::{player::PlayerId, tournament::matchmaker::Matchmaker};

struct HeapPlayer {
    id: PlayerId,
    diff: NotNan<f64>,
}

impl PartialEq for HeapPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.diff == other.diff && self.id == other.id
    }
}

impl Eq for HeapPlayer {}

impl PartialOrd for HeapPlayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapPlayer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.diff.cmp(&other.diff).then_with(|| self.id.cmp(&other.id))
    }
}

impl Matchmaker<'_> {
    pub(super) fn create_player_pool(&self, player: PlayerId) -> im::HashSet<PlayerId> {
        let target_elo = self.tourn.get_player_or_default_stats(player).elo();
        let elo_range = self.config.elo_range();

        let (mut pool, outliers): (Vec<_>, _) = self.players.iter().copied().partition(|id| {
            (self.tourn.get_player_or_default_stats(*id).elo() - target_elo).abs() < elo_range
        });

        pool.retain(|id| *id != player);

        if pool.len() < self.config.min_pool_size() {
            pool.reserve(self.config.min_pool_size() - pool.len());

            let mut heap = outliers
                .into_iter()
                .filter_map(|id| {
                    Some(HeapPlayer {
                        id,
                        diff: NotNan::new(
                            (self.tourn.get_player_or_default_stats(id).elo() - target_elo).abs(),
                        )
                        .ok()?,
                    })
                })
                .collect::<BinaryHeap<_>>();

            while pool.len() < self.config.min_pool_size()
                && let Some(entry) = heap.pop()
            {
                pool.push(entry.id);
            }
        }

        pool.into_iter().collect()
    }
}

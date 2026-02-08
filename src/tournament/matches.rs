use std::{cmp::Ordering, collections::HashMap, iter::empty};

use itertools::Itertools;

use crate::{Tournament, error::TournamentError, stats::PlayerStats};

fn with_tie_breaker(cmp: Ordering, tie_breaker: impl Fn() -> Ordering) -> Ordering {
    match cmp {
        Ordering::Equal => tie_breaker(),
        cmp => cmp,
    }
}

impl Tournament {
    fn ensure_id_registered(&self, id: &u32) -> Result<(), TournamentError> {
        if !self.is_id_registered(id) {
            return Err(TournamentError::InvalidPlayerId(*id));
        }
        Ok(())
    }

    fn get_elo(&self, id: &u32) -> f64 {
        self.get_player_stats(id)
            .map(PlayerStats::elo)
            .unwrap_or(self.config.starting_elo)
    }

    pub fn rank_least_played(
        &self,
        id: &u32,
    ) -> Result<impl Iterator<Item = u32>, TournamentError> {
        self.ensure_id_registered(id)?;

        let mut counts = self
            .players
            .keys()
            .map(|i| (*i, 0))
            .collect::<HashMap<u32, u32>>();

        for game in &self.games {
            let players = game.players();
            if players.contains(id) {
                for player in players {
                    *counts
                        .get_mut(player)
                        .ok_or(TournamentError::InvalidPlayerId(*player))? += 1;
                }
            }
        }

        counts.remove(id);

        let cmp_elo = self.get_elo(id);

        Ok(counts
            .into_iter()
            .map(|(id, count)| (id, count, (cmp_elo - self.get_elo(&id)).abs()))
            .sorted_by(|(id1, c1, elo1), (id2, c2, elo2)| {
                with_tie_breaker(c1.cmp(c2), || {
                    with_tie_breaker(elo1.total_cmp(elo2), || id1.cmp(id2))
                })
            })
            .map(|(id, _, _)| id))
    }
}

use std::{cmp::Ordering, collections::HashMap};

use itertools::{Itertools, chain};

use crate::{Tournament, error::TournamentError, stats::PlayerStats};

fn with_tie_breaker(cmp: Ordering, tie_breaker: impl Fn() -> Ordering) -> Ordering {
    match cmp {
        Ordering::Equal => tie_breaker(),
        cmp => cmp,
    }
}

fn get_mut_from_map<'a, T>(
    id: &'a u32,
    map: &'a mut HashMap<u32, T>,
) -> Result<&'a mut T, TournamentError> {
    map.get_mut(id).ok_or(TournamentError::InvalidPlayerId(*id))
}

fn to_weight_rank(
    ranking: impl IntoIterator<Item = u32>,
    weight: f64,
) -> impl Iterator<Item = (u32, f64)> {
    ranking
        .into_iter()
        .enumerate()
        .map(move |(score, id)| (id, (score as u64) as f64 * weight))
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

    fn get_wr(&self, id: &u32) -> f64 {
        self.get_player_stats(id)
            .and_then(PlayerStats::wr)
            .unwrap_or(0.25)
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
                    *get_mut_from_map(player, &mut counts)? += 1;
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

    pub fn rank_nemesis(&self, id: &u32) -> Result<impl Iterator<Item = u32>, TournamentError> {
        self.ensure_id_registered(id)?;

        let mut counts = self
            .players
            .keys()
            .map(|i| (*i, 0))
            .collect::<HashMap<u32, i32>>();

        for game in &self.games {
            let players = game.players();
            if players.contains(id) {
                let val = if game.winner() == *id { 1 } else { -1 };
                for player in players {
                    *get_mut_from_map(player, &mut counts)? += val;
                }
            }
        }

        counts.remove(id);

        Ok(counts
            .into_iter()
            .map(|(id, score)| (id, score, self.get_elo(&id)))
            .sorted_by(|(id1, s1, e1), (id2, s2, e2)| {
                with_tie_breaker(s1.cmp(s2), || {
                    with_tie_breaker(e1.total_cmp(e2), || id1.cmp(id2))
                })
            })
            .map(|(id, _, _)| id))
    }

    pub fn rank_loss_with(&self, id: &u32) -> Result<impl Iterator<Item = u32>, TournamentError> {
        self.ensure_id_registered(id)?;

        let mut counts = self
            .players
            .keys()
            .map(|i| (*i, (0, 0)))
            .collect::<HashMap<u32, (i32, u32)>>();
        // Highest score means first pick
        // Matched scores pick highest games

        for game in &self.games {
            let players = game.players();
            if players.contains(id) {
                for pid in players {
                    let (score, games) = get_mut_from_map(pid, &mut counts)?;
                    *games += 1;
                    if game.winner() == *id || game.winner() == *pid {
                        *score -= 1
                    } else {
                        *score += 1;
                    }
                }
            }
        }

        counts.remove(id);

        Ok(counts
            .into_iter()
            .sorted_by(|(id1, (s1, c1)), (id2, (s2, c2))| {
                with_tie_breaker(s2.cmp(s1), || with_tie_breaker(c2.cmp(c1), || id1.cmp(id2)))
            })
            .map(|(id, _)| id))
    }

    pub fn rank_neighbors(&self, id: &u32) -> Result<impl Iterator<Item = u32>, TournamentError> {
        self.ensure_id_registered(id)?;
        let elo = self.get_elo(id);

        Ok(self
            .players
            .keys()
            .filter(|pid| &id != pid)
            .map(|pid| (*pid, (self.get_elo(pid) - elo).abs()))
            .sorted_by(|(i1, d1), (i2, d2)| with_tie_breaker(d1.total_cmp(d2), || i1.cmp(i2)))
            .map(|(i, _)| i))
    }

    pub fn rank_wr_neighbors(
        &self,
        id: &u32,
    ) -> Result<impl Iterator<Item = u32>, TournamentError> {
        self.ensure_id_registered(id)?;
        let wr = self.get_wr(id);

        Ok(self
            .players
            .keys()
            .filter(|pid| &id != pid)
            .map(|pid| (*pid, (self.get_wr(pid) - wr).abs()))
            .sorted_by(|(i1, d1), (i2, d2)| with_tie_breaker(d1.total_cmp(d2), || i1.cmp(i2)))
            .map(|(i, _)| i))
    }

    pub fn rank_combined(&self, id: &u32) -> Result<impl Iterator<Item = u32>, TournamentError> {
        let scores = chain!(
            to_weight_rank(
                self.rank_least_played(id)?,
                self.config.match_weight_least_played
            ),
            to_weight_rank(self.rank_nemesis(id)?, self.config.match_weight_nemesis),
            to_weight_rank(
                self.rank_wr_neighbors(id)?,
                self.config.match_weight_wr_neighbor
            ),
            to_weight_rank(self.rank_neighbors(id)?, self.config.match_weight_neighbor),
            to_weight_rank(self.rank_loss_with(id)?, self.config.match_weight_lost_with)
        );

        Ok(scores
            .into_grouping_map()
            .sum()
            .into_iter()
            .filter(|(i, _)| i != id)
            .sorted_by(|(p1, p1_s), (p2, p2_s)| {
                with_tie_breaker(p1_s.total_cmp(p2_s), || p1.cmp(p2))
            })
            .map(|(pid, _)| pid))
    }
}

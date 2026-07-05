mod games_played;
mod neighbors;
mod play_next;

use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    config::matchmaker::MatchmakerConfig,
    error::TournamentError,
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

#[derive(Debug, Clone)]
pub struct Matchmaker<'a> {
    tourn: &'a Tournament,
    config: Option<MatchmakerConfig>,
}

impl Tournament {
    #[must_use]
    pub const fn matchmaker(&self) -> Matchmaker<'_> {
        Matchmaker {
            tourn: self,
            config: None,
        }
    }
}

impl Matchmaker<'_> {
    #[must_use]
    pub const fn with_config(self, config: MatchmakerConfig) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn with_modified_config<F>(self, modifier: F) -> Self
    where
        F: Fn(&MatchmakerConfig) -> MatchmakerConfig,
    {
        Self {
            config: Some(modifier(self.config())),
            ..self
        }
    }

    const fn config(&self) -> &MatchmakerConfig {
        if let Some(config) = &self.config {
            config
        } else {
            self.tourn.config.matchmaker_config()
        }
    }

    pub fn create_match(&self, player: PlayerId) -> Result<Matchup, TournamentError> {
        self.create_match_filtered(player, |_| true)
    }

    pub fn create_match_filtered<F>(
        &self,
        player: PlayerId,
        mut allowed: F,
    ) -> Result<Matchup, TournamentError>
    where
        F: FnMut(PlayerId) -> bool,
    {
        let mut players = Vec::with_capacity(POD_SIZE);
        let mut aggregate_stats = AggregateStats::from(self.tourn.get_player_or_default_stats(player));
        players.push(player);

        let mut performances = self.tourn.analytics().player_performance_all_others(player)?;
        performances.retain(|&id, _| allowed(id));

        for _ in 1..POD_SIZE {
            let player = self
                .get_match_next_player(&aggregate_stats, &performances)
                .ok_or(TournamentError::NotEnoughPlayers)?;
            players.push(player);
            performances.remove(&player);
            for (pl, per) in self.tourn.analytics().player_performance(player)? {
                performances.entry(pl).and_modify(|entry| *entry += per);
            }
            aggregate_stats += self.tourn.get_player_or_default_stats(player);
        }

        let players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.tourn.create_match(players)
    }

    fn get_match_next_player(
        &self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> Option<PlayerId> {
        let games_played_ranked = self.ranked_games_played(agg_stats, performances);
        let neighbors_ranked = self.ranked_neighbors(agg_stats, performances);
        let combined = chain!(games_played_ranked, neighbors_ranked);
        let scores = combined.into_grouping_map().sum();
        Some(scores.into_iter().min_by_key(|&(id, n)| (n, id))?.0)
    }
}

fn to_weight_rank<I, T>(ranking: I, weight: usize) -> impl Iterator<Item = (T, usize)>
where
    I: IntoIterator<Item = T>,
{
    ranking
        .into_iter()
        .enumerate()
        .map(move |(score, val)| (val, score * weight))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn too_few_players_fails() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.matchmaker().create_match(id).unwrap_err();
    }

    #[test]
    fn first_player_always_parameter() {
        let mut t = Tournament::generate_tournament(4, 0).unwrap();
        for _ in 0..10 {
            let id = t.register_debug_player().unwrap();
            let game = t.matchmaker().create_match(id).unwrap();
            assert_eq!(id, game.players().first().unwrap().id());
        }
    }

    #[test]
    fn matchmaker_no_duplicate_players() {
        let t = Tournament::generate_tournament(10, 20).unwrap();
        let seed = *t.players().keys().next().unwrap();
        let matchup = t.matchmaker().create_match(seed).unwrap();

        let ids = matchup.ids();
        let mut unique = HashSet::new();
        for id in ids {
            assert!(unique.insert(id), "Matchup contained duplicate players!");
        }
        assert_eq!(unique.len(), POD_SIZE);
    }

    #[test]
    fn matchmaker_selects_exact_pod_size() {
        let t = Tournament::generate_tournament(10, 5).unwrap();
        let seed = *t.players().keys().next().unwrap();
        let matchup = t.matchmaker().create_match(seed).unwrap();

        assert_eq!(matchup.players().len(), POD_SIZE);
    }

    #[test]
    fn test_to_weight_rank() {
        let items = vec!["PlayerA", "PlayerB", "PlayerC"];
        let weight = 5;
        let result: Vec<_> = to_weight_rank(items, weight).collect();

        // 0 * 5 = 0, 1 * 5 = 5, 2 * 5 = 10
        assert_eq!(result, vec![("PlayerA", 0), ("PlayerB", 5), ("PlayerC", 10)]);
    }

    #[test]
    fn matchmaker_filtered_excludes_players() {
        let t = Tournament::generate_tournament(10, 5).unwrap();
        let mut keys = t.players().keys().copied().collect::<Vec<_>>();
        let seed = keys.pop().unwrap();
        let excluded_player = keys.pop().unwrap();

        // Matchmaker should create a match with seed but exclude `excluded_player`
        let matchup = t
            .matchmaker()
            .create_match_filtered(seed, |id| id != excluded_player)
            .unwrap();

        for player in matchup.players() {
            assert_ne!(player.id(), excluded_player, "Matchup contained excluded player!");
        }
        assert_eq!(matchup.players().len(), POD_SIZE);
    }
}

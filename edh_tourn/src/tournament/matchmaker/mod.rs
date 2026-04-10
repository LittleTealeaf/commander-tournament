mod neighbors;

use std::collections::{HashMap, HashSet};

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    error::TournamentError,
    game::{POD_SIZE, matchup::Matchup},
    matchmaking::MatchmakingPlayer,
    player::PlayerId,
    tournament::Tournament,
    utils::IntoCopiedIter,
};

#[derive(Debug, Clone, Copy)]
pub struct Matchmaker<'a>(&'a Tournament);

impl Tournament {
    #[must_use]
    pub const fn matchmaker(&self) -> Matchmaker<'_> {
        Matchmaker(self)
    }
}

impl Matchmaker<'_> {
    pub fn create_match(self, player: MatchmakingPlayer) -> Result<Matchup, TournamentError> {
        let Some(root_player) = self.get_root_player(player) else {
            return Err(TournamentError::NotEnoughPlayers);
        };

        let mut players = Vec::with_capacity(POD_SIZE);
        let mut aggregate_stats =
            AggregateStats::from(self.0.get_player_or_default_stats(root_player));
        players.push(root_player);

        let mut performances = self.0.analytics().player_performance(root_player)?;
        for player in self.0.players().keys() {
            performances.entry(*player).or_default();
        }
        performances.remove(&root_player);

        for _ in 1..POD_SIZE {
            #[allow(clippy::cast_precision_loss)]
            let player = self
                .get_next_player(&aggregate_stats, &performances)
                .ok_or(TournamentError::NotEnoughPlayers)?;
            players.push(player);
            performances.remove(&player);
            for (pl, per) in self.0.analytics().player_performance(player)? {
                performances.entry(pl).and_modify(|entry| *entry += per);
            }
            aggregate_stats += self.0.get_player_or_default_stats(player);
        }

        let players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.0.create_match(players)
    }

    fn get_root_player(self, player: MatchmakingPlayer) -> Option<PlayerId> {
        match player {
            MatchmakingPlayer::Player(player_id) => Some(player_id),
            MatchmakingPlayer::LongestBreak => self.longest_break(),
            MatchmakingPlayer::LeastPlayed => self
                .0
                .get_registered_players()
                .min_by_key(|player| (player.stats().games(), player.id()))
                .map(|player| player.id()),
        }
    }

    fn longest_break(self) -> Option<PlayerId> {
        let mut players = self.0.players().keys().copied().collect::<HashSet<_>>();

        for player in &players {
            if self.0.get_player_or_default_stats(*player).games() == 0 {
                return Some(*player);
            }
        }

        for game in self.0.games().iter().rev() {
            for player in game.players() {
                if players.len() == 1 {
                    return players.into_iter().next();
                }

                players.remove(&player.id());
            }
        }

        players.into_iter().next()
    }

    fn get_next_player(
        self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> Option<PlayerId> {
        let games_played_ranked = self.ranked_games_played(agg_stats, performances);
        let neighbors_ranked = self.ranked_neighbors(agg_stats, performances);
        let combined = chain!(games_played_ranked, neighbors_ranked);
        let scores = combined.into_grouping_map().sum();
        Some(scores.into_iter().min_by_key(|(_, n)| *n)?.0)
    }

    fn ranked_games_played(
        self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        let avg_elo = agg_stats
            .avg_elo()
            .unwrap_or_else(|| self.0.default_stats().elo());
        let config = self.0.matchmaker_config();
        let ranking = self.0.ranking();

        chain!(
            to_weight_rank(
                ranking
                    .sort_lost_with(avg_elo, performances.iter_copied())
                    .map(|(id, _)| id),
                config.player_lost_with
            ),
            to_weight_rank(
                ranking
                    .sort_nemesis(avg_elo, performances.iter_copied())
                    .map(|(id, _)| id),
                config.player_nemesis
            ),
            to_weight_rank(
                ranking
                    .sort_least_played(avg_elo, performances.iter_copied())
                    .map(|(id, _)| id),
                config.player_least_played
            )
        )
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

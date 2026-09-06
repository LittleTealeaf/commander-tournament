mod next_player;
mod pool;

use core::ops::AddAssign;

use itertools::Itertools;

use crate::{
    analytics::aggregate::AggregateStats,
    config::matchmaker::MatchmakerConfig,
    error::TournamentError,
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

#[derive(Debug, Clone, getset::WithSetters)]
pub struct Matchmaker<'a> {
    tourn: &'a Tournament,
    #[set_with]
    players: im::HashSet<PlayerId>,
    #[set_with]
    config: MatchmakerConfig,
}

impl Tournament {
    #[must_use]
    pub fn matchmaker(&self) -> Matchmaker<'_> {
        Matchmaker {
            tourn: self,
            players: self
                .players
                .iter()
                .filter_map(|(id, info)| (!info.is_archived()).then_some(*id))
                .collect(),
            config: self.config.matchmaker_config().clone(),
        }
    }
}

impl Matchmaker<'_> {
    pub fn create_match(&self, player: PlayerId) -> Result<Matchup, TournamentError> {
        let pool = self.create_player_pool(player);
        let analytics = self.tourn.analytics();

        let mut aggregate_stats = AggregateStats::from(self.tourn.get_player_or_default_stats(player));
        let mut performances = analytics.player_performance_all_others(player)?;
        performances.retain(|id, _| pool.contains(id));

        let mut players = Vec::with_capacity(POD_SIZE);
        players.push(player);

        for _ in 1..POD_SIZE {
            let Some(id) = self.next_player(&aggregate_stats, &performances) else {
                return Err(TournamentError::NotEnoughPlayers);
            };

            players.push(id);
            performances.remove(&id);

            for (pl, per) in analytics.player_performance(id).into_iter().flatten() {
                performances.entry(pl).and_modify(|entry| entry.add_assign(per));
            }
            aggregate_stats += self.tourn.get_player_or_default_stats(id);
        }

        let game_players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.tourn.create_match(game_players)
    }
}

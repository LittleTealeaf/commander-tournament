use std::collections::HashMap;

use crate::{
    Tournament,
    analytics::winloss::{AggregatePerformance, GetAggregatePerformance},
    player::RegisteredPlayer,
};

impl<'a> GetAggregatePerformance<RegisteredPlayer<'a>> for &'a Tournament {
    fn get_player_aggregate_performance(
        &self,
        id: u32,
    ) -> Result<
        impl Iterator<Item = super::AggregatePerformance<RegisteredPlayer<'a>>>,
        crate::error::TournamentError,
    > {
        self.require_id_registered(id)?;

        let mut records = self
            .get_registered_players()
            .map(|value| (value.id(), AggregatePerformance::new(value)))
            .collect::<HashMap<_, _>>();

        for game in self.get_player_games(id)? {
            let winner = game.winner();
            let losers = game.losers();

            if winner == id {
                for player in losers {
                    records.entry(player).and_modify(|entry| {
                        entry.games_played += 1;
                        entry.games_won += 1;
                    });
                }
            } else {
                records.entry(winner).and_modify(|entry| {
                    entry.games_played += 1;
                    entry.games_lost += 1;
                });
                for player in losers {
                    if player == id {
                        continue;
                    }
                    records.entry(player).and_modify(|entry| {
                        entry.games_played += 1;
                    });
                }
            }
        }
        records.remove(&id);

        Ok(records.into_values())
    }
}


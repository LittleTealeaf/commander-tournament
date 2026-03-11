use std::collections::HashMap;

use crate::{
    Tournament, analytics::winloss::MatchPerformance, error::TournamentError,
    player::RegisteredPlayer,
};

impl Tournament {
    pub fn player_get_player_match_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        self.require_id_registered(id)?;

        let mut records = self
            .get_registered_players()
            .map(|value| (value.id(), (value, MatchPerformance::default())))
            .collect::<HashMap<_, _>>();

        for game in self.get_player_games(id)? {
            let winner = game.winner();
            let losers = game.losers();

            if winner == id {
                for player in losers {
                    records.entry(player).and_modify(|(_, perf)| {
                        perf.add_win();
                    });
                }
            } else {
                records.entry(winner).and_modify(|(_, perf)| {
                    perf.add_loss();
                });
                for player in losers {
                    if player == id {
                        continue;
                    }
                    records.entry(player).and_modify(|(_, perf)| {
                        perf.add_draw();
                    });
                }
            }
        }
        records.remove(&id);

        Ok(records.into_values())
    }
}

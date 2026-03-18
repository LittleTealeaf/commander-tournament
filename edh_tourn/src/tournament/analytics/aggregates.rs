use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    analytics::aggregate::AggregateStats,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::Tournament,
};

impl Tournament {
    pub fn get_aggregated_player_stats(
        &self,
    ) -> impl Iterator<Item = (RegisteredPlayer<'_>, AggregateStats)> {
        self.get_registered_players()
            .map(|player| (player, player.stats().into()))
    }

    #[must_use]
    pub fn get_aggregated_identity_stats(&self) -> HashMap<ColorIdentity, AggregateStats> {
        self.get_aggregated_player_stats()
            .map(|(player, stats)| (*player.info().color_identity(), stats))
            .into_grouping_map()
            .sum()
    }

    #[must_use]
    pub fn get_aggregated_color_stats(&self) -> HashMap<MtgColor, AggregateStats> {
        self.get_aggregated_player_stats()
            .flat_map(|(player, stats)| {
                player
                    .info()
                    .color_identity()
                    .into_colors()
                    .map(move |color| (color, stats.clone()))
            })
            .into_grouping_map()
            .sum()
    }
}

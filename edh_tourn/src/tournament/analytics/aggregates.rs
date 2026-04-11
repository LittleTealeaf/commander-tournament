use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    analytics::aggregate::AggregateStats,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::analytics::Analytics,
};

impl<'a> Analytics<'a> {
    #[must_use]
    pub fn aggregated_identity_stats(self) -> HashMap<ColorIdentity, AggregateStats> {
        self.aggregated_player_stats()
            .map(|(player, stats)| (player.info().color_identity(), stats))
            .into_grouping_map()
            .sum()
    }

    #[must_use]
    pub fn aggregated_color_stats(self) -> HashMap<MtgColor, AggregateStats> {
        self.aggregated_player_stats()
            .flat_map(|(player, stats)| {
                player
                    .info()
                    .color_identity()
                    .colors()
                    .map(move |color| (color, stats.clone()))
            })
            .into_grouping_map()
            .sum()
    }

    fn aggregated_player_stats(
        self,
    ) -> impl Iterator<Item = (RegisteredPlayer<'a>, AggregateStats)> {
        self.0
            .get_registered_players()
            .map(|player| (player, player.stats().into()))
    }
}

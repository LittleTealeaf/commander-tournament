mod best_by;
mod home;

use edh_tourn::tournament::Tournament;

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate},
};

#[derive(Debug, Clone, Default)]
pub struct TournamentStatsView {
    tab: StatsTab,
    best_deck_aggregate: AggregateCategory,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatsTab {
    #[default]
    Home,
    BestDecks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AggregateCategory {
    #[default]
    Identity,
    Color,
}

#[derive(Debug, Clone)]
pub enum TournStatsMsg {
    SetTab(StatsTab),
    SetBestDeckAggregate(AggregateCategory),
}

#[derive(Debug, Clone)]
pub enum TournStatsOut {}

impl Component for TournamentStatsView {
    type Message = TournStatsMsg;
    type OutMessage = TournStatsOut;
}

impl ComponentUpdate for TournamentStatsView {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            TournStatsMsg::SetTab(stats_tab) => {
                self.tab = stats_tab;
                Effect::done()
            }
            TournStatsMsg::SetBestDeckAggregate(aggregate_category) => {
                self.best_deck_aggregate = aggregate_category;
                Effect::done()
            }
        }
    }
}

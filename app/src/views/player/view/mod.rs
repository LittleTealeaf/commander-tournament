mod info;
mod stats;

use edh_tourn::tournament::Tournament;
use iced::{
    Alignment, Length,
    widget::{column, container, row, text},
};

use crate::{
    components::tab_bar,
    traits::ComponentView,
    views::player::{
        PlayerDetailsMsg, StatsTab,
        view::{
            info::view_info_panel,
            stats::{
                stats_color_matchups, stats_game_history, stats_identity_matchups, stats_player_matchups,
                stats_summary,
            },
        },
    },
};

impl ComponentView for super::PlayerView {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let info_panel = view_info_panel(self).max_width(700);

        let deck_progress = self.id.map(|id| {
            column![
                stats_summary(context.get_player_or_default_stats(id)),
                tab_bar(
                    &self.stats,
                    StatsTab::VALUES,
                    crate::views::player::PlayerDetailsMsg::SetStatsTab
                ),
                match self.stats {
                    StatsTab::Games => stats_game_history(context, id),
                    StatsTab::Players => stats_player_matchups(context, id),
                    StatsTab::Identities => stats_identity_matchups(context, id),
                    StatsTab::Colors => stats_color_matchups(context, id),
                }
                .unwrap_or_else(|| {
                    container(
                        text("No Stats Available")
                            .size(25)
                            .width(Length::Fill)
                            .align_x(Alignment::Center),
                    )
                })
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .spacing(30)
        });

        container(
            row![info_panel, deck_progress]
                .height(Length::Fill)
                .spacing(40)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .into()
    }
}

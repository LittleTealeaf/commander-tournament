mod info;
mod stats;

use iced::{
    Alignment, Length,
    widget::{button, column, container, row, space, text},
};
use iced_aw::{TabBar, TabLabel};

use crate::{
    traits::ComponentView,
    views::player_details::{
        Message, StatsTab,
        view::{
            info::view_info_panel,
            stats::{
                stats_color_matchups, stats_game_history, stats_identity_matchups,
                stats_player_matchups, stats_summary,
            },
        },
    },
};

impl ComponentView for super::State {
    fn view<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        if let Some(prompt) = &self.prompt_confirm_delete {
            return prompt.view_into(());
        }

        let title_text = if self.id.is_none() {
            "Create New Player".to_owned()
        } else {
            self.initial_name.clone()
        };

        let title = text(title_text).width(Length::Fill).center().size(50);

        let info_panel = view_info_panel(self).max_width(700);

        let deck_progress = self.id.map(|id| {
            column![
                stats_summary(context.get_player_or_default_stats(id)),
                StatsTab::VALUES
                    .into_iter()
                    .fold(TabBar::new(super::Message::SetStatsTab), |tab_bar, tab| {
                        tab_bar.push(tab, TabLabel::Text(format!("{tab}")))
                    })
                    .set_active_tab(&self.stats),
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

        let bottom_row = row![
            self.id.is_some().then_some(
                button("Delete")
                    .style(button::danger)
                    .on_press(Message::DeletePlayer)
            ),
            space().width(Length::Fill),
            button("Close").on_press(Message::Close),
            button("Save").on_press(Message::SaveAndClose)
        ]
        .spacing(20);

        container(
            column![
                title,
                row![info_panel, deck_progress]
                    .height(Length::Fill)
                    .spacing(40),
                bottom_row
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .height(Length::Fill)
        .into()
    }
}

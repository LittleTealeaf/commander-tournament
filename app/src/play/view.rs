use edh_tourn::{ranking::RankingMethod, tournament::Tournament};
use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, pick_list, row, space, text},
};
use nerd_font_symbols::md::MD_CLOSE;

use crate::{
    play::{PlayMsg, PlayNextMode, PlayView},
    traits::ComponentView,
};

use super::PlayMode;

impl ComponentView for PlayView {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let header = match &self.mode {
            PlayMode::Player { ranking, id } => {
                let name = context
                    .get_player_name(id)
                    .map_or("Unknown Player", |name| name.as_str());

                row![
                    text(name).size(25),
                    space().width(Length::Fill),
                    column![
                        text("Ranking Method"),
                        pick_list(
                            RankingMethod::VALUES,
                            Some(ranking),
                            PlayMsg::SetRankingMethod
                        )
                    ]
                    .spacing(5)
                    .align_x(Horizontal::Left)
                ]
            }
            PlayMode::Next { ranking, mode } => row![
                text("Tournament Game").size(25),
                space().width(Length::Fill),
                column![
                    text("Select Mode"),
                    pick_list(PlayNextMode::VALUES, Some(mode), PlayMsg::SetNextMode)
                ]
                .spacing(5)
                .align_x(Horizontal::Left),
                column![
                    text("Ranking Method"),
                    pick_list(
                        RankingMethod::VALUES,
                        Some(ranking),
                        PlayMsg::SetRankingMethod
                    )
                ]
                .spacing(5)
                .align_x(Horizontal::Left),
            ]
            .spacing(10),
            PlayMode::Custom { players } => {
                let options = context.get_registered_players().collect::<Vec<_>>();
                let selectors = players.iter().enumerate().map(|(index, player)| {
                    let reg_pl = player.and_then(|id| context.get_registered_player(id));
                    pick_list(options.clone(), reg_pl, move |player| {
                        PlayMsg::SetPlayer(index, Some(player.id()))
                    })
                    .into()
                });

                row![
                    container(text("Custom Game").size(25)).align_y(Vertical::Top),
                    space().width(Length::Fill),
                    column(selectors).spacing(10)
                ]
            }
        };

        let main_content = self.match_preview.as_ref().map_or_else(
            || container(text("Match Not Created Yet")),
            |preview| container(preview.view_into(context)),
        );

        container(
            column![
                row![
                    container(button(MD_CLOSE).on_press(PlayMsg::Close)).align_y(Vertical::Top),
                    header
                ]
                .spacing(15)
                .align_y(Vertical::Top),
                main_content
            ]
            .spacing(20),
        )
        .padding(5)
        .into()
    }
}

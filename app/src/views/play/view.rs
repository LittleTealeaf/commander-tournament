use edh_tourn::tournament::Tournament;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{checkbox, column, container, pick_list, row, text},
};

use crate::{
    traits::ComponentView,
    views::play::{PlayMsg, PlayNextMode, PlayView},
};

use super::PlayMode;

impl ComponentView for PlayView {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let options = match &self.mode {
            PlayMode::Player(_) => None,
            PlayMode::Next {
                mode,
                ignore_precons,
            } => Some(
                row![
                    column![
                        text("Select Mode"),
                        pick_list(PlayNextMode::VALUES, Some(mode), PlayMsg::SetNextMode)
                    ]
                    .spacing(5)
                    .align_x(Horizontal::Left),
                    checkbox(*ignore_precons)
                        .label("Ignore Precons")
                        .on_toggle(PlayMsg::IgnorePrecons)
                ]
                .align_y(Vertical::Bottom)
                .spacing(10),
            ),
            PlayMode::Custom { players } => {
                let options = context.get_registered_players().collect::<Vec<_>>();
                let selectors = players.iter().enumerate().map(|(index, player)| {
                    let reg_pl = player.and_then(|id| context.get_registered_player(id));
                    pick_list(options.clone(), reg_pl, move |player| {
                        PlayMsg::SetPlayer(index, Some(player.id()))
                    })
                    .into()
                });

                Some(row![column(selectors).spacing(10)])
            }
        };

        let main_content = self.match_preview.as_ref().map_or_else(
            || container(text("Match Not Created Yet")),
            |preview| container(preview.view_into(context)),
        );

        container(column![options, main_content].spacing(20))
            .padding(5)
            .into()
    }
}

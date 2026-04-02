use edh_tourn::{ranking::RankingMethod, tournament::Tournament};
use iced::widget::{column, container, pick_list, row, text};

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
            PlayMode::Player { ranking, .. } => container(row![pick_list(
                RankingMethod::VALUES,
                Some(ranking),
                PlayMsg::SetRankingMethod
            )]),
            PlayMode::Next { ranking, mode } => container(row![
                pick_list(PlayNextMode::VALUES, Some(mode), PlayMsg::SetNextMode),
                pick_list(
                    RankingMethod::VALUES,
                    Some(ranking),
                    PlayMsg::SetRankingMethod
                )
            ]),
            PlayMode::Custom { .. } => todo!(),
        };

        let main_content = self.match_preview.as_ref().map_or_else(
            || container(text("Match Not Created Yet")),
            |preview| container(preview.view_into(context)),
        );

        container(column![header, main_content].spacing(20)).into()
    }
}

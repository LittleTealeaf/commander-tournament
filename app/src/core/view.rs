use crate::{App, core::message::Message, error, player_details, traits::ComponentView};

impl App {
    #[must_use]
    pub fn handle_view(&self) -> iced::Element<'_, Message> {
        self.view(())
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Error(error::State),
    PlayerDetails(player_details::State),
}

impl ComponentView for App {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        self.views.last().map_or_else(
            || self.home.view_into((&self.tournament, &self.file)),
            |view| match view {
                View::Error(error) => error.view_into(()),
                View::PlayerDetails(state) => state.view_into(&self.tournament),
            },
        )
    }
}

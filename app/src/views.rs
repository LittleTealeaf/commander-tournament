use crate::{message::Message, traits::ComponentView, App};

pub mod error;
pub mod home;
pub mod player_details;

impl App {
    #[must_use]
    pub fn handle_view(&self) -> iced::Element<'_, Message> {
        self.view(())
    }
}

#[derive(Clone, Debug)]
pub enum View {
    Error(error::State),
    PlayerDetails(player_details::State)
}

impl ComponentView for App {
    fn view<'a>(&'a self, (): Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        self.views.last().map_or_else(
            || self.home.view_into(&self.tournament),
            |view| match view {
                View::Error(error) => error.view_into(()),
                View::PlayerDetails(state) => state.view_into(&self.tournament),
            },
        )
    }
}

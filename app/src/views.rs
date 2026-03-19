use crate::{App, traits::ComponentView};

pub mod error;
pub mod home;
pub mod player_details;

#[derive(Clone, Debug)]
pub enum View {
    Error(error::State),
}

impl ComponentView for App {
    fn view<'a>(&'a self, (): Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        self.views.last().map_or_else(
            || self.home.view_into(&self.tournament),
            |view| match view {
                View::Error(error) => error.view_into(()),
            },
        )
    }
}

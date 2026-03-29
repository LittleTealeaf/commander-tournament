
use crate::{
    App, app::message::Message, error::Error, player_details::PlayerDetails, traits::ComponentView,
};

impl App {
    #[must_use]
    pub fn handle_view(&self) -> iced::Element<'_, Message> {
        self.view(())
    }

    #[must_use]
    pub fn get_view(&self) -> Option<&View> {
        self.views.last()
    }

    pub fn get_view_mut(&mut self) -> Option<&mut View> {
        self.views.last_mut()
    }

    pub fn push_view<V>(&mut self, view: V)
    where
        V: Into<View>,
    {
        self.views.push(view.into());
    }

    pub fn pop_view(&mut self) {
        let _ = self.views.pop();
    }

    pub fn clear_views(&mut self) {
        self.views.clear();
    }

    pub fn display_error(&mut self, error: String) {
        self.push_view(crate::error::Error::new(error));
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Error(Error),
    PlayerDetails(PlayerDetails),
}

impl ComponentView for App {
    type ViewContext<'a>
        = ()
    where
        Self: 'a;
    fn view<'a>(&'a self, (): Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        self.get_view().map_or_else(
            || self.home.view_into((&self.tournament, &self.file)),
            |view| match view {
                View::Error(error) => error.view_into(()),
                View::PlayerDetails(state) => state.view_into(&self.tournament),
            },
        )
    }
}

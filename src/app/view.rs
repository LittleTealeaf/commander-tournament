use crate::app::{App, components::leaderboard::LeaderboardComponent};
use iced::Element;

use crate::app::{message::Message, traits::View};

impl View for App {
    fn view(app: &App) -> Element<'_, Message> {
        LeaderboardComponent::view(app)
    }
}

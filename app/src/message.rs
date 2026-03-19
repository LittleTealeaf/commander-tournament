use crate::{services::tournament, views::home};

#[derive(Debug, Clone)]
pub enum Message {
    Tournament(tournament::Action),
    Home(home::Message),
}

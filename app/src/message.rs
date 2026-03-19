use crate::{services::tournament, views::home};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Tournament(tournament::Action),
    Home(home::Message),
}

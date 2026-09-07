use std::path::PathBuf;

use iced::{
    Length,
    widget::{button, row, space},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md::MD_COG;

#[derive(Debug, Clone, Default)]
pub struct Menu;

#[derive(Debug, Clone)]
pub enum MenuMsg {
    New,
    Open,
    Save,
    SaveAs,
    OpenGameConfig,
}

impl Model for Menu {
    type Message = MenuMsg;
    type OutMessage = MenuMsg;
    type Context<'a> = &'a Option<PathBuf>;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        Signal::out(message).ok()
    }
}

impl Component for Menu {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        row![
            button("New").on_press(MenuMsg::New).style(button::subtle),
            button("Open").on_press(MenuMsg::Open).style(button::subtle),
            button("SaveAs").on_press(MenuMsg::SaveAs).style(button::subtle),
            button("Save")
                .on_press_maybe(context.is_some().then_some(MenuMsg::Save))
                .style(button::subtle),
            space().width(Length::Fill),
            button(MD_COG).on_press(MenuMsg::OpenGameConfig),
        ]
        .spacing(5)
        .into()
    }
}

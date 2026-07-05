use std::path::PathBuf;

use iced::{
    Length,
    widget::{button, row, rule, space},
};
use iced_aw::widget::InnerBounds::Padding;
use nerd_font_symbols::md::MD_COG;

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

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

impl Component for Menu {
    type Message = MenuMsg;
    type OutMessage = MenuMsg;
}

impl ComponentUpdate for Menu {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        Effect::out(message).ok()
    }
}

impl ComponentView for Menu {
    type ViewContext<'a>
        = &'a Option<PathBuf>
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
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

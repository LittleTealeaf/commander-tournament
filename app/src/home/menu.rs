use std::path::PathBuf;

use iced::{
    Length,
    widget::{button, row, space},
};
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
    OpenPlayNext,
    OpenPlayCustom,
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
            button("New").on_press(MenuMsg::New),
            button("Open").on_press(MenuMsg::Open),
            button("SaveAs").on_press(MenuMsg::SaveAs),
            button("Save").on_press_maybe(context.is_some().then_some(MenuMsg::Save)),
            space().width(Length::Fill),
            button(MD_COG).on_press(MenuMsg::OpenGameConfig),
            button("Custom Game").on_press(MenuMsg::OpenPlayCustom),
            button("Next Game").on_press(MenuMsg::OpenPlayNext),
        ]
        .spacing(5)
        .into()
    }
}

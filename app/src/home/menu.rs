use std::path::PathBuf;

use iced::widget::{button, row};

use crate::{
    core::file::FileAction, effect::Effect, traits::{Component, ComponentUpdate, ComponentView}
};

#[derive(Debug, Clone, Default)]
pub struct Menu;

#[derive(Debug, Clone)]
pub enum MenuMsg {
    New,
    Open,
    Save,
    SaveAs,
}

impl Component for Menu {
    type Message = MenuMsg;
    type OutMessage = ();
}

impl ComponentUpdate for Menu {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            MenuMsg::New => Effect::global(FileAction::New),
            MenuMsg::Open => Effect::global(FileAction::Open),
            MenuMsg::Save => Effect::global(FileAction::Save),
            MenuMsg::SaveAs => Effect::global(FileAction::SaveAs),
        }
        .ok()
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
        ]
        .spacing(5)
        .into()
    }
}

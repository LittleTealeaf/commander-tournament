pub mod play;
pub mod player;
pub mod matchmaker_config;

use core::iter::{empty, once};

use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{Button, button, column, row, space, text},
};
use nerd_font_symbols::md::MD_CLOSE;

use crate::{fonts::FONT_BOLD, traits::ComponentView};

pub trait ViewScreen: ComponentView {
    const CLOSE_MESSAGE: Self::Message;
    const ON_RESUME: Option<Self::Message> = None;

    fn title<'a>(&'a self, context: Self::ViewContext<'a>) -> String;

    #[allow(unused)]
    fn primary_actions<'a>(
        &'a self,
        context: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = Button<'a, Self::Message>> {
        empty()
    }

    #[allow(unused)]
    fn secondary_actions<'a>(
        &'a self,
        context: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = Button<'a, Self::Message>> {
        empty()
    }

    fn screen_view<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::Message>
    where
        Self::ViewContext<'a>: Clone,
    {
        column![
            row![
                row(once(button(MD_CLOSE).on_press(Self::CLOSE_MESSAGE))
                    .chain(self.primary_actions(context.clone()))
                    .map(Into::into))
                .spacing(5),
                space().width(10),
                text(self.title(context.clone())).size(30).font(FONT_BOLD),
                space().width(Length::Fill),
                row(self
                    .secondary_actions(context.clone())
                    .into_iter()
                    .map(Into::into))
                .spacing(5),
            ]
            .spacing(5)
            .align_y(Vertical::Top),
            self.view(context)
        ]
        .spacing(20)
        .padding(10)
        .into()
    }

    fn screen_view_into<'a, M>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a + Clone,
        Self::ViewContext<'a>: Clone,
    {
        self.screen_view(context).map(Into::into)
    }
}

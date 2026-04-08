use core::iter::{empty, once};

use iced::{
    Element, Length,
    alignment::Vertical,
    font::Weight,
    widget::{Button, button, column, row, space, text},
};
use iced_futures::MaybeSend;
use nerd_font_symbols::md::MD_CLOSE;

use crate::{
    effect::Effect,
    style::{FontBuilder, font_default},
};

pub trait Component {
    type OutMessage;
    type Message: Clone + Send + 'static;
}

pub trait ComponentView: Component {
    type ViewContext<'a>
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, Self::Message>;

    fn view_into<'a, M>(&'a self, context: Self::ViewContext<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a,
    {
        self.view(context).map(Into::into)
    }
}

pub trait ComponentUpdate: Component {
    type UpdateContext<'a>;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;

    fn mapped_update<M, O, F>(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
        map_out: F,
    ) -> anyhow::Result<Effect<M, O>>
    where
        F: Fn(Self::OutMessage) -> anyhow::Result<Effect<M, O>>,
        Self::Message: Into<M> + 'static + MaybeSend,
        M: Send + MaybeSend + 'static,
    {
        self.update(message, context)?.map(map_out)
    }

    fn empty_update<M, O>(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<M, O>>
    where
        Self: Component<OutMessage = ()>,
        M: Send + MaybeSend + 'static,
        Self::Message: Into<M>,
    {
        self.update(message, context)?.map_empty()
    }
}

pub trait HandleMessage<M>: ComponentUpdate {
    fn handle_message(
        &mut self,
        message: M,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}

impl<T: Component + ComponentUpdate> HandleMessage<T::Message> for T {
    fn handle_message(
        &mut self,
        message: T::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.update(message, context)
    }
}

pub trait ViewScreen: ComponentView {
    const CLOSE_MESSAGE: Self::Message;

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
                text(self.title(context.clone()))
                    .size(30)
                    .font(font_default().weight(Weight::Bold)),
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

use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, row, space, text},
};
use iced_futures::MaybeSend;
use nerd_font_symbols::md::{MD_CLOSE, MD_CONTENT_SAVE, MD_DELETE};

use crate::effect::Effect;

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
    fn title<'a>(&'a self, context: Self::ViewContext<'a>) -> String;
    fn save(&self) -> Option<Self::Message> {
        None
    }
    fn delete(&self) -> Option<Self::Message> {
        None
    }

    fn screen_view<'a, M>(&'a self, context: Self::ViewContext<'a>, on_close: M) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a + Clone,
        Self::ViewContext<'a>: Clone,
    {
        column![
            row![
                button(MD_CLOSE).on_press(on_close),
                self.save().map(
                    |on_save| Element::from(button(MD_CONTENT_SAVE).on_press(on_save))
                        .map(Into::into)
                ),
                space().width(10),
                text(self.title(context.clone())).size(30),
                space().width(Length::Fill),
                self.delete().map(|on_delete| Element::from(
                    button(MD_DELETE).on_press(on_delete).style(button::danger)
                )
                .map(Into::into)),
            ]
            .spacing(5)
            .align_y(Vertical::Top),
            self.view_into(context)
        ]
        .spacing(20)
        .padding(10)
        .into()
    }
}

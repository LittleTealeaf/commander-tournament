use iced::Element;
use iced_futures::MaybeSend;

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
        F: FnMut(Self::OutMessage) -> anyhow::Result<Effect<M, O>>,
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

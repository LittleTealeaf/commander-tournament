use crate::{
    app::Message, effect::Effect, traits::{Component, ComponentUpdate, ComponentView, Mapped}
};

#[derive(Debug)]
pub struct ConfirmModal<M> {
    title: String,
    details: String,
    on_confirm: M,
    on_cancel: Option<M>,
}

impl<M> ConfirmModal<M> {
    pub fn new<T, D, C>(title: T, details: D, confirm: M, cancel: C) -> Self
    where
        T: ToString,
        D: ToString,
        C: Into<Option<M>>,
    {
        Self {
            title: title.to_string(),
            details: details.to_string(),
            on_confirm: confirm,
            on_cancel: cancel.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConfirmMsg {
    Confirm,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum ConfirmOut<M>
where
    M: Clone,
{
    Confirm(M),
    Cancel(Option<M>),
}

impl<M, N> Mapped<ConfirmModal<N>> for ConfirmModal<M>
where
    M: Into<N>,
{
    fn map(self) -> ConfirmModal<N> {
        ConfirmModal {
            title: self.title,
            details: self.details,
            on_confirm: self.on_confirm.into(),
            on_cancel: self.on_cancel.map(Into::into),
        }
    }
}

impl<M> Component for ConfirmModal<M>
where
    M: Clone,
{
    type Message = ConfirmMsg;
    type OutMessage = ConfirmOut<M>;
}

impl<M> ComponentUpdate for ConfirmModal<M>
where
    M: Clone,
{
    type UpdateContext<'a> = ();

    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        Effect::out(match message {
            ConfirmMsg::Confirm => ConfirmOut::Confirm(self.on_confirm.clone()),
            ConfirmMsg::Cancel => ConfirmOut::Cancel(self.on_cancel.clone()),
        })
        .ok()
    }
}

impl ComponentView for ConfirmModal<Message> {
   type ViewContext<'a> = ()
        where
            Self: 'a; 
   fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
       
       todo!()
   }
}

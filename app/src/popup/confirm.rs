use iced::widget::{button, text};

use crate::popup::Popup;

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct ConfirmPopup<Msg> {
    title: String,
    description: String,
    on_success: Msg,
    on_cancel: Msg,
}

impl<Msg> ConfirmPopup<Msg>
where
    Msg: Clone,
{
    pub fn to_popup(&self) -> Popup<'_, Msg> {
        Popup {
            title: &self.title,
            content: text(&self.description).into(),
            actions: vec![
                button("Cancel").on_press(self.on_cancel.clone()).into(),
                button("Confirm").on_press(self.on_success.clone()).into(),
            ],
        }
    }
}

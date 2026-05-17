use iced::widget::{button, text};

use crate::popup::Popup;

#[derive(Debug)]
pub struct ConfirmPopup<Msg> {
    title: String,
    description: String,
    on_success: Msg,
    on_cancel: Option<Msg>,
}

impl<Msg> ConfirmPopup<Msg>
where
    Msg: Clone,
{
    fn to_popup(&self) -> Popup<'_, Msg> {
        Popup {
            title: self.title.clone(),
            content: text(&self.description).into(),
            actions: vec![
                button("Cancel").on_press_maybe(self.on_cancel.clone()).into(),
                button("Confirm").on_press(self.on_success.clone()).into(),
            ],
        }
    }
}

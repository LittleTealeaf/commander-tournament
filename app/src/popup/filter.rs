use edh_tourn::{player::PlayerId, tournament::Tournament};
use iced::widget::{button, text};
use im::OrdSet;

use crate::popup::Popup;

#[derive(Debug)]
pub struct FilterPopup<Msg> {
    selected: OrdSet<PlayerId>,
    on_submit: Msg,
    on_cancel: Msg,
}

impl<Msg> FilterPopup<Msg>
where
    Msg: Clone,
{
    pub fn to_popup(&self, _tournament: &Tournament) -> Popup<'_, Msg> {
        Popup {
            title: "Filter Players",
            content: text("hi").into(),
            actions: vec![
                button("Cancel").on_press(self.on_cancel.clone()).into(),
                button("Filter").on_press(self.on_submit.clone()).into(),
            ],
        }
    }
}


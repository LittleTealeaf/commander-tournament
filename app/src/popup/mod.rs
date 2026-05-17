pub mod confirm;

use iced::{
    Color, Element, Length,
    widget::{center, column, container, opaque, row, space, stack, text},
};

use crate::fonts::FONT_BOLD;

#[allow(missing_debug_implementations, reason = "Element does not implement Debug")]
pub struct Popup<'a, Msg> {
    title: String,
    content: Element<'a, Msg>,
    actions: Vec<Element<'a, Msg>>,
}

impl<'a, Msg> Popup<'a, Msg>
where
    Msg: Clone + 'a,
{
    pub fn overlay<Screen>(self, screen: Screen) -> Element<'a, Msg>
    where
        Screen: Into<Element<'a, Msg>>,
    {
        let popup = container(column![
            text(self.title).font(FONT_BOLD).size(20),
            self.content,
            row![space().width(Length::Fill), row(self.actions)]
        ]);

        let popup = center(popup).style(|style| container::Style {
            background: Some(
                Color {
                    a: 0.5,
                    ..style.palette().background
                }
                .into(),
            ),
            ..container::Style::default()
        });

        let popup = opaque(popup);

        stack![screen.into(), popup].into()
    }
}

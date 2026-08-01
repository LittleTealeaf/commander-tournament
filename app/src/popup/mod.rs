pub mod confirm;
pub mod filter_players;

use iced::{
    Color, Element,
    widget::{center, column, container, opaque, row, stack, text},
};

use crate::fonts::FONT_BOLD;

#[derive(derive_more::Constructor)]
#[allow(missing_debug_implementations, reason = "Element does not implement Debug")]
pub struct Popup<'a, Msg> {
    title: &'a str,
    content: Element<'a, Msg>,
    actions: Vec<Element<'a, Msg>>,
}

impl<'a, Msg> Popup<'a, Msg>
where
    Msg: Clone + 'a,
{
    pub fn map_into<NewMsg>(self) -> Popup<'a, NewMsg>
    where
        Msg: Into<NewMsg>,
        NewMsg: 'a,
    {
        Popup {
            title: self.title,
            content: self.content.map(Into::into),
            actions: self
                .actions
                .into_iter()
                .map(|item| item.map(Into::into))
                .collect(),
        }
    }

    pub fn overlay<Screen>(self, screen: Screen) -> Element<'a, Msg>
    where
        Screen: Into<Element<'a, Msg>>,
    {
        let popup = container(
            column![
                text(self.title).font(FONT_BOLD).size(20),
                self.content,
                row(self.actions).spacing(7)
            ]
            .spacing(10),
        )
        .padding(10)
        .style(container::rounded_box);

        let popup = center(popup).padding(75).style(|style| container::Style {
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

use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::{column, container, space, text},
};

use crate::{
    fonts::FONT_BOLD,
    style::{self, COLORS},
};

#[must_use]
pub fn empty_card<'a, M: 'a>(
    icon: &'static str,
    title: &'static str,
    subtitle: &'static str,
    cta: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let mut col = column![
        text(icon).size(32).color(COLORS.overlay1),
        text(title).font(FONT_BOLD).size(16).color(COLORS.text),
        text(subtitle).size(13).color(COLORS.subtext0),
    ]
    .spacing(8)
    .align_x(Horizontal::Center);

    if let Some(action) = cta {
        col = col.push(space::vertical().height(8)).push(action);
    }

    container(col)
        .width(Length::Fill)
        .padding(32)
        .align_x(Horizontal::Center)
        .style(style::card)
        .into()
}

#[must_use]
pub fn kpi_card<'a, M: 'a>(
    label: &'static str,
    value: String,
    subtitle: String,
    accent: catppuccin::Color,
) -> Element<'a, M> {
    container(
        column![
            text(label).size(12).color(COLORS.subtext0),
            text(value).font(FONT_BOLD).size(24).color(accent),
            text(subtitle).size(12).color(COLORS.subtext1),
        ]
        .spacing(4),
    )
    .width(Length::FillPortion(1))
    .padding(16)
    .style(style::card)
    .into()
}

use iced::{
    Background, Border, Element, Length,
    widget::{container, row, space},
};

use crate::style::COLORS;

#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Bounded 0..=100 percentage"
)]
pub fn progress_bar<'a, M: 'a>(ratio: f64, accent: catppuccin::Color) -> Element<'a, M> {
    let pct = (ratio.clamp(0.0, 1.0) * 100.0).round() as u16;
    let remaining = 100_u16.saturating_sub(pct);

    let mut bar = row![].height(Length::Fixed(8.0)).width(Length::Fill);
    if pct > 0 {
        bar = bar.push(
            container(space::horizontal())
                .width(Length::FillPortion(pct))
                .height(Length::Fill)
                .style(move |_theme| container::Style {
                    background: Some(Background::Color(accent.into())),
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        );
    }
    if remaining > 0 {
        bar = bar.push(space::horizontal().width(Length::FillPortion(remaining)));
    }

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(8.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(COLORS.base.into())),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

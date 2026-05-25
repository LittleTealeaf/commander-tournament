use core::fmt::Display;

use iced::{
    Background, Length, Theme,
    widget::{
        Container,
        button::{Button, Status, Style},
        container, row, text,
    },
};

pub fn tab_bar<'a, Val, Opts, Msg, Sel>(selected: &'a Val, tabs: Opts, on_select: Sel) -> Container<'a, Msg>
where
    Val: Display + 'a + Eq,
    Opts: IntoIterator<Item = Val>,
    Sel: Fn(Val) -> Msg + 'a,
    Msg: Clone + 'a,
{
    container(
        row(tabs.into_iter().map(|col| {
            let is_selected = &col == selected;
            Button::new(text(format!("{col}")))
                .width(Length::Fill)
                .style(move |theme: &Theme, status: Status| {
                    let palette = theme.extended_palette();

                    Style {
                        text_color: palette.primary.base.text,
                        background: Some(Background::Color({
                            if is_selected {
                                palette.primary.strong.color.scale_alpha(0.9)
                            } else if matches!(status, Status::Hovered) {
                                palette.primary.strong.color
                            } else {
                                palette.primary.base.color
                            }
                        })),
                        ..Style::default()
                    }
                })
                .on_press(on_select(col))
                .into()
        }))
        .spacing(1),
    )
    .width(Length::Fill)
}

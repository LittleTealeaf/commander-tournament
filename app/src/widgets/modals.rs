use iced::{
    Color, Element,
    widget::{button, center, column, container, opaque, row, stack, text},
};

pub fn confirm_modal<'a, Message, Content, Title, Details>(
    content: Content,
    title: Title,
    details: Details,
    on_confirm: Message,
    on_cancel: Message,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
    Content: Into<Element<'a, Message>>,
    Title: text::IntoFragment<'a>,
    Details: text::IntoFragment<'a>,
{
    stack![
        content.into(),
        opaque(
            center(column![
                text(title).size(20),
                text(details),
                row![
                    button("Confirm").on_press(on_confirm),
                    button("Cancel").on_press(on_cancel)
                ]
            ])
            .style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            })
        )
    ]
    .into()
}

use iced::{
    Length, Padding,
    widget::{Container, container, scrollable, table::Table},
};

#[must_use]
pub fn scrollable_table<'a, M>(table: Table<'a, M>) -> Container<'a, M>
where
    M: 'a,
{
    container(
        scrollable(container(table.width(Length::Fill)).padding(Padding::new(0.0).right(15)))
            .width(Length::Fill)
            .height(Length::Fill),
    )
}

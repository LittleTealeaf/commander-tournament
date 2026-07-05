use iced::{
    Length,
    widget::{Container, container, scrollable, table::Table},
};

#[must_use]
pub fn scrollable_table<'a, M>(table: Table<'a, M>) -> Container<'a, M>
where
    M: 'a,
{
    container(
        scrollable(table.width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
}

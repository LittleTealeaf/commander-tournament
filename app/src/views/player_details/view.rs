use crate::traits::ComponentView;


impl ComponentView for super::State {
    fn view<'a>(&'a self, _context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        todo!()
    }
}

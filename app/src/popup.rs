use iced::Element;

#[allow(missing_debug_implementations, reason = "Element does not implement Debug")]
pub struct Popup<'a, Msg> {
    title: String,
    content: Element<'a, Msg>,
    actions: Vec<Element<'a, Msg>>,
}

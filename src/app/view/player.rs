use commander_tournament::{
    Tournament,
    error::TournamentError,
    info::{MtgColor, PlayerInfo},
};
use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, column, container, row, space, text, text_editor, text_input},
};

use crate::app::{message::Message, traits::HandleMessage};

#[derive(Clone)]
pub struct EditPlayer {
    id: Option<u32>,
    info: PlayerInfo,
    description: text_editor::Content,
}

#[derive(Clone)]
pub enum EditPlayerMessage {
    SetName(String),
    ToggleColor(MtgColor),
    SetMoxifledId(String),
    DescriptionAction(text_editor::Action),
}

impl From<EditPlayerMessage> for Message {
    fn from(value: EditPlayerMessage) -> Self {
        Message::EditPlayerAction(value)
    }
}

impl EditPlayer {
    pub fn create(tournament: &Tournament, id: Option<u32>) -> Self {
        let info = id
            .and_then(|id| tournament.get_player_info(id).ok())
            .unwrap_or_default();
        let description = text_editor::Content::with_text(info.description());

        Self {
            id,
            info,
            description,
        }
    }

    pub fn submit(&mut self, tournament: &mut Tournament) -> Result<(), TournamentError> {
        self.info.set_description(self.description.text());
        self.info.set_name(self.info.name().trim().to_owned());
        let id = match self.id {
            Some(id) => id,
            None => tournament.register_player(self.info.name().to_string())?,
        };

        tournament.set_player_info(id, self.info.clone())
    }

    pub fn view(&self) -> Element<'_, Message> {
        let empty_str = String::new();

        let title = if self.id.is_some() {
            format!("Editing {}", self.info.name())
        } else {
            String::from("Creating New Player")
        };

        let row_colors = {
            row(MtgColor::COLORS.map(|color| {
                let is_selected = self.info.has_color(&color);
                let button_theme = if is_selected {
                    button::primary
                } else {
                    button::secondary
                };

                button(text(color.letter().to_owned()))
                    .style(button_theme)
                    .on_press(EditPlayerMessage::ToggleColor(color).into())
                    .width(Length::Fixed(40.0))
                    .height(Length::Fixed(40.0))
                    .into()
            }))
            .width(Length::Fill)
        };

        let content = column![
            text(title).size(18),
            space().height(10),
            text("Deck Name:"),
            text_input("Name", self.info.name())
                .on_input(|name| EditPlayerMessage::SetName(name).into()),
            space().height(5),
            text("Deck Description"),
            text_editor(&self.description)
                .on_action(|action| EditPlayerMessage::DescriptionAction(action).into()),
            text("Moxfield Link:"),
            row![
                text_input(
                    "https://moxfield.com/decks/...",
                    self.info.moxfield_id().unwrap_or(&empty_str).as_str()
                )
                .on_input(|s| EditPlayerMessage::SetMoxifledId(s).into())
                .width(Length::Fill),
                button(text("Deck"))
                    .on_press_maybe(self.info.moxfield_link().map(Message::OpenLink)),
                button(text("Goldfish"))
                    .on_press_maybe(self.info.moxfield_goldfish_link().map(Message::OpenLink))
            ]
            .padding(5)
            .width(Length::Fill),
            space().height(5),
            text("Colors"),
            row_colors,
            space().height(10),
            row![
                space().width(Length::Fill),
                button("Cancel").on_press(Message::CloseEditPlayer(false)),
                button("Save").on_press(Message::CloseEditPlayer(true))
            ]
        ]
        .width(Length::Fixed(500.0));

        container(content)
            .padding(Padding::new(10f32))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl HandleMessage<EditPlayerMessage> for EditPlayer {
    fn update(&mut self, msg: EditPlayerMessage) -> anyhow::Result<Option<iced::Task<Message>>> {
        match msg {
            EditPlayerMessage::DescriptionAction(action) => self.description.perform(action),
            EditPlayerMessage::SetName(name) => self.info.set_name(name),
            EditPlayerMessage::ToggleColor(mtg_color) => self.info.toggle_color(mtg_color),
            EditPlayerMessage::SetMoxifledId(id) => {
                if id.is_empty() {
                    self.info.set_moxfield_id(None);
                } else {
                    self.info.set_moxfield_id(Some(id));
                }
            }
        }

        Ok(None)
    }
}

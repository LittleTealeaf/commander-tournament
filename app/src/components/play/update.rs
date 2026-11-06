use commander_tournament_core::{game::record::GameRecord, player::PlayerId, tournament::Tournament};

use iced_tea::{Model, Signal};

use crate::components::play::{PlayComponent, PlayMode};

#[derive(Debug, Clone)]
pub enum PlayComponentMsg {
    Submit,
    Refresh,
    OpenLink(String),
    OpenMatchLinks,
    SetWinner(PlayerId),
    ClickPlayer(PlayerId),
    SetPlayer(usize, PlayerId),
    AddPlayerSlot,
    RemovePlayerSlot(usize),
}

#[derive(Debug)]
pub enum PlayComponentOut {
    OpenPlayer(PlayerId),
    OpenLink(String),
    RecordGame(Box<GameRecord>),
}

impl Model for PlayComponent {
    type Message = PlayComponentMsg;
    type OutMessage = PlayComponentOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            PlayComponentMsg::SetPlayer(row, id) => match &mut self.mode {
                PlayMode::Player(player_id) => {
                    if row != 0 {
                        return Err(anyhow::anyhow!("Only the first player is editable"));
                    }
                    *player_id = id;
                    self.refresh(context);
                    Signal::done()
                }
                PlayMode::Custom(players) => {
                    let len = players.len();
                    let entry = players.get_mut(row).ok_or_else(|| {
                        anyhow::anyhow!("Invalid index. Got {row}, length of {len}")
                    })?;
                    *entry = Some(id);
                    self.refresh(context);
                    Signal::done()
                }
                PlayMode::Next { .. } => Signal::done(),
            },
            PlayComponentMsg::Refresh => {
                self.refresh(context);
                Signal::done()
            }
            PlayComponentMsg::Submit => {
                if let Some(preview) = &self.preview
                    && let Some(winner) = preview.winner
                {
                    Signal::out(PlayComponentOut::RecordGame(Box::new(
                        preview.matchup.clone().record(winner)?,
                    )))
                    .chain(Signal::msg(PlayComponentMsg::Refresh))
                    .ok()
                } else {
                    Signal::done()
                }
            }
            PlayComponentMsg::SetWinner(player_id) => {
                if let Some(preview) = &mut self.preview {
                    preview.winner = Some(player_id);
                }
                Signal::done()
            }
            PlayComponentMsg::ClickPlayer(player_id) => {
                Signal::out(PlayComponentOut::OpenPlayer(player_id)).ok()
            }
            PlayComponentMsg::AddPlayerSlot => {
                if let PlayMode::Custom(players) = &mut self.mode {
                    players.push(None);
                    self.refresh(context);
                }
                Signal::done()
            }
            PlayComponentMsg::RemovePlayerSlot(row) => {
                if let PlayMode::Custom(players) = &mut self.mode
                    && players.len() > 2
                    && row < players.len()
                {
                    players.remove(row);
                    self.refresh(context);
                }
                Signal::done()
            }
            PlayComponentMsg::OpenLink(url) => Signal::out(PlayComponentOut::OpenLink(url)).ok(),
            PlayComponentMsg::OpenMatchLinks => {
                let Some(preview) = &self.preview else {
                    return Signal::done();
                };

                Signal::sequence(
                    context
                        .get_registered_players(preview.matchup.ids())
                        .filter_map(|player| player.info().moxfield_goldfish_link())
                        .map(|link| Signal::out(PlayComponentOut::OpenLink(link))),
                )
                .ok()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commander_tournament_core::game::POD_SIZE;

    #[test]
    fn test_add_player_slot() {
        let t = Tournament::new();
        let mut component = PlayComponent::new(PlayMode::custom(), &t);
        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(players.len(), POD_SIZE);

        component.update(PlayComponentMsg::AddPlayerSlot, &t).unwrap();
        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(players.len(), POD_SIZE + 1);
        assert_eq!(players.last(), Some(&None));
    }

    #[test]
    fn test_remove_player_slot() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();
        let p3 = t.register_player("P3").unwrap();

        let mut component = PlayComponent::new(PlayMode::Custom(vec![Some(p1), Some(p2), Some(p3)]), &t);

        // Remove slot at index 1
        component.update(PlayComponentMsg::RemovePlayerSlot(1), &t).unwrap();
        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(*players, vec![Some(p1), Some(p3)]);
    }

    #[test]
    fn test_remove_player_slot_min_limit() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();

        let mut component = PlayComponent::new(PlayMode::Custom(vec![Some(p1), Some(p2)]), &t);

        // When len == 2, removal must not occur (minimum 2 players)
        component.update(PlayComponentMsg::RemovePlayerSlot(0), &t).unwrap();
        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(players.len(), 2);
    }

    #[test]
    fn test_remove_player_slot_out_of_bounds() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();
        let p3 = t.register_player("P3").unwrap();

        let mut component = PlayComponent::new(PlayMode::Custom(vec![Some(p1), Some(p2), Some(p3)]), &t);

        // Out of bounds index
        component.update(PlayComponentMsg::RemovePlayerSlot(99), &t).unwrap();
        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(players.len(), 3);
    }

    #[test]
    fn test_set_player_slot() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();

        let mut component = PlayComponent::new(PlayMode::Custom(vec![None, None]), &t);
        component.update(PlayComponentMsg::SetPlayer(0, p1), &t).unwrap();
        component.update(PlayComponentMsg::SetPlayer(1, p2), &t).unwrap();

        let PlayMode::Custom(players) = component.mode() else {
            panic!("Expected Custom");
        };
        assert_eq!(*players, vec![Some(p1), Some(p2)]);

        // Out of bounds should return an error
        let err = component.update(PlayComponentMsg::SetPlayer(5, p1), &t);
        err.unwrap_err();
    }
}

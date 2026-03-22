use anyhow::anyhow;
use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};

use crate::{
    traits::{ComponentUpdate, Effect},
    views::home::ranking::{Message, OutMessage, State},
};

fn create_game_players<'a>(
    id: u32,
    ranked: impl IntoIterator<Item = RegisteredPlayer<'a>>,
) -> Option<[u32; 4]> {
    let mut iter = ranked.into_iter().map(|player| player.id());
    Some([id, iter.next()?, iter.next()?, iter.next()?])
}

impl ComponentUpdate for State {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayer(id) => {
                context.require_id_registered(id)?;
                self.player = Some(id);
                Effect::ok()
            }
            Message::AddTopThree => {
                let Some(id) = self.player else {
                    return Err(anyhow!("Player not specified"));
                };
                let ranked = context.get_player_ranked(id, self.method)?;
                let players =
                    create_game_players(id, ranked).ok_or_else(|| anyhow!("Not enough players"))?;

                Effect::out(OutMessage::LoadGame(players))
            }
            Message::SetMethod(method) => {
                self.method = method;
                Effect::ok()
            }
            Message::OpenPlayerDetails(id) => Effect::out(OutMessage::OpenPlayerDetails(id)),
        }
    }
}

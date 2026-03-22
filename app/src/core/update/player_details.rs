use crate::{App, core::update::player_details, traits::HandleMessage};

impl HandleMessage<crate::player_details::Message> for App {
    fn handle_message(
        &mut self,
        message: crate::player_details::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(View::PlayerDetails(state)) = self.views.last_mut() {
            state
                .handle_message(message, ())?
                .map(|message| match message {
                    crate::player_details::OutMessage::OpenPlayer(id) => {
                        self.views
                            .push(View::PlayerDetails(player_details::State::new(
                                self.tournament.get_registered_player(id),
                            )));
                        Effect::ok()
                    }
                    crate::player_details::OutMessage::SaveAndClose(maybe_id, info) => {
                        let effect = self.handle_message(
                            match maybe_id {
                                Some(id) => tournament::Action::SetPlayerInfo(id, info),
                                None => tournament::Action::Register(info),
                            },
                            (),
                        )?;
                        self.views.pop();
                        Ok(effect)
                    }
                    crate::player_details::OutMessage::Close => {
                        self.views.pop();
                        Effect::ok()
                    }
                    crate::player_details::OutMessage::DeletePlayer(id) => {
                        let effect =
                            self.handle_message(tournament::Action::DeletePlayer(id), ())?;
                        self.views.pop();
                        Ok(effect)
                    }
                })
        } else {
            Effect::ok()
        }
    }
}

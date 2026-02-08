use crate::{
    Tournament,
    error::{TournResult, TournamentError},
    game::GameRecord,
};

impl Tournament {
    fn create_or_get_id(&mut self, name: String) -> Result<u32, TournamentError> {
        match self.register_player(name) {
            Ok(id) => Ok(id),
            Err(err) => match err {
                TournamentError::PlayerAlreadyRegistered(_, id) => Ok(id),
                err => Err(err),
            },
        }
    }

    fn parse_tsv_games(&mut self, text: &str) -> impl Iterator<Item = GameRecord> {
        text.lines().filter_map(|line| {
            let parts = line.split('\t').collect::<Vec<_>>();
            if parts.len() < 5 {
                return None;
            }

            let players = [
                self.create_or_get_id(parts[0].to_string()).ok()?,
                self.create_or_get_id(parts[1].to_string()).ok()?,
                self.create_or_get_id(parts[2].to_string()).ok()?,
                self.create_or_get_id(parts[3].to_string()).ok()?,
            ];
            let winner = self.create_or_get_id(parts[4].to_string()).ok()?;

            GameRecord::new(players, winner).ok()
        })
    }

    pub fn ingest_tsv_games(&mut self, text: &str) -> TournResult<()> {
        let games = self.parse_tsv_games(text).collect::<Vec<_>>();
        for game in games {
            self.register_record(game)?;
        }
        Ok(())
    }
}

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
                self.create_or_get_id(parts.first()?.to_string()).ok()?,
                self.create_or_get_id(parts.get(1)?.to_string()).ok()?,
                self.create_or_get_id(parts.get(2)?.to_string()).ok()?,
                self.create_or_get_id(parts.get(3)?.to_string()).ok()?,
            ];
            let winner = self.create_or_get_id(parts.get(4)?.to_string()).ok()?;

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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::Tournament;

    #[test]
    fn parse_tsv_games() {
        let mut t = Tournament::new();
        let tsv = include_str!("../../tests/sample-tsv.tsv");
        let game_count = tsv.lines().count();
        let records = t.parse_tsv_games(tsv).collect_vec();
        assert_eq!(game_count, records.len());
    }

    #[test]
    fn ingest_tsv_games() {
        let mut t = Tournament::new();
        let tsv = include_str!("../../tests/sample-tsv.tsv");
        let game_count = tsv.lines().count();
        assert_eq!(0, t.games().len());
        t.ingest_tsv_games(tsv).unwrap();
        assert_eq!(game_count, t.games().len());
    }
}

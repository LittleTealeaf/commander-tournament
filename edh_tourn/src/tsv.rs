use crate::{
    Tournament,
    error::{TournResult, TournamentError},
    game::GameRecord,
};

impl Tournament {
    pub fn from_tsv_games(text: &str) -> TournResult<Self> {
        let mut tourn = Self::new();
        for line in text.lines() {
            let mut parts = line.split('\t');
            // Grabs names
            let a_name = parts.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let b_name = parts.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let c_name = parts.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let d_name = parts.next().ok_or(TournamentError::NotEnoughPlayers)?;
            let w_name = parts.next().ok_or(TournamentError::NotEnoughPlayers)?;

            // Get IDS
            let a_id = tourn.get_or_register_player(a_name.to_owned())?;
            let b_id = tourn.get_or_register_player(b_name.to_owned())?;
            let c_id = tourn.get_or_register_player(c_name.to_owned())?;
            let d_id = tourn.get_or_register_player(d_name.to_owned())?;
            let w_id = tourn.get_or_register_player(w_name.to_owned())?;

            // Create record
            let record = GameRecord::new([a_id, b_id, c_id, d_id], w_id)?;

            // Register record
            tourn.register_record(record)?;
        }

        Ok(tourn)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn parse_tsv_game_count() {
        let tsv = include_str!("../../tests/sample-tsv.tsv");
        let game_count = tsv.lines().count();
        let records = Tournament::from_tsv_games(tsv).unwrap();
        assert_eq!(game_count, records.games().len());
    }
}

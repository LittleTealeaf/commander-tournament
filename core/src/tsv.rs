use crate::{
    error::{TournResult, TournamentError},
    game::entry::GameEntry,
    tournament::Tournament,
};

impl Tournament {
    pub fn from_tsv_games(text: &str) -> TournResult<Self> {
        let mut tourn = Self::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 {
                return Err(TournamentError::NotEnoughPlayers);
            }
            let (player_names, winner_slice) = parts.split_at(parts.len() - 1);
            let winner_name = winner_slice
                .first()
                .copied()
                .ok_or(TournamentError::NotEnoughPlayers)?;
            let players = player_names
                .iter()
                .map(|name| tourn.get_or_register_player((*name).to_owned()))
                .collect::<Result<Vec<_>, _>>()?;
            let winner = tourn.get_or_register_player(winner_name.to_owned())?;
            let record = GameEntry::new(players, winner)?;
            tourn.record_entry(record)?;
        }

        Ok(tourn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tsv_game_count() {
        let tsv = include_str!("../../res/tests/sample-tsv.tsv");
        let game_count = tsv.lines().count();
        let records = Tournament::from_tsv_games(tsv).unwrap();
        assert_eq!(game_count, records.games().len());
    }

    #[test]
    fn parse_tsv_two_players() {
        let tsv = "Alice\tBob\tAlice";
        let tourn = Tournament::from_tsv_games(tsv).unwrap();
        assert_eq!(tourn.games().len(), 1);
        let game = tourn.games().first().unwrap();
        assert_eq!(game.players().len(), 2);
        let alice_id = tourn.get_player_id(&"Alice".to_owned()).unwrap();
        let bob_id = tourn.get_player_id(&"Bob".to_owned()).unwrap();
        assert_eq!(game.winner, alice_id);
        assert_eq!(game.losers(), vec![bob_id]);
    }

    #[test]
    fn parse_tsv_three_players() {
        let tsv = "Alice\tBob\tCharlie\tBob";
        let tourn = Tournament::from_tsv_games(tsv).unwrap();
        assert_eq!(tourn.games().len(), 1);
        let game = tourn.games().first().unwrap();
        assert_eq!(game.players().len(), 3);
        let alice_id = tourn.get_player_id(&"Alice".to_owned()).unwrap();
        let bob_id = tourn.get_player_id(&"Bob".to_owned()).unwrap();
        let charlie_id = tourn.get_player_id(&"Charlie".to_owned()).unwrap();
        assert_eq!(game.winner, bob_id);
        assert_eq!(game.losers(), vec![alice_id, charlie_id]);
    }

    #[test]
    fn parse_tsv_four_players() {
        let tsv = "Alice\tBob\tCharlie\tDavid\tCharlie";
        let tourn = Tournament::from_tsv_games(tsv).unwrap();
        assert_eq!(tourn.games().len(), 1);
        let game = tourn.games().first().unwrap();
        assert_eq!(game.players().len(), 4);
        let charlie_id = tourn.get_player_id(&"Charlie".to_owned()).unwrap();
        assert_eq!(game.winner, charlie_id);
    }

    #[test]
    fn parse_tsv_not_enough_players() {
        let res_single = Tournament::from_tsv_games("Alice\tAlice");
        assert!(matches!(res_single, Err(TournamentError::NotEnoughPlayers)));

        let res_one_token = Tournament::from_tsv_games("Alice");
        assert!(matches!(res_one_token, Err(TournamentError::NotEnoughPlayers)));

        let res_empty = Tournament::from_tsv_games("");
        // "" has 0 lines, so returns Ok(tourn) with 0 games
        assert_eq!(res_empty.unwrap().games().len(), 0);

        let res_empty_line = Tournament::from_tsv_games("\n");
        assert!(matches!(res_empty_line, Err(TournamentError::NotEnoughPlayers)));
    }

    #[test]
    fn parse_tsv_winner_not_in_match() {
        let tsv = "Alice\tBob\tCharlie";
        let res = Tournament::from_tsv_games(tsv);
        assert!(matches!(res, Err(TournamentError::PlayerNotInMatch(_))));
    }
}

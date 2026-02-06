use crate::tournament::{Tournament, TournamentError};
use anyhow::Result;

mod tournament;

fn main() -> Result<()> {
    let mut t = Tournament::new();
    ingest_tsv(&mut t)?;

    dbg!(t.players().collect::<Vec<_>>());

    Ok(())
}

fn ingest_tsv(tournament: &mut Tournament) -> Result<(), TournamentError> {
    let data = std::fs::read_to_string("data.tsv").unwrap();
    for line in data.lines() {
        let parts = line.split('\t').collect::<Vec<_>>();
        if parts.len() < 5 {
            continue;
        }

        let players = [parts[0], parts[1], parts[2], parts[3]];
        let winner = parts[4].to_string();
        let game = tournament.create_game(players);
        tournament.submit_game(game, winner)?;
    }

    Ok(())
}

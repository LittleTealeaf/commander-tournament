use crate::tournament::{ScoreConfig, Tournament, TournamentError};
use anyhow::Result;

mod tournament;

fn main() -> Result<()> {
    let mut t = Tournament::new();
    let config = ScoreConfig {
        starting_elo: 3000.0,
        ..*t.get_score_config()
    };

    ingest_tsv(&mut t)?;
    dbg!(&t);

    t.set_score_config(config)?;

    dbg!(&t);

    // let game = t.create_game(["Tifa", "Aurelia", "Rocksanne", "Anim"]);
    // dbg!(&game);
    // t.submit_game(game, "Tifa")?;
    // dbg!(t);

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

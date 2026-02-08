use commander_tournament::Tournament;

mod app;

fn main() -> anyhow::Result<()> {
    let mut tourn = Tournament::default();
    tourn.ingest_tsv_games(include_str!("../data.tsv"))?;


    dbg!(ron::to_string(&tourn)?);

    Ok(())
}

// fn main() -> iced::Result {
//     todo!()
// }

use commander_tournament::Tournament;

mod app;

fn main() -> anyhow::Result<()> {
    let mut tourn = Tournament::default();
    tourn.ingest_tsv_games(include_str!("../data.tsv"))?;

    let deserialized = ron::to_string(&tourn)?;
    let serialized = ron::from_str::<Tournament>(&deserialized)?;
    dbg!(deserialized);

    Ok(())
}

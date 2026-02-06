use crate::{
    tournament::{Tournament, TournamentError},
    ui::launch,
};
use anyhow::Result;

mod tournament;
mod ui;

fn main() -> iced::Result {
    launch()

    // let mut t = Tournament::new();
    // ingest_tsv(&mut t)?;
    //
    // dbg!(t.players().collect::<Vec<_>>());
    //
    // Ok(())
}


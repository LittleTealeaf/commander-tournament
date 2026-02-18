use commander_tournament::tournament::Tournament;

use crate::app::launch;

mod app;

fn main() -> anyhow::Result<()> {
    let tournament = Tournament::from_compat(include_str!("../old-game.ron"))?;
    println!("{}", ron::to_string(&tournament)?);

    // launch()?;
    Ok(())
}

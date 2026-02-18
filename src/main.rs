use commander_tournament::tourn::Tournament;
use ron::ser::PrettyConfig;

use crate::app::launch;

mod app;

fn main() -> anyhow::Result<()> {
    let tournament = Tournament::from_compat(include_str!("../old-game.ron"))?;
    println!("{}", ron::ser::to_string_pretty(&tournament, PrettyConfig::default())?);

    // launch()?;
    Ok(())
}

use crate::app::launch;

mod app;

fn main() -> anyhow::Result<()> {
    launch()?;
    Ok(())
}

use std::{env, fs, path::PathBuf};

use backwards_compat::BackwardsCompat;
use commander_tournament_core::{
    player::{color::ColorIdentity, info::PlayerInfo},
    tournament::Tournament,
};
use ron::ser::PrettyConfig;

fn main() -> anyhow::Result<()> {
    let path = file_path()?;
    let tourn = tourn()?;
    let ron_data = ron::ser::to_string_pretty(&tourn, PrettyConfig::new())?;
    fs::write(&path, ron_data)?;
    Ok(())
}

fn tourn() -> anyhow::Result<Tournament> {
    let mut tourn = Tournament::new();
    register_players(&mut tourn)?;
    Ok(tourn)
}

fn register_players(tourn: &mut Tournament) -> anyhow::Result<()> {
    let mut identities = ColorIdentity::IDENTITIES.iter().cycle();

    for precon in [true, false] {
        for archived in [true, false] {
            for moxfield_url in [
                Some("https://moxfield.com/decks/EoWfmyQpRkqeHHs_t52xIw"),
                Some("https://moxfield.com/decks/-6AhFOKbMkKDzXQAULhzGA/playtest"),
                None,
            ] {
                if let Some(identity) = identities.next() {
                    let mut info = PlayerInfo::new(format!("{identity}-{precon}-{archived}"))
                        .with_color_identity(*identity)
                        .with_is_precon(precon)
                        .with_is_archived(archived);

                    if let Some(url) = moxfield_url {
                        info = info.with_moxfield_id(url.to_owned());
                    }

                    let _ = tourn.register_player(info)?;
                }
            }
        }
    }

    Ok(())
}

fn file_path() -> anyhow::Result<PathBuf> {
    let mut args = env::args();
    let _ = args.next();

    let variant = if let Some(s) = args.next() {
        Some(s.parse::<u32>()?)
    } else {
        None
    };

    let version = Tournament::CURRENT_VERSION;

    let file_name = variant.map_or_else(
        || format!("compat-v{version}.ron"),
        |var| format!("compat-v{version}-{var}.ron"),
    );

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_path = manifest_dir
        .join("tests")
        .join("fixtures")
        .join("saves")
        .join(file_name);

    Ok(out_path)
}

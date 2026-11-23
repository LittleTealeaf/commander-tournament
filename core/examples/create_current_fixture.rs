use core::hash::{Hash, Hasher};
use std::{env, fs, hash::DefaultHasher, path::PathBuf};

use backwards_compat::BackwardsCompat;
use commander_tournament_core::{
    game::next_mode::NextPlayerMode,
    player::{color::ColorIdentity, info::PlayerInfo},
    tournament::Tournament,
};
use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
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
    register_players(&mut tourn, 35)?;
    sample_games(&mut tourn, 100)?;
    Ok(tourn)
}

fn register_players(tourn: &mut Tournament, count: usize) -> anyhow::Result<()> {
    let identities = ColorIdentity::IDENTITIES.iter().cycle().take(count);

    let options = [
        Some("https://moxfield.com/decks/EoWfmyQpRkqeHHs_t52xIw"),
        Some("https://moxfield.com/decks/-6AhFOKbMkKDzXQAULhzGA/playtest"),
        None,
    ]
    .into_iter()
    .flat_map(|moxfield| [true, false].into_iter().map(move |precon| (precon, moxfield)))
    .flat_map(|(precon, moxfield)| {
        [true, false]
            .into_iter()
            .map(move |archived| (precon, archived, moxfield))
    })
    .cycle();

    for (id, (identity, (precon, archived, moxfield))) in identities.zip(options).enumerate() {
        let mut info = PlayerInfo::new(format!("{id}-{identity}"))
            .with_color_identity(*identity)
            .with_is_precon(precon)
            .with_is_archived(archived);

        if let Some(url) = moxfield {
            info = info.with_moxfield_id(url.to_owned());
        }

        let _ = tourn.register_player(info)?;
    }
    Ok(())
}

fn sample_games(tourn: &mut Tournament, games: usize) -> anyhow::Result<()> {
    for _ in 0..games {
        let Some(player) = NextPlayerMode::Default.next_player(tourn) else {
            return Err(anyhow::anyhow!("Player could not be selected"));
        };

        let matchup = tourn.matchmaker().create_match(player)?;
        let ids = matchup.ids();
        let mut hasher = DefaultHasher::new();
        ids.hash(&mut hasher);
        let mut rng = StdRng::seed_from_u64(hasher.finish());

        let weights = matchup
            .players()
            .iter()
            .map(|player| (*player.expected(), player.id()))
            .collect::<Vec<_>>();

        let (_, winner_id) = weights.choose_weighted(&mut rng, |w| w.0)?;

        let record = matchup.record(*winner_id)?;
        tourn.record_game(record)?;
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

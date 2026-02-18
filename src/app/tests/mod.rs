pub mod utils;

use std::{io::Write, path::PathBuf};

use tempfile::{NamedTempFile, tempfile};

use crate::app::{App, message::Message, tests::utils::sample_tournament, traits::HandleMessage};

#[test]
fn load_tournament() {
    let mut app = App::default();
    let t = sample_tournament();
    let _ = app.update(Message::LoadTournament(t.clone().into()));
    assert_eq!(t, app.tournament);
}

#[test]
fn open_file() {
    let t = sample_tournament();
    let ron = ron::ser::to_string(&t).unwrap();
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(ron.as_bytes()).unwrap();
    let path = file.path().to_path_buf();
    file.close().unwrap();

    let mut app = App::default();
    let task = app.update(Message::OpenFile(path)).unwrap();
}

use std::path::{Path, PathBuf};

use anyhow::anyhow;
use edh_tourn::{Tournament, compat::v1::TournamentCompatV1};
use iced::{Task, futures::FutureExt};
use rfd::AsyncFileDialog;
use serde::{Deserialize, Serialize};

use crate::{App, logic::Message, traits::HandleMessage};

pub fn accepted_file_types() -> Vec<&'static str> {
    vec!["ron", "json", "toml"]
}

#[derive(Clone)]
pub enum FileMessage {
    LoadFromFile(PathBuf),
    SaveToFile(PathBuf),
    OpenFile,
    SaveAs,
    Save,
    New,
    SetOpenedFile(PathBuf),
    LoadTournamentFromFile(PathBuf, Box<Tournament>),
}

impl From<FileMessage> for Message {
    fn from(value: FileMessage) -> Self {
        Self::File(value)
    }
}

impl HandleMessage<FileMessage> for App {
    fn update(&mut self, msg: FileMessage) -> anyhow::Result<iced::Task<super::Message>> {
        match msg {
            FileMessage::LoadFromFile(path_buf) => Ok(Task::perform(
                load_file(path_buf.clone()),
                Message::handle_error_fn(move |t: Tournament| {
                    FileMessage::LoadTournamentFromFile(path_buf.clone(), t.into())
                }),
            )),
            FileMessage::SaveToFile(path) => {
                let extension =
                    get_extension(&path).ok_or_else(|| anyhow!("Invalid File Extension"))?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                Ok(Task::perform(
                    async_fs::write(path.clone(), serialized),
                    Message::handle_error_fn(move |()| FileMessage::SetOpenedFile(path.clone())),
                ))
            }
            FileMessage::OpenFile => Ok(Task::perform(
                AsyncFileDialog::new()
                    .add_filter("formats", &accepted_file_types())
                    .set_directory(".")
                    .set_title("Open Tournament")
                    .pick_file()
                    .then(async |res| res.map(|handle| handle.path().to_path_buf())),
                Message::handle_option_fn(FileMessage::LoadFromFile),
            )),
            FileMessage::SaveAs => Ok(Task::perform(
                AsyncFileDialog::new()
                    .add_filter("formats", &accepted_file_types())
                    .set_directory(".")
                    .set_title("Save Tournament")
                    .save_file()
                    .then(async |res| res.map(|handle| handle.path().to_path_buf())),
                Message::handle_option_fn(FileMessage::SaveToFile),
            )),
            FileMessage::Save => self.update(
                self.file
                    .clone()
                    .map_or(FileMessage::SaveAs, FileMessage::SaveToFile),
            ),
            FileMessage::SetOpenedFile(path_buf) => {
                self.file = Some(path_buf);
                Message::done()
            }
            FileMessage::LoadTournamentFromFile(path_buf, tournament) => {
                self.tournament = *tournament;
                self.file = Some(path_buf);
                Message::done()
            }
            FileMessage::New => {
                self.tournament = Tournament::default();
                self.file = None;
                Message::done()
            }
        }
    }
}

async fn load_file(path: PathBuf) -> anyhow::Result<Tournament> {
    let extension = get_extension(&path).ok_or_else(|| anyhow!("Invalid File Extension"))?;
    let data = async_fs::read_to_string(&path).await?;

    deserialize_by_extension(&data, extension).or_else(|error| {
        deserialize_by_extension::<TournamentCompatV1>(&data, extension)
            .and_then(|tourn| Ok(Tournament::try_from(tourn)?))
            .map_err(|_| error)
    })
}

fn get_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

fn deserialize_by_extension<'a, T>(data: &'a str, extension: &str) -> anyhow::Result<T>
where
    T: Deserialize<'a>,
{
    Ok(match extension {
        "ron" => ron::from_str(data)?,
        "json" => serde_json::from_str(data)?,
        "toml" => toml::from_str(data)?,
        ext => return Err(anyhow!("File type not supported: {ext}")),
    })
}

fn serialize_by_extension<T>(data: &T, extension: &str) -> anyhow::Result<String>
where
    T: Serialize,
{
    Ok(match extension {
        "ron" => ron::to_string(data)?,
        "json" => serde_json::to_string(data)?,
        "toml" => toml::to_string(data)?,
        ext => return Err(anyhow!("File type not supported: {ext}")),
    })
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::PathBuf};

    use edh_tourn::Tournament;
    use tempfile::NamedTempFile;

    use crate::{
        App,
        logic::{
            Message,
            file::{
                FileMessage, deserialize_by_extension, get_extension, load_file,
                serialize_by_extension,
            },
        },
        traits::HandleMessage,
    };

    #[test]
    fn new_sets_default_tournament() {
        let mut app = App::default();
        app.test_update(Message::LoadTournament(Tournament::sample_game().into()));
        let tournament = app.tournament.clone();
        app.test_update(FileMessage::New);
        let new_tournament = app.tournament.clone();
        assert_ne!(tournament, new_tournament);
    }

    #[test]
    fn new_clears_file() {
        let mut app = App::default();
        let temp_file = NamedTempFile::new().unwrap();
        app.file = Some(temp_file.path().to_path_buf());
        app.test_update(FileMessage::New);
        assert!(app.file.is_none());
    }

    #[tokio::test]
    async fn loads_v1_compat() {
        let compat_str = include_bytes!("../../../tests/v1-game.ron");
        let mut file = NamedTempFile::with_suffix(".ron").unwrap();
        file.write_all(compat_str).unwrap();
        load_file(file.path().to_path_buf()).await.unwrap();
    }

    #[test]
    fn load_file_sets_file() {
        let temp_file = NamedTempFile::new().unwrap();

        let mut app = App::default();
        app.test_update(FileMessage::LoadTournamentFromFile(
            temp_file.path().to_path_buf(),
            Tournament::sample_game().into(),
        ));
        assert_eq!(Some(temp_file.path().to_path_buf()), app.file);
    }

    #[test]
    fn gets_correct_extension() {
        let test_cases = vec![
            // (Path, Expected Extension)
            ("image.png", Some("png")),
            ("archive.tar.gz", Some("gz")),
            ("script.min.js", Some("js")),
            ("README", None),                 // No extension
            ("data..csv", Some("csv")),       // Double dot
            ("backup.2024.sql", Some("sql")), // Dot in filename stem
            ("folder.name/file", None),       // Dot in directory only
            ("long.extension_name", Some("extension_name")),
            ("space in name.pdf", Some("pdf")),
            ("archive.7z.001", Some("001")), // Numeric extension
            ("config.JSON", Some("JSON")),   // Case sensitivity check
        ];

        for (path_str, exp_ext) in test_cases {
            let path: PathBuf = path_str.into();
            assert_eq!(exp_ext, get_extension(&path), "Failed for {path_str}");
        }
    }

    #[test]
    fn error_deserialize_invalid_extension() {
        deserialize_by_extension::<String>("", "notanextension").unwrap_err();
    }

    #[test]
    fn error_serialize_invalid_extension() {
        serialize_by_extension(&Tournament::new(), "notanextension").unwrap_err();
    }

    macro_rules! test_extension {
        ($id: ident, $ext: expr, $serialize: expr, $deserialize: expr) => {
            mod $id {
                use std::io::Write;

                #[test]
                fn serialize() {
                    let tournament = edh_tourn::Tournament::sample_game();
                    let serialized_tournament =
                        crate::logic::file::serialize_by_extension(&tournament, $ext).unwrap();
                    let deserialized_tournament = ($deserialize)(&serialized_tournament);
                    assert_eq!(tournament, deserialized_tournament);
                }

                #[test]
                fn deserialize() {
                    let tournament = edh_tourn::Tournament::sample_game();
                    let serialized_tournament = ($serialize)(&tournament);
                    let deserialized_tournament =
                        crate::logic::file::deserialize_by_extension(&serialized_tournament, $ext)
                            .unwrap();
                    assert_eq!(tournament, deserialized_tournament);
                }

                #[tokio::test]
                async fn parse_from_file() {
                    let tournament = edh_tourn::Tournament::sample_game();
                    let serialized = ($serialize)(&tournament);
                    let mut file =
                        tempfile::NamedTempFile::with_suffix(format!(".{}", $ext)).unwrap();
                    file.write_all(serialized.as_bytes()).unwrap();
                    let path = file.path().to_path_buf();
                    let res = crate::logic::file::load_file(path).await.unwrap();
                    assert_eq!(tournament, res);
                }
            }
        };
    }

    test_extension!(ron_ext, "ron", |t| ron::to_string(t).unwrap(), |s| {
        ron::from_str(s).unwrap()
    });
    test_extension!(
        json_ext,
        "json",
        |t| serde_json::to_string(t).unwrap(),
        |s| serde_json::from_str(s).unwrap()
    );

    test_extension!(toml_ext, "toml", |t| toml::to_string(t).unwrap(), |s| {
        toml::from_str(s).unwrap()
    });
}

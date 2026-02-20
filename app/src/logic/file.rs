use std::path::{PathBuf};

use edh_tourn::Tournament;
use iced::Task;

use crate::{App, logic::Message, traits::HandleMessage};

#[derive(Clone)]
pub enum FileMessage {
    LoadJson(PathBuf),
    SaveToJson(PathBuf),
    LoadRon(PathBuf),
    SaveToRon(PathBuf),
}

impl From<FileMessage> for Message {
    fn from(value: FileMessage) -> Self {
        Self::File(value)
    }
}

impl HandleMessage<FileMessage> for App {
    fn update(&mut self, msg: FileMessage) -> anyhow::Result<iced::Task<super::Message>> {
        match msg {
            FileMessage::LoadJson(path_buf) => Ok(Task::perform(
                parse_json_file(path_buf),
                Message::handle_error_fn(|tourn: Tournament| Message::LoadTournament(tourn.into())),
            )),
            FileMessage::SaveToJson(path_buf) => Ok(Task::perform(
                async_fs::write(path_buf, serde_json::to_string(&self.tournament)?),
                Message::handle_error_fn(Message::from),
            )),
            FileMessage::LoadRon(path_buf) => Ok(Task::perform(
                parse_ron_file(path_buf),
                Message::handle_error_fn(|tourn: Tournament| Message::LoadTournament(tourn.into())),
            )),
            FileMessage::SaveToRon(path_buf) => Ok(Task::perform(
                async_fs::write(path_buf, ron::to_string(&self.tournament)?),
                Message::handle_error_fn(Message::from),
            )),
        }
    }
}

pub async fn parse_ron_file(path: PathBuf) -> anyhow::Result<Tournament> {
    let data = async_fs::read_to_string(path).await?;
    Ok(ron::de::from_str(&data)?)
}

pub async fn parse_json_file(path: PathBuf) -> anyhow::Result<Tournament> {
    let data = async_fs::read_to_string(path).await?;
    Ok(serde_json::from_str(&data)?)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use edh_tourn::Tournament;
    use tempfile::NamedTempFile;

    use crate::logic::file::{parse_json_file, parse_ron_file};

    #[tokio::test]
    async fn parses_from_ron_file() {
        let t = Tournament::sample_game();
        let ron = ron::ser::to_string(&t).unwrap();
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(ron.as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let res = parse_ron_file(path).await.unwrap();
        assert_eq!(res, t);
    }

    #[tokio::test]
    async fn parses_from_json_file() {
        let t = Tournament::sample_game();
        let json_data = serde_json::to_string(&t).unwrap();
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let res = parse_json_file(path).await.unwrap();
        assert_eq!(res, t);
    }
}

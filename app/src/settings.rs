use anyhow::anyhow;
use iced::Task;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::{
    services::system::{load_from_file_async, save_file_async},
    traits::{Component, ComponentUpdate, Effect, HandleMessage},
};

const QUALIFIER: &str = "io.github.littletealeaf";
const ORGANIZATION: &str = "LittleTealeaf";
const APPLICATION: &str = "commander-tournament";

fn get_project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
}

fn get_config_path() -> Option<PathBuf> {
    let project_dir = get_project_dir()?;
    let path = project_dir.config_dir().join("config.ron");
    Some(path)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AppSettings {
    #[serde(skip)]
    save_path: PathBuf,
    #[serde(skip)]
    is_saving: bool,
    last_opened: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Save,
    IsSaved,
    SetOpenedFile(PathBuf),
    ClearOpenedFile,
    Error(String),
    Nothing,
}

impl AppSettings {
    #[allow(clippy::assertions_on_constants)]
    pub async fn load() -> anyhow::Result<Self> {
        debug_assert!(!cfg!(test), "Do not run AppSettings::load() in tests");
        let path = get_config_path().ok_or_else(|| anyhow!("Could not file app config path"))?;
        let config = Self::load_from_path_or_default(path).await;
        Ok(config)
    }

    pub async fn load_from_path(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            save_path: path.clone(),
            ..load_from_file_async(path).await?
        })
    }

    pub async fn load_from_path_or_default(path: PathBuf) -> Self {
        Self::load_from_path(path.clone()).await.unwrap_or(Self {
            save_path: path,
            is_saving: false,
            last_opened: None,
        })
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        save_file_async(&self, self.save_path.clone()).await
    }

    #[must_use]
    pub const fn last_opened(&self) -> &Option<PathBuf> {
        &self.last_opened
    }

    pub fn clear_last_opened(&mut self) {
        self.last_opened = None;
    }

    pub fn set_last_opened(&mut self, last_opened: PathBuf) {
        self.last_opened = Some(last_opened);
    }
}

impl Component for AppSettings {
    type Message = Message;
    type Context<'a> = ();
    type OutMessage = String;
}

impl ComponentUpdate for AppSettings {
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::Save => {
                self.is_saving = true;
                let settings = self.clone();
                let future = async move {
                    match settings.save().await {
                        Ok(()) => Message::IsSaved,
                        Err(error) => Message::Error(error.to_string()),
                    }
                };
                let task = Task::future(future);
                Effect::task(task)
            }
            Message::IsSaved => {
                self.is_saving = false;
                Effect::ok()
            }
            Message::SetOpenedFile(path_buf) => {
                self.set_last_opened(path_buf);
                self.handle_message(Message::Save, ())
            }
            Message::ClearOpenedFile => {
                self.clear_last_opened();
                self.handle_message(Message::Save, ())
            }
            Message::Nothing => Effect::ok(),
            Message::Error(error) => Effect::out(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_dir_exists() {
        get_project_dir().unwrap();
    }

    #[test]
    fn config_path_exists() {
        get_config_path().unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Do not run AppSettings::load() in tests")]
    async fn load_in_tests_panics() {
        AppSettings::load().await.unwrap();
    }
}

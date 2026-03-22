use iced::Task;
use std::path::PathBuf;

use crate::{
    core::message::Message,
    services::system::{load_from_file_async, project_dir, save_file_async},
    traits::{Component, ComponentUpdate, Effect, HandleMessage},
};

const QUALIFIER: &str = "io.github.littletealeaf";
const ORGANIZATION: &str = "LittleTealeaf";
const APPLICATION: &str = "commander-tournament";

fn get_state_path() -> Option<PathBuf> {
    let project = project_dir()?;
    Some(
        project
            .state_dir()
            .unwrap_or_else(|| project.data_dir())
            .to_path_buf(),
    )
}

#[cfg(feature = "dev")]
#[must_use]
pub fn debug_config_path() -> Option<PathBuf> {
    get_state_path()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AppState {
    #[serde(skip)]
    save_path: PathBuf,
    #[serde(skip)]
    is_saving: bool,
    #[serde(skip)]
    is_modified: bool,
    last_opened: Option<PathBuf>,
}
impl AppState {
    pub async fn load() -> anyhow::Result<Self> {
        let path = {
            #[cfg(feature = "dev")]
            {
                tempfile::NamedTempFile::with_suffix(".ron")?
                    .path()
                    .to_path_buf()
            }
            #[cfg(not(feature = "dev"))]
            {
                get_state_path().ok_or_else(|| anyhow::anyhow!("Could not file app config path"))?
            }
        };
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
            is_modified: false,
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
        self.is_modified = true;
    }

    pub fn set_last_opened(&mut self, last_opened: PathBuf) {
        self.last_opened = Some(last_opened);
        self.is_modified = true;
    }

    #[cfg(feature = "dev")]
    #[must_use]
    pub const fn settings_loc(&self) -> &PathBuf {
        &self.save_path
    }
}

#[derive(Debug, Clone)]
pub enum AppStateMsg {
    Save,
    IsSaved,
    SetOpenedFile(PathBuf),
    ClearOpenedFile,
    Error(String),
    Nothing,
}

impl Component for AppState {
    type Message = AppStateMsg;
    type OutMessage = ();
}

impl ComponentUpdate for AppState {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            AppStateMsg::Save => {
                if self.is_saving {
                    self.is_modified = true;
                    return Effect::done();
                }

                self.is_saving = true;
                self.is_modified = false;
                let settings = self.clone();
                let future = async move {
                    match settings.save().await {
                        Ok(()) => AppStateMsg::IsSaved,
                        Err(error) => AppStateMsg::Error(error.to_string()),
                    }
                };
                let task = Task::future(future);
                Effect::task(task)
            }
            AppStateMsg::IsSaved => {
                self.is_saving = false;
                if self.is_modified {
                    self.handle_message(AppStateMsg::Save, ())
                } else {
                    Effect::done()
                }
            }
            AppStateMsg::SetOpenedFile(path_buf) => {
                self.set_last_opened(path_buf);
                self.handle_message(AppStateMsg::Save, ())
            }
            AppStateMsg::ClearOpenedFile => {
                self.clear_last_opened();
                self.handle_message(AppStateMsg::Save, ())
            }
            AppStateMsg::Nothing => Effect::done(),
            AppStateMsg::Error(error) => {
                self.is_saving = false;
                self.is_modified = true;
                Effect::global(Message::Error(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_exists() {
        get_state_path().unwrap();
    }

    #[tokio::test]
    async fn testing_uses_non_system_config_path() {
        let system_path = get_state_path().unwrap();
        let settings = AppState::load().await.unwrap();
        assert_ne!(system_path, settings.save_path);
    }
}

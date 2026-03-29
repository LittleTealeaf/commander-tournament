use iced::Task;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    effect::Effect,
    services::system::{load_from_file_async, project_dir, save_file_async},
    traits::{Component, ComponentUpdate},
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
            .to_path_buf()
            .join("app-state.json"),
    )
}

#[cfg(feature = "dev")]
#[must_use]
pub fn debug_config_path() -> Option<PathBuf> {
    get_state_path()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    save_path: PathBuf,
    is_saving: bool,
    is_modified: bool,
    data: AppStatePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AppStatePayload {
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
            is_saving: false,
            is_modified: false,
            data: load_from_file_async(path).await?,
        })
    }

    pub async fn load_from_path_or_default(path: PathBuf) -> Self {
        Self::load_from_path(path.clone()).await.unwrap_or(Self {
            save_path: path,
            is_saving: false,
            is_modified: false,
            data: AppStatePayload { last_opened: None },
        })
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        save_file_async(&self.data, self.save_path.clone()).await
    }

    #[must_use]
    pub const fn last_opened(&self) -> &Option<PathBuf> {
        &self.data.last_opened
    }

    pub fn clear_last_opened(&mut self) {
        self.data.last_opened = None;
        self.is_modified = true;
    }

    pub fn set_last_opened(&mut self, last_opened: PathBuf) {
        self.data.last_opened = Some(last_opened);
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
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
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
                Effect::Task(task).ok()
            }
            AppStateMsg::IsSaved => {
                self.is_saving = false;
                if self.is_modified {
                    Effect::Msg(AppStateMsg::Save).ok()
                } else {
                    Effect::done()
                }
            }
            AppStateMsg::SetOpenedFile(path_buf) => {
                self.set_last_opened(path_buf);
                Effect::Msg(AppStateMsg::Save).ok()
            }
            AppStateMsg::ClearOpenedFile => {
                self.clear_last_opened();
                Effect::Msg(AppStateMsg::Save).ok()
            }
            AppStateMsg::Nothing => Effect::done(),
            AppStateMsg::Error(error) => {
                self.is_saving = false;
                self.is_modified = true;
                Err(anyhow::anyhow!("{error:#}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn testing_uses_non_system_config_path() {
        let system_path = get_state_path().unwrap();
        let settings = AppState::load().await.unwrap();
        assert_ne!(system_path, settings.save_path);
    }
}

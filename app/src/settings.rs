use anyhow::anyhow;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::services::system::load_from_file_async;

const QUALIFIER: &str = "io.github.littletealeaf";
const ORGANIZATION: &str = "LittleTealeaf";
const APPLICATION: &str = "commander-tournament";

fn get_config_path() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
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
    OnSave,
}

impl AppSettings {
    pub async fn load() -> anyhow::Result<Self> {
        let project_dir =
            get_config_path().ok_or_else(|| anyhow!("Could not file app config path"))?;
        let path = project_dir.config_dir().join("config.ron");
        let config = Self::load_from_path_or_default(path).await;
        Ok(config)
    }

    pub async fn load_from_path(path: PathBuf) -> anyhow::Result<Self> {
        load_from_file_async(path).await
    }

    pub async fn load_from_path_or_default(path: PathBuf) -> Self {
        Self::load_from_path(path.clone()).await.unwrap_or(Self {
            save_path: path,
            is_saving: false,
            last_opened: None,
        })
    }

    #[must_use]
    pub const fn last_opened(&self) -> &Option<PathBuf> {
        &self.last_opened
    }
}

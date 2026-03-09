use anyhow::anyhow;
use std::{
    fs::{self, File},
    path::PathBuf,
};

use directories::ProjectDirs;

const QUALIFIER: &str = "";
const ORGANIZATION: &str = "LittleTealeaf";
const APPLICATION: &str = "commander-tournament";

fn get_config_path() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    #[serde(skip)]
    save_path: PathBuf,
    last_opened: Option<PathBuf>,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let project_dir =
            get_config_path().ok_or_else(|| anyhow!("Could not find app config path"))?;
        let path = project_dir.config_dir().join("config.ron");
        let config = Self::load_from_path_or_default(path);
        Ok(config)
    }

    #[must_use]
    fn load_from_path_or_default(path: PathBuf) -> Self {
        if let Ok(config) = Self::load_from_path(path.clone()) {
            return config;
        }
        Self {
            save_path: path,
            last_opened: None,
        }
    }

    fn load_from_path(path: PathBuf) -> anyhow::Result<Self> {
        let mut config: Self = ron::de::from_reader(File::open(path.clone())?)?;
        config.save_path = path;
        Ok(config)
    }

    fn save(&self) -> anyhow::Result<()> {
        let path = &self.save_path;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data_str = ron::ser::to_string(&self)?;

        fs::write(path, data_str)?;

        Ok(())
    }

    #[must_use]
    pub const fn last_opened(&self) -> Option<&PathBuf> {
        self.last_opened.as_ref()
    }

    pub fn set_last_opened(&mut self, last_opened: PathBuf) -> anyhow::Result<()> {
        self.last_opened = Some(last_opened);
        self.save()
    }

    pub fn clear_last_opened(&mut self) -> anyhow::Result<()> {
        self.last_opened = None;
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn config_path_exists() {
        get_config_path().unwrap();
    }

    #[test]
    fn saves_to_file() -> anyhow::Result<()> {
        let file = NamedTempFile::new()?;
        let path = file.path().to_path_buf();
        let config = AppConfig::load_from_path_or_default(path.clone());
        config.save()?;

        // Tests that it can be loaded
        AppConfig::load_from_path(path)?;

        Ok(())
    }

    #[test]
    fn set_last_opened_writes_config() -> anyhow::Result<()> {
        let file = NamedTempFile::new()?;
        let path = file.path().to_path_buf();

        let mut config = AppConfig::load_from_path_or_default(path.clone());

        let new_file = NamedTempFile::new()?;
        let new_path = new_file.path().to_path_buf();
        config.set_last_opened(new_path.clone())?;
        assert_eq!(&new_path, config.last_opened().unwrap());

        let new_config = AppConfig::load_from_path(path)?;
        assert_eq!(&new_path, new_config.last_opened().unwrap());
        Ok(())
    }
}

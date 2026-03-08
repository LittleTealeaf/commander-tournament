use std::{fs, path::PathBuf};

use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// System‑wide configuration persisted between runs.
///
/// Right now the only piece of information we keep is the path to the
/// last‑opened tournament file.  The design is intentionally open so new
/// fields can be added later without touching the caller sites.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemConfig {
    /// full path to the most‑recently opened tournament file
    pub last_opened: Option<PathBuf>,
}

impl SystemConfig {
    /// Compute the location of the configuration file on the host OS.
    ///
    /// Returns an error if the directory cannot be determined or cannot be
    /// created.  `%APPDATA%`/`XDG_CONFIG_HOME` etc. are handled by
    /// [`directories::ProjectDirs`].
    pub fn config_path() -> Result<PathBuf> {
        let proj = ProjectDirs::from("com", "LittleTealeaf", "commander-tournament")
            .ok_or_else(|| anyhow!("unable to determine project directories"))?;
        let cfg_dir = proj.config_dir();
        fs::create_dir_all(cfg_dir)
            .with_context(|| format!("creating config directory {}", cfg_dir.display()))?;
        Ok(cfg_dir.join("config.ron"))
    }

    /// Load the configuration from disk, returning `Default` if the file does
    /// not exist.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let file = fs::File::open(&path)
            .with_context(|| format!("opening config file {}", path.display()))?;
        let cfg: Self = ron::de::from_reader(file)
            .with_context(|| format!("deserializing config from {}", path.display()))?;
        Ok(cfg)
    }

    /// Persist the configuration to disk.  The directory will be created if
    /// necessary.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let serialized = ron::to_string(self).context("serializing system configuration")?;
        fs::write(&path, serialized)
            .with_context(|| format!("writing config file {}", path.display()))?;
        Ok(())
    }

    /// Convenience helper used by the UI layer when the active file changes.
    pub fn update_last_opened(path: Option<PathBuf>) -> Result<()> {
        let mut cfg = Self::load()?;
        cfg.last_opened = path;
        cfg.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_returns_expected_file() {
        let path = SystemConfig::config_path().unwrap();
        assert_eq!(path.file_name().unwrap(), "config.ron");
        assert!(path.to_string_lossy().contains("commander-tournament"));
    }
}

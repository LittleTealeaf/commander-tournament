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
    use std::env;
    use tempfile::tempdir;

    /// run a closure with a temporary `XDG_CONFIG_HOME` set; restore the old
    /// value afterwards.
    fn with_temp_xdg<T>(f: impl FnOnce() -> T) -> T {
        let tmp = tempdir().unwrap();
        let old = env::var_os("XDG_CONFIG_HOME");
        unsafe {
            env::set_var("XDG_CONFIG_HOME", tmp.path());
        }
        let res = f();
        if let Some(old) = old {
            unsafe {
                env::set_var("XDG_CONFIG_HOME", old);
            }
        } else {
            unsafe {
                env::remove_var("XDG_CONFIG_HOME");
            }
        }
        res
    }

    use serial_test::serial;

    #[serial]
    #[test]
    fn config_path_points_at_config_dir() {
        with_temp_xdg(|| {
            let path = SystemConfig::config_path().unwrap();
            assert!(path.ends_with("commander-tournament/config.ron"));
            assert!(path.starts_with(env::var_os("XDG_CONFIG_HOME").unwrap()));
        });
    }

    #[serial]
    #[test]
    fn load_when_missing_returns_default() {
        with_temp_xdg(|| {
            let cfg = SystemConfig::load().unwrap();
            assert_eq!(cfg, SystemConfig::default());
        });
    }

    #[serial]
    #[test]
    fn save_and_load_roundtrip() {
        with_temp_xdg(|| {
            let cfg = SystemConfig {
                last_opened: Some(PathBuf::from("/some/file")),
            };
            cfg.save().unwrap();
            let loaded = SystemConfig::load().unwrap();
            assert_eq!(cfg, loaded);
        });
    }

    #[serial]
    #[test]
    fn update_last_opened_sets_value_and_persists() {
        with_temp_xdg(|| {
            // start with no config file
            assert_eq!(SystemConfig::load().unwrap().last_opened, None);
            SystemConfig::update_last_opened(Some(PathBuf::from("foo"))).unwrap();
            assert_eq!(
                SystemConfig::load().unwrap().last_opened,
                Some(PathBuf::from("foo"))
            );
            SystemConfig::update_last_opened(None).unwrap();
            assert_eq!(SystemConfig::load().unwrap().last_opened, None);
        });
    }
}

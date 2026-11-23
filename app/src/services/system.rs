use anyhow::anyhow;
use directories::ProjectDirs;
use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

const QUALIFIER: &str = "io.github.littletealeaf";
const ORGANIZATION: &str = "LittleTealeaf";
const APPLICATION: &str = "commander-tournament";

#[must_use]
pub fn project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
}

#[must_use]
pub fn accepted_file_types() -> Vec<&'static str> {
    let mut types = vec!["ron"];
    #[cfg(feature = "serde_json")]
    types.push("json");
    #[cfg(feature = "serde_toml")]
    types.push("toml");
    types
}

#[must_use]
pub fn get_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

pub fn require_extension(path: &Path) -> anyhow::Result<&str> {
    get_extension(path).ok_or_else(|| {
        anyhow!(
            "Invalid File Extension for path: {}",
            path.to_path_buf().to_string_lossy()
        )
    })
}

pub fn deserialize_by_extension<'a, T>(data: &'a str, extension: &str) -> anyhow::Result<T>
where
    T: Deserialize<'a>,
{
    Ok(match extension {
        "ron" => ron::from_str(data)?,
        #[cfg(feature = "serde_json")]
        "json" => serde_json::from_str(data)?,
        #[cfg(feature = "serde_toml")]
        "toml" => toml::from_str(data)?,
        ext => {
            return Err(anyhow!("File type not supported: {ext}"));
        }
    })
}

pub fn serialize_by_extension<T>(data: &T, extension: &str) -> anyhow::Result<String>
where
    T: Serialize,
{
    Ok(match extension {
        "ron" => ron::to_string(data)?,
        #[cfg(feature = "serde_json")]
        "json" => serde_json::to_string(data)?,
        #[cfg(feature = "serde_toml")]
        "toml" => toml::to_string(data)?,
        ext => {
            return Err(anyhow!("File type not supported: {ext}"));
        }
    })
}

pub fn load_from_file_sync<T>(path: &PathBuf) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let extension = require_extension(path)?;
    let data = fs::read_to_string(path)?;
    deserialize_by_extension(&data, extension)
}

pub async fn load_from_file_async<T>(path: PathBuf) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let extension = require_extension(&path)?;
    let data = async_fs::read_to_string(&path).await?;
    deserialize_by_extension(&data, extension)
}

pub fn save_file_sync<T>(data: &T, path: PathBuf) -> anyhow::Result<()>
where
    T: Serialize,
{
    let extension = require_extension(&path)?;
    let serialized = serialize_by_extension(data, extension)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serialized.as_bytes())?;
    Ok(())
}

pub async fn save_file_async<T>(data: &T, path: PathBuf) -> anyhow::Result<()>
where
    T: Serialize + Sync,
{
    let extension = require_extension(&path)?;
    let serialized = serialize_by_extension(data, extension)?;

    if let Some(parent) = path.parent() {
        async_fs::create_dir_all(parent).await?;
    }

    async_fs::write(path, serialized.as_bytes()).await?;
    Ok(())
}

pub async fn open_link(link: String) -> anyhow::Result<()> {
    Ok(tokio::task::spawn_blocking(move || opener::open_browser(link)).await??)
}

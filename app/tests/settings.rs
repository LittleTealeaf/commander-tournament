use tempfile::NamedTempFile;

// Adjust `commander_tournament` to your actual crate name if different
use app::core::settings::{AppState, debug_config_path};

fn temp_file() -> NamedTempFile {
    NamedTempFile::with_suffix(".ron").unwrap()
}

#[tokio::test]
async fn settings_loc_uses_temp_path() {
    let settings = AppState::load().await.unwrap();
    let settings_path = settings.settings_loc().clone();
    let system_path = debug_config_path().unwrap();
    assert_ne!(
        settings_path, system_path,
        "Settings is using system path in dev flag"
    );
}

#[tokio::test]
async fn loading_from_empty() {
    let path = temp_file().path().to_path_buf();
    AppState::load_from_path(path).await.unwrap_err();
}

#[tokio::test]
async fn saves_to_path() {
    let path = temp_file().path().to_path_buf();
    let settings = AppState::load_from_path_or_default(path.clone()).await;
    settings.save().await.unwrap();

    let data = async_fs::read_to_string(path).await.unwrap();
    assert!(!data.is_empty());
}

#[tokio::test]
async fn loading_from_saved() {
    let file = temp_file();
    let path = file.path().to_path_buf();
    let settings = AppState::load_from_path_or_default(path.clone()).await;
    settings.save().await.unwrap();

    AppState::load_from_path(path).await.unwrap();
}

#[tokio::test]
async fn set_and_clear_last_updated() {
    let mut settings = AppState::load().await.unwrap();
    assert!(
        settings.last_opened().is_none(),
        "Expected new settings object to have no last_opened file"
    );

    let new_file = temp_file();
    let new_path = new_file.path().to_path_buf();
    settings.set_last_opened(new_path.clone());
    assert_eq!(
        new_path,
        settings.last_opened().clone().unwrap(),
        "Expected setting last updated to return last updated"
    );

    settings.clear_last_opened();
    assert!(
        settings.last_opened().is_none(),
        "Clearing last updated did not clear value"
    );
}

#[tokio::test]
async fn last_updated_persists_through_saves() {
    let path = temp_file().path().to_path_buf();

    let tourn_test = temp_file().path().to_path_buf();

    let mut settings = AppState::load_from_path_or_default(path.clone()).await;
    settings.set_last_opened(tourn_test.clone());
    settings.save().await.unwrap();

    let new_settings = AppState::load_from_path_or_default(path.clone()).await;
    assert_eq!(tourn_test, new_settings.last_opened().clone().unwrap());
}

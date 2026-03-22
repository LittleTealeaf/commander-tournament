use tempfile::NamedTempFile;

// Adjust `commander_tournament` to your actual crate name if different
use app::settings::AppSettings;

fn temp_file() -> NamedTempFile {
    NamedTempFile::with_suffix(".ron").unwrap()
}

#[tokio::test]
async fn loading_from_empty() {
    let path = temp_file().path().to_path_buf();
    AppSettings::load_from_path(path).await.unwrap_err();
}

#[tokio::test]
async fn saves_to_path() {
    let path = temp_file().path().to_path_buf();
    let settings = AppSettings::load_from_path_or_default(path.clone()).await;
    settings.save().await.unwrap();

    let data = async_fs::read_to_string(path).await.unwrap();
    assert!(!data.is_empty());
}

#[tokio::test]
async fn loading_from_saved() {
    let file = temp_file();
    let path = file.path().to_path_buf();
    let settings = AppSettings::load_from_path_or_default(path.clone()).await;
    settings.save().await.unwrap();

    AppSettings::load_from_path(path).await.unwrap();
}

#[tokio::test]
async fn set_and_clear_last_updated() {
    let file = temp_file();
    let path = file.path().to_path_buf();
    let mut settings = AppSettings::load_from_path_or_default(path).await;
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

    let mut settings = AppSettings::load_from_path_or_default(path.clone()).await;
    settings.set_last_opened(tourn_test.clone());
    settings.save().await.unwrap();

    let new_settings = AppSettings::load_from_path_or_default(path.clone()).await;
    assert_eq!(tourn_test, new_settings.last_opened().clone().unwrap());
}

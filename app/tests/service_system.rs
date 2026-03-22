use app::services::system::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tempfile::tempdir;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestData {
    name: String,
    value: i32,
}

#[test]
fn test_extensions() {
    let valid_path = Path::new("config.json");
    assert_eq!(get_extension(valid_path), Some("json"));
    assert_eq!(require_extension(valid_path).unwrap(), "json");

    let invalid_path = Path::new("config");
    assert_eq!(get_extension(invalid_path), None);
    require_extension(invalid_path).unwrap_err();
}

#[test]
fn test_serialize_deserialize_supported_formats() {
    let data = TestData {
        name: "test_data".to_owned(),
        value: 42,
    };

    for ext in ["json", "toml", "ron"] {
        let serialized = serialize_by_extension(&data, ext).unwrap();
        let deserialized: TestData = deserialize_by_extension(&serialized, ext).unwrap();
        assert_eq!(data, deserialized, "Failed for extension: {ext}");
    }
}

#[test]
fn test_unsupported_format() {
    let data = TestData {
        name: "fail".to_owned(),
        value: 0,
    };

    serialize_by_extension(&data, "xml").unwrap_err();
    deserialize_by_extension::<TestData>("<xml></xml>", "xml").unwrap_err();
}

#[test]
fn test_sync_file_io() {
    let dir = tempdir().unwrap();
    let data = TestData {
        name: "sync_test".to_owned(),
        value: 100,
    };

    for ext in ["json", "toml", "ron"] {
        let path = dir.path().join(format!("test.{ext}"));

        // save_file_sync takes PathBuf
        save_file_sync(&data, path.clone()).unwrap();

        // load_from_file_sync takes &PathBuf
        let loaded: TestData = load_from_file_sync(&path).unwrap();

        assert_eq!(data, loaded, "Sync IO failed for extension: {ext}");
    }
}

#[tokio::test]
async fn test_async_file_io() {
    let dir = tempdir().unwrap();
    let data = TestData {
        name: "async_test".to_owned(),
        value: 200,
    };

    for ext in ["json", "toml", "ron"] {
        let path = dir.path().join(format!("test.{ext}"));

        // save_file_async takes PathBuf
        save_file_async(&data, path.clone()).await.unwrap();

        // load_from_file_async takes PathBuf
        let loaded: TestData = load_from_file_async(path).await.unwrap();

        assert_eq!(data, loaded, "Async IO failed for extension: {ext}");
    }
}

use std::path::PathBuf;

use commander_tournament::tourn::Tournament;

pub async fn parse_tournament_file(path: PathBuf) -> anyhow::Result<Tournament> {
    let data = async_fs::read_to_string(path).await?;
    Ok(ron::de::from_str(&data)?)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::app::{message::filesystem::parse_tournament_file, tests::utils::sample_tournament};

    #[tokio::test]
    async fn parses_from_tournament_file() {
        let t = sample_tournament();
        let ron = ron::ser::to_string(&t).unwrap();
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(ron.as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let res = parse_tournament_file(path).await.unwrap();
        assert_eq!(res, t);
    }
}

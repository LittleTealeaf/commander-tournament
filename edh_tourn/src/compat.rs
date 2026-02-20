use crate::{Tournament, compat::v1::TournamentCompatV1};

pub mod v1;

impl Tournament {
    pub fn from_compat(data: &str) -> anyhow::Result<Self> {
        let v1: TournamentCompatV1 = ron::from_str(data)?;
        Ok(Self::try_from(v1)?)
    }

    pub fn from_compat_bytes(data: &[u8]) -> anyhow::Result<Self> {
        let string = String::from_utf8(data.to_vec())?;
        Self::from_compat(&string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_compat() -> anyhow::Result<()> {
        let string = include_str!("../../tests/v1-game.ron");
        let _ = Tournament::from_compat(string)?;
        Ok(())
    }

    #[test]
    fn test_v1_compat_bytes() -> anyhow::Result<()> {
        let bytes = include_bytes!("../../tests/v1-game.ron");
        let _ = Tournament::from_compat_bytes(bytes)?;
        Ok(())
    }
}

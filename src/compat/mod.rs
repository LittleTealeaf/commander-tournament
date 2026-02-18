use crate::{compat::v1::TournamentCompatV1, tournament::Tournament};

pub mod v1;

impl Tournament {
    pub fn from_compat(data: &str) -> anyhow::Result<Self> {
        let v1: TournamentCompatV1 = ron::from_str(data)?;
        Ok(Self::try_from(v1)?)
    }
}

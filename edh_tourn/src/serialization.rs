pub mod v1;
pub mod v2;

use serde::Deserialize;

use crate::{
    Tournament,
    error::TournamentError,
    serialization::{v1::V1SerializedTournament, v2::V2SerializedTournament},
};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SerializedTournament {
    V1(V1SerializedTournament),
    V2(V2SerializedTournament),
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;

    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        match value {
            SerializedTournament::V2(v2) => v2.try_into(), // Uses V2 logic
            SerializedTournament::V1(v1) => v1.try_into(), // Uses V1 migration logic
        }
    }
}

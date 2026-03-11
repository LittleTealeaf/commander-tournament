pub mod v1;

use serde::Deserialize;

use crate::serialization::v1::V1SerializedTournament;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SerializedTournament {
    V1(V1SerializedTournament),
    V2,
}


use core::hash::BuildHasher;
use core::num::ParseIntError;
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::game::{entry::GameEntry, record::GameRecord};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DeserializableMap<T> {
    Integer(HashMap<u32, T>),
    String(HashMap<String, T>),
}

impl<T> DeserializableMap<T>
where
    T: for<'a> Deserialize<'a>,
{
    pub fn deserialize_to_map<'de, D>(deserializer: D) -> Result<HashMap<u32, T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = Self::deserialize(deserializer)?;
        variant.try_into().map_err(serde::de::Error::custom)
    }
}

#[allow(clippy::implicit_hasher)]
impl<T> TryFrom<DeserializableMap<T>> for HashMap<u32, T> {
    type Error = ParseIntError;
    fn try_from(value: DeserializableMap<T>) -> Result<Self, Self::Error> {
        match value {
            DeserializableMap::<T>::Integer(map) => Ok(map),
            DeserializableMap::<T>::String(map) => map
                .into_iter()
                .map(|(key, val)| key.parse().map(|id| (id, val)))
                .collect(),
        }
    }
}

/// For use with serde's ``serialize_with`` attribute
pub fn ordered_map<S, K, V, HS>(value: &HashMap<K, V, HS>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    HS: BuildHasher,
    V: Serialize,
    K: Ord + Serialize,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

pub fn convert_games<S>(items: &[GameRecord], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let values = items
        .iter()
        .flat_map(|record| GameEntry::new(record.ids(), record.winner()))
        .collect::<Vec<_>>();
    values.serialize(serializer)
}

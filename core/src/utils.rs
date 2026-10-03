use core::hash::BuildHasher;
use core::num::ParseIntError;
use std::{
    collections::{BTreeMap, HashMap},
    hash::RandomState,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) trait IntoCopiedIter<K, V>
where
    K: Copy,
    V: Copy,
{
    fn iter_copied(&self) -> impl Iterator<Item = (K, V)>;
}

impl<K, V> IntoCopiedIter<K, V> for HashMap<K, V>
where
    K: Copy,
    V: Copy,
{
    fn iter_copied(&self) -> impl Iterator<Item = (K, V)> {
        self.iter().map(|(&k, &v)| (k, v))
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum DeserializableMap<T> {
    Integer(HashMap<u32, T>),
    String(HashMap<String, T>),
}

impl<T> DeserializableMap<T>
where
    T: for<'a> Deserialize<'a>,
{
    pub(crate) fn deserialize_to_map<'de, D>(deserializer: D) -> Result<HashMap<u32, T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = Self::deserialize(deserializer)?;
        variant.try_into().map_err(serde::de::Error::custom)
    }
}

impl<T> TryFrom<DeserializableMap<T>> for HashMap<u32, T, RandomState> {
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

/// For use with serde's `serialize_with` attribute
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::indexing_slicing, reason = "tests")]
    fn is_copied() {
        let map = HashMap::from([(1, 2), (3, 4), (5, 6)]);
        let values = map.iter_copied();
        for (key, value) in values {
            assert_eq!(&map[&key], &value);
        }
    }
}

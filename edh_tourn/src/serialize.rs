use core::hash::BuildHasher;
use std::collections::{BTreeMap, HashMap};

use serde::{Serialize, Serializer};

pub mod compat;
pub mod deserialize;

use crate::game::{entry::GameEntry, record::GameRecord};

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

use core::hash::BuildHasher;
use std::collections::{BTreeMap, HashMap};

use serde::{Serialize, Serializer};

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

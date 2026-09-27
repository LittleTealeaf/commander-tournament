use std::collections::HashMap;

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

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

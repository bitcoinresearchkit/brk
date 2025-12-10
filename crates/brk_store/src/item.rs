use std::cmp::Ordering;

pub enum Item<K, V> {
    Value { key: K, value: V },
    Tomb(K),
}

impl<K, V> Item<K, V> {
    #[inline]
    fn key(&self) -> &K {
        match self {
            Self::Value { key, .. } | Self::Tomb(key) => key,
        }
    }
}

impl<K: Ord, V> Ord for Item<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(other.key())
    }
}

impl<K: Ord, V> PartialOrd for Item<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Eq, V> PartialEq for Item<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<K: Eq, V> Eq for Item<K, V> {}

use crate::{
    CachedRangeMapCursor, RangeMapCursor,
    cached_cursor::{LookupCache, cached_get, empty_cache},
};

/// Maps ranges of indices to values for efficient reverse lookups.
///
/// Stores first_index values in a sorted Vec and uses binary search
/// to find the value for any index. The value is derived from the position.
///
/// Includes a direct-mapped cache for O(1) floor lookups when there's locality.
pub struct RangeMap<I, V> {
    first_indexes: Vec<I>,
    cache: LookupCache<I, V>,
}

impl<I: Default + Copy, V: Default + Copy> Clone for RangeMap<I, V> {
    fn clone(&self) -> Self {
        Self::from(self.first_indexes.clone())
    }
}

impl<I: Default + Copy, V: Default + Copy> From<Vec<I>> for RangeMap<I, V> {
    fn from(first_indexes: Vec<I>) -> Self {
        Self {
            first_indexes,
            cache: empty_cache(),
        }
    }
}

impl<I: Default + Copy, V: Default + Copy> Default for RangeMap<I, V> {
    fn default() -> Self {
        Self::from(Vec::new())
    }
}

impl<I, V> RangeMap<I, V> {
    /// Number of ranges stored.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.first_indexes.len()
    }

    /// Forward mapping: the first index of each range, in value order.
    pub fn as_slice(&self) -> &[I] {
        &self.first_indexes
    }

    /// Truncate to `new_len` ranges and clear the cache.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len() {
            self.first_indexes.truncate(new_len);
            self.clear_cache();
        }
    }

    fn clear_cache(&mut self) {
        for entry in self.cache.iter_mut() {
            entry.3 = false;
        }
    }
}

impl<I: Ord + Copy, V> RangeMap<I, V> {
    /// Push a new first_index. Value is implicitly the current length.
    /// Must be called in order (first_index must be >= all previous).
    #[inline]
    pub fn push(&mut self, first_index: I) {
        debug_assert!(
            self.first_indexes
                .last()
                .is_none_or(|&last| first_index >= last),
            "RangeMap: first_index must be monotonically increasing"
        );
        self.first_indexes.push(first_index);
    }

    /// Extend an ordered suffix without rebuilding the unchanged prefix.
    pub fn extend(&mut self, values: impl IntoIterator<Item = I>) {
        let values = values.into_iter();
        self.first_indexes.reserve(values.size_hint().0);
        for value in values {
            self.push(value);
        }
    }
}

impl<I: Ord + Copy + Default + Into<usize>, V: From<usize> + Copy + Default> RangeMap<I, V> {
    /// Request-local floor lookup hint. The shared borrow prevents mutations
    /// from invalidating a remembered interval.
    pub fn cursor(&self) -> RangeMapCursor<'_, I, V> {
        RangeMapCursor::new(self)
    }

    /// Compute-local cache for interleaved lookups without cloning boundaries.
    pub fn cached_cursor(&self) -> CachedRangeMapCursor<'_, I, V> {
        CachedRangeMapCursor::new(self)
    }

    /// Floor: returns the value (position) of the largest first_index <= given index.
    #[inline]
    pub fn get(&mut self, index: I) -> Option<V> {
        cached_get(&self.first_indexes, &mut self.cache, index)
    }

    /// Shared (immutable) floor lookup — binary search only, no cache update.
    /// Use when you only have `&self` (e.g. read-only clones in the query layer).
    #[inline]
    pub fn get_shared(&self, index: I) -> Option<V> {
        if self.first_indexes.is_empty() {
            return None;
        }
        let pos = self.first_indexes.partition_point(|&first| first <= index);
        if pos > 0 {
            Some(V::from(pos - 1))
        } else {
            None
        }
    }
}

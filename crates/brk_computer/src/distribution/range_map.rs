use std::marker::PhantomData;

/// Direct-mapped cache size. Power of 2 for fast masking.
const CACHE_SIZE: usize = 128;
const CACHE_MASK: usize = CACHE_SIZE - 1;

/// Maps ranges of indices to values for efficient reverse lookups.
///
/// Instead of storing a value for every index, stores first_index values
/// in a sorted Vec and uses binary search to find the value for any index.
/// The value is derived from the position in the Vec.
///
/// Includes a direct-mapped cache for O(1) lookups when there's locality.
#[derive(Debug, Clone)]
pub struct RangeMap<I, V> {
    /// Sorted vec of first_index values. Position in vec = value.
    first_indexes: Vec<I>,
    /// Direct-mapped cache: (range_low, range_high, value, occupied). Inline for zero indirection.
    cache: [(I, I, V, bool); CACHE_SIZE],
    _phantom: PhantomData<V>,
}

impl<I: Default + Copy, V: Default + Copy> Default for RangeMap<I, V> {
    fn default() -> Self {
        Self {
            first_indexes: Vec::new(),
            cache: [(I::default(), I::default(), V::default(), false); CACHE_SIZE],
            _phantom: PhantomData,
        }
    }
}

impl<I: Ord + Copy + Default + Into<usize>, V: From<usize> + Copy + Default> RangeMap<I, V> {
    /// Number of ranges stored.
    pub(crate) fn len(&self) -> usize {
        self.first_indexes.len()
    }

    /// Truncate to `new_len` ranges and clear the cache.
    pub(crate) fn truncate(&mut self, new_len: usize) {
        self.first_indexes.truncate(new_len);
        self.clear_cache();
    }

    /// Push a new first_index. Value is implicitly the current length.
    /// Must be called in order (first_index must be >= all previous).
    #[inline]
    pub(crate) fn push(&mut self, first_index: I) {
        debug_assert!(
            self.first_indexes
                .last()
                .is_none_or(|&last| first_index >= last),
            "RangeMap: first_index must be monotonically increasing"
        );
        self.first_indexes.push(first_index);
    }

    /// Look up value for an index, checking cache first.
    /// Returns the value (position) of the largest first_index <= given index.
    #[inline]
    pub(crate) fn get(&mut self, index: I) -> Option<V> {
        if self.first_indexes.is_empty() {
            return None;
        }

        // Direct-mapped cache lookup: O(1), no aging
        let slot = Self::cache_slot(&index);
        let entry = &self.cache[slot];
        if entry.3 && index >= entry.0 && index < entry.1 {
            return Some(entry.2);
        }

        // Cache miss - binary search
        let pos = self.first_indexes.partition_point(|&first| first <= index);
        if pos > 0 {
            let value = V::from(pos - 1);
            let low = self.first_indexes[pos - 1];
            let is_last = pos == self.first_indexes.len();

            // Cache non-last ranges (last range has unbounded high)
            if !is_last {
                let high = self.first_indexes[pos];
                self.cache[slot] = (low, high, value, true);
            }

            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn cache_slot(index: &I) -> usize {
        let v: usize = (*index).into();
        v & CACHE_MASK
    }

    fn clear_cache(&mut self) {
        for entry in self.cache.iter_mut() {
            entry.3 = false;
        }
    }
}

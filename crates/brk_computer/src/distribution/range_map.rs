use std::marker::PhantomData;

/// Number of ranges to cache. Small enough for O(1) linear scan,
/// large enough to cover the "hot" source blocks in a typical block.
const CACHE_SIZE: usize = 8;

/// Maps ranges of indices to values for efficient reverse lookups.
///
/// Instead of storing a value for every index, stores first_index values
/// in a sorted Vec and uses binary search to find the value for any index.
/// The value is derived from the position in the Vec.
///
/// Includes an LRU cache of recently accessed ranges to avoid binary search
/// when there's locality in access patterns.
#[derive(Debug)]
pub struct RangeMap<I, V> {
    /// Sorted vec of first_index values. Position in vec = value.
    first_indexes: Vec<I>,
    /// LRU cache: (range_low, range_high, value, age). Lower age = more recent.
    cache: [(I, I, V, u8); CACHE_SIZE],
    cache_len: u8,
    _phantom: PhantomData<V>,
}

impl<I: Default + Copy, V: Default + Copy> Default for RangeMap<I, V> {
    fn default() -> Self {
        Self {
            first_indexes: Vec::new(),
            cache: [(I::default(), I::default(), V::default(), 0); CACHE_SIZE],
            cache_len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<I: Ord + Copy + Default, V: From<usize> + Copy + Default> RangeMap<I, V> {
    /// Create with pre-allocated capacity.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            first_indexes: Vec::with_capacity(capacity),
            cache: [(I::default(), I::default(), V::default(), 0); CACHE_SIZE],
            cache_len: 0,
            _phantom: PhantomData,
        }
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

        let cache_len = self.cache_len as usize;

        // Check cache first (linear scan of small array)
        for i in 0..cache_len {
            let (low, high, value, _) = self.cache[i];
            if index >= low && index < high {
                // Cache hit - mark as most recently used
                if self.cache[i].3 != 0 {
                    for j in 0..cache_len {
                        self.cache[j].3 = self.cache[j].3.saturating_add(1);
                    }
                    self.cache[i].3 = 0;
                }
                return Some(value);
            }
        }

        // Cache miss - binary search
        let pos = self.first_indexes.partition_point(|&first| first <= index);
        if pos > 0 {
            let value = V::from(pos - 1);
            let low = self.first_indexes[pos - 1];

            // For last range, use low as high (special marker)
            // The check `index < high` will fail, but `index >= low` handles it
            let high = self.first_indexes.get(pos).copied().unwrap_or(low);
            let is_last = pos == self.first_indexes.len();

            // Add to cache (skip if last range - unbounded high is tricky)
            if !is_last {
                self.add_to_cache(low, high, value);
            }

            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn add_to_cache(&mut self, low: I, high: I, value: V) {
        let cache_len = self.cache_len as usize;

        // Age all entries
        for i in 0..cache_len {
            self.cache[i].3 = self.cache[i].3.saturating_add(1);
        }

        if cache_len < CACHE_SIZE {
            // Not full - append
            self.cache[cache_len] = (low, high, value, 0);
            self.cache_len += 1;
        } else {
            // Full - evict oldest (highest age)
            let mut oldest_idx = 0;
            let mut oldest_age = 0u8;
            for i in 0..CACHE_SIZE {
                if self.cache[i].3 > oldest_age {
                    oldest_age = self.cache[i].3;
                    oldest_idx = i;
                }
            }
            self.cache[oldest_idx] = (low, high, value, 0);
        }
    }
}

//! Range-based lookup map for efficient index -> value lookups.
//!
//! Uses the pattern that many indices share the same value (e.g., all txindexes
//! in a block have the same height) to provide O(log n) lookups via BTreeMap.

use std::collections::BTreeMap;

use vecdb::VecIndex;

/// Maps ranges of indices to values for efficient reverse lookups.
///
/// Instead of storing a value for every index, stores (first_index, value)
/// pairs and uses range search to find the value for any index.
#[derive(Debug, Default)]
pub struct RangeMap<I, V>(BTreeMap<I, V>);

impl<I: VecIndex, V: Copy> RangeMap<I, V> {
    /// Create a new empty map.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Insert a new (first_index, value) mapping.
    #[inline]
    pub fn insert(&mut self, first_index: I, value: V) {
        self.0.insert(first_index, value);
    }

    /// Look up value for an index using range search.
    /// Returns the value associated with the largest first_index <= given index.
    #[inline]
    pub fn get(&self, index: I) -> Option<V> {
        self.0.range(..=index).next_back().map(|(_, &v)| v)
    }

    /// Clear all entries (for reset/rollback).
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

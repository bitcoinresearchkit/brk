//! Address data cache for flush intervals.
//!
//! Accumulates address data across blocks within a flush interval.
//! Data is flushed to disk at checkpoints.

use brk_types::{OutputType, TypeIndex};

use super::super::address::AddressTypeToTypeIndexMap;
use super::{AddressLookup, EmptyAddressDataWithSource, LoadedAddressDataWithSource, TxIndexVec};

/// Cache for address data within a flush interval.
pub struct AddressCache {
    /// Addresses with non-zero balance
    loaded: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    /// Addresses that became empty (zero balance)
    empty: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
}

impl Default for AddressCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressCache {
    pub fn new() -> Self {
        Self {
            loaded: AddressTypeToTypeIndexMap::default(),
            empty: AddressTypeToTypeIndexMap::default(),
        }
    }

    /// Check if address is in cache (either loaded or empty).
    #[inline]
    pub fn contains(&self, address_type: OutputType, typeindex: TypeIndex) -> bool {
        self.loaded
            .get(address_type)
            .is_some_and(|m| m.contains_key(&typeindex))
            || self
                .empty
                .get(address_type)
                .is_some_and(|m| m.contains_key(&typeindex))
    }

    /// Merge address data into loaded cache.
    #[inline]
    pub fn merge_loaded(&mut self, data: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>) {
        self.loaded.merge_mut(data);
    }

    /// Create an AddressLookup view into this cache.
    #[inline]
    pub fn as_lookup(&mut self) -> AddressLookup<'_> {
        AddressLookup {
            loaded: &mut self.loaded,
            empty: &mut self.empty,
        }
    }

    /// Update transaction counts for addresses.
    pub fn update_tx_counts(&mut self, txindex_vecs: AddressTypeToTypeIndexMap<TxIndexVec>) {
        super::update_tx_counts(&mut self.loaded, &mut self.empty, txindex_vecs);
    }

    /// Take the cache contents for flushing, leaving empty caches.
    pub fn take(
        &mut self,
    ) -> (
        AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
        AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    ) {
        (
            std::mem::take(&mut self.empty),
            std::mem::take(&mut self.loaded),
        )
    }
}

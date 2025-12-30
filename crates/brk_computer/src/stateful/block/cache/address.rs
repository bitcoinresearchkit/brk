use brk_cohort::ByAddressType;
use brk_types::{AnyAddressDataIndexEnum, LoadedAddressData, OutputType, TypeIndex};
use vecdb::GenericStoredVec;

use crate::stateful::{
    address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs},
    compute::VecsReaders,
};

use super::super::cohort::{
    EmptyAddressDataWithSource, LoadedAddressDataWithSource, TxIndexVec, WithAddressDataSource,
    update_tx_counts,
};
use super::lookup::AddressLookup;

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
        update_tx_counts(&mut self.loaded, &mut self.empty, txindex_vecs);
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

/// Load address data from storage or create new.
///
/// Returns None if address is already in cache (loaded or empty).
#[allow(clippy::too_many_arguments)]
pub fn load_uncached_address_data(
    address_type: OutputType,
    typeindex: TypeIndex,
    first_addressindexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> Option<LoadedAddressDataWithSource> {
    // Check if this is a new address (typeindex >= first for this height)
    let first = *first_addressindexes.get(address_type).unwrap();
    if first <= typeindex {
        return Some(WithAddressDataSource::New(LoadedAddressData::default()));
    }

    // Skip if already in cache
    if cache.contains(address_type, typeindex) {
        return None;
    }

    // Read from storage
    let reader = vr.address_reader(address_type);
    let anyaddressindex = any_address_indexes.get(address_type, typeindex, reader);

    Some(match anyaddressindex.to_enum() {
        AnyAddressDataIndexEnum::Loaded(loaded_index) => {
            let reader = &vr.anyaddressindex_to_anyaddressdata.loaded;
            let loaded_data = addresses_data
                .loaded
                .get_pushed_or_read_unwrap(loaded_index, reader);
            WithAddressDataSource::FromLoaded(loaded_index, loaded_data)
        }
        AnyAddressDataIndexEnum::Empty(empty_index) => {
            let reader = &vr.anyaddressindex_to_anyaddressdata.empty;
            let empty_data = addresses_data
                .empty
                .get_pushed_or_read_unwrap(empty_index, reader);
            WithAddressDataSource::FromEmpty(empty_index, empty_data.into())
        }
    })
}

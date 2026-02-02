use brk_cohort::ByAddressType;
use brk_types::{AnyAddressDataIndexEnum, FundedAddressData, OutputType, TypeIndex};

use crate::distribution::{
    address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs},
    compute::VecsReaders,
};

use super::super::cohort::{
    EmptyAddressDataWithSource, FundedAddressDataWithSource, TxIndexVec, WithAddressDataSource,
    update_tx_counts,
};
use super::lookup::AddressLookup;

/// Cache for address data within a flush interval.
pub struct AddressCache {
    /// Addresses with non-zero balance
    funded: AddressTypeToTypeIndexMap<FundedAddressDataWithSource>,
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
            funded: AddressTypeToTypeIndexMap::default(),
            empty: AddressTypeToTypeIndexMap::default(),
        }
    }

    /// Check if address is in cache (either funded or empty).
    #[inline]
    pub fn contains(&self, address_type: OutputType, typeindex: TypeIndex) -> bool {
        self.funded
            .get(address_type)
            .is_some_and(|m| m.contains_key(&typeindex))
            || self
                .empty
                .get(address_type)
                .is_some_and(|m| m.contains_key(&typeindex))
    }

    /// Merge address data into funded cache.
    #[inline]
    pub fn merge_funded(&mut self, data: AddressTypeToTypeIndexMap<FundedAddressDataWithSource>) {
        self.funded.merge_mut(data);
    }

    /// Create an AddressLookup view into this cache.
    #[inline]
    pub fn as_lookup(&mut self) -> AddressLookup<'_> {
        AddressLookup {
            funded: &mut self.funded,
            empty: &mut self.empty,
        }
    }

    /// Update transaction counts for addresses.
    pub fn update_tx_counts(&mut self, txindex_vecs: AddressTypeToTypeIndexMap<TxIndexVec>) {
        update_tx_counts(&mut self.funded, &mut self.empty, txindex_vecs);
    }

    /// Take the cache contents for flushing, leaving empty caches.
    pub fn take(
        &mut self,
    ) -> (
        AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
        AddressTypeToTypeIndexMap<FundedAddressDataWithSource>,
    ) {
        (
            std::mem::take(&mut self.empty),
            std::mem::take(&mut self.funded),
        )
    }
}

/// Load address data from storage or create new.
///
/// Returns None if address is already in cache (funded or empty).
#[allow(clippy::too_many_arguments)]
pub fn load_uncached_address_data(
    address_type: OutputType,
    typeindex: TypeIndex,
    first_addressindexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> Option<FundedAddressDataWithSource> {
    // Check if this is a new address (typeindex >= first for this height)
    let first = *first_addressindexes.get(address_type).unwrap();
    if first <= typeindex {
        return Some(WithAddressDataSource::New(FundedAddressData::default()));
    }

    // Skip if already in cache
    if cache.contains(address_type, typeindex) {
        return None;
    }

    // Read from storage
    let reader = vr.address_reader(address_type);
    let anyaddressindex = any_address_indexes.get(address_type, typeindex, reader);

    Some(match anyaddressindex.to_enum() {
        AnyAddressDataIndexEnum::Funded(funded_index) => {
            let reader = &vr.anyaddressindex_to_anyaddressdata.funded;
            // Use get_any_or_read_unwrap to check updated layer (needed after rollback)
            let funded_data = addresses_data
                .funded
                .get_any_or_read_unwrap(funded_index, reader);
            WithAddressDataSource::FromFunded(funded_index, funded_data)
        }
        AnyAddressDataIndexEnum::Empty(empty_index) => {
            let reader = &vr.anyaddressindex_to_anyaddressdata.empty;
            // Use get_any_or_read_unwrap to check updated layer (needed after rollback)
            let empty_data = addresses_data
                .empty
                .get_any_or_read_unwrap(empty_index, reader);
            WithAddressDataSource::FromEmpty(empty_index, empty_data.into())
        }
    })
}

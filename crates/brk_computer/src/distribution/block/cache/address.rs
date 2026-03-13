use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_types::{
    AnyAddressDataIndexEnum, EmptyAddressData, FundedAddressData, OutputType, TxIndex, TypeIndex,
};
use smallvec::SmallVec;

use crate::distribution::{
    address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs},
    compute::VecsReaders,
};

use super::super::cohort::{WithAddressDataSource, update_tx_counts};
use super::lookup::AddressLookup;

/// Cache for address data within a flush interval.
pub struct AddressCache {
    /// Addresses with non-zero balance
    funded: AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
    /// Addresses that became empty (zero balance)
    empty: AddressTypeToTypeIndexMap<WithAddressDataSource<EmptyAddressData>>,
}

impl Default for AddressCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressCache {
    pub(crate) fn new() -> Self {
        Self {
            funded: AddressTypeToTypeIndexMap::default(),
            empty: AddressTypeToTypeIndexMap::default(),
        }
    }

    /// Check if address is in cache (either funded or empty).
    #[inline]
    pub(crate) fn contains(&self, address_type: OutputType, type_index: TypeIndex) -> bool {
        self.funded
            .get(address_type)
            .is_some_and(|m| m.contains_key(&type_index))
            || self
                .empty
                .get(address_type)
                .is_some_and(|m| m.contains_key(&type_index))
    }

    /// Merge address data into funded cache.
    #[inline]
    pub(crate) fn merge_funded(
        &mut self,
        data: AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
    ) {
        self.funded.merge_mut(data);
    }

    /// Create an AddressLookup view into this cache.
    #[inline]
    pub(crate) fn as_lookup(&mut self) -> AddressLookup<'_> {
        AddressLookup {
            funded: &mut self.funded,
            empty: &mut self.empty,
        }
    }

    /// Update transaction counts for addresses.
    pub(crate) fn update_tx_counts(
        &mut self,
        tx_index_vecs: AddressTypeToTypeIndexMap<SmallVec<[TxIndex; 4]>>,
    ) {
        update_tx_counts(&mut self.funded, &mut self.empty, tx_index_vecs);
    }

    /// Take the cache contents for flushing, leaving empty caches.
    pub(crate) fn take(
        &mut self,
    ) -> (
        AddressTypeToTypeIndexMap<WithAddressDataSource<EmptyAddressData>>,
        AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
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
pub(crate) fn load_uncached_address_data(
    address_type: OutputType,
    type_index: TypeIndex,
    first_address_indexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> Result<Option<WithAddressDataSource<FundedAddressData>>> {
    // Check if this is a new address (type_index >= first for this height)
    let first = *first_address_indexes.get(address_type).unwrap();
    if first <= type_index {
        return Ok(Some(WithAddressDataSource::New(
            FundedAddressData::default(),
        )));
    }

    // Skip if already in cache
    if cache.contains(address_type, type_index) {
        return Ok(None);
    }

    // Read from storage
    let reader = vr.address_reader(address_type);
    let any_address_index = any_address_indexes.get(address_type, type_index, reader)?;

    Ok(Some(match any_address_index.to_enum() {
        AnyAddressDataIndexEnum::Funded(funded_index) => {
            let reader = &vr.any_address_index_to_any_address_data.funded;
            let funded_data = addresses_data
                .funded
                .get_any_or_read_at(funded_index.into(), reader)?
                .unwrap();
            WithAddressDataSource::FromFunded(funded_index, funded_data)
        }
        AnyAddressDataIndexEnum::Empty(empty_index) => {
            let reader = &vr.any_address_index_to_any_address_data.empty;
            let empty_data = addresses_data
                .empty
                .get_any_or_read_at(empty_index.into(), reader)?
                .unwrap();
            WithAddressDataSource::FromEmpty(empty_index, empty_data.into())
        }
    }))
}

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::{
    AnyAddrDataIndexEnum, EmptyAddrData, FundedAddrData, OutputType, TxIndex, TypeIndex,
};
use smallvec::SmallVec;

use crate::distribution::{
    addr::{AddrTypeToTypeIndexMap, AddrsDataVecs, AnyAddrIndexesVecs},
    compute::VecsReaders,
};

use super::super::cohort::{WithAddrDataSource, update_tx_counts};
use super::lookup::AddrLookup;

/// Cache for address data within a flush interval.
pub struct AddrCache {
    /// Addrs with non-zero balance
    funded: AddrTypeToTypeIndexMap<WithAddrDataSource<FundedAddrData>>,
    /// Addrs that became empty (zero balance)
    empty: AddrTypeToTypeIndexMap<WithAddrDataSource<EmptyAddrData>>,
}

impl Default for AddrCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AddrCache {
    pub(crate) fn new() -> Self {
        Self {
            funded: AddrTypeToTypeIndexMap::default(),
            empty: AddrTypeToTypeIndexMap::default(),
        }
    }

    /// Check if address is in cache (either funded or empty).
    #[inline]
    pub(crate) fn contains(&self, addr_type: OutputType, type_index: TypeIndex) -> bool {
        self.funded
            .get(addr_type)
            .is_some_and(|m| m.contains_key(&type_index))
            || self
                .empty
                .get(addr_type)
                .is_some_and(|m| m.contains_key(&type_index))
    }

    /// Merge address data into funded cache.
    #[inline]
    pub(crate) fn merge_funded(
        &mut self,
        data: AddrTypeToTypeIndexMap<WithAddrDataSource<FundedAddrData>>,
    ) {
        self.funded.merge_mut(data);
    }

    /// Create an AddrLookup view into this cache.
    #[inline]
    pub(crate) fn as_lookup(&mut self) -> AddrLookup<'_> {
        AddrLookup {
            funded: &mut self.funded,
            empty: &mut self.empty,
        }
    }

    /// Update transaction counts for addresses.
    pub(crate) fn update_tx_counts(
        &mut self,
        tx_index_vecs: AddrTypeToTypeIndexMap<SmallVec<[TxIndex; 4]>>,
    ) {
        update_tx_counts(&mut self.funded, &mut self.empty, tx_index_vecs);
    }

    /// Take the cache contents for flushing, leaving empty caches.
    pub(crate) fn take(
        &mut self,
    ) -> (
        AddrTypeToTypeIndexMap<WithAddrDataSource<EmptyAddrData>>,
        AddrTypeToTypeIndexMap<WithAddrDataSource<FundedAddrData>>,
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
pub(crate) fn load_uncached_addr_data(
    addr_type: OutputType,
    type_index: TypeIndex,
    first_addr_indexes: &ByAddrType<TypeIndex>,
    cache: &AddrCache,
    vr: &VecsReaders,
    any_addr_indexes: &AnyAddrIndexesVecs,
    addrs_data: &AddrsDataVecs,
) -> Result<Option<WithAddrDataSource<FundedAddrData>>> {
    // Check if this is a new address (type_index >= first for this height)
    let first = *first_addr_indexes.get(addr_type).unwrap();
    if first <= type_index {
        return Ok(Some(WithAddrDataSource::New(
            FundedAddrData::default(),
        )));
    }

    // Skip if already in cache
    if cache.contains(addr_type, type_index) {
        return Ok(None);
    }

    // Read from storage
    let reader = vr.addr_reader(addr_type);
    let any_addr_index = any_addr_indexes.get(addr_type, type_index, reader)?;

    Ok(Some(match any_addr_index.to_enum() {
        AnyAddrDataIndexEnum::Funded(funded_index) => {
            let reader = &vr.any_addr_index_to_any_addr_data.funded;
            let funded_data = addrs_data
                .funded
                .get_any_or_read_at(funded_index.into(), reader)?
                .unwrap();
            WithAddrDataSource::FromFunded(funded_index, funded_data)
        }
        AnyAddrDataIndexEnum::Empty(empty_index) => {
            let reader = &vr.any_addr_index_to_any_addr_data.empty;
            let empty_data = addrs_data
                .empty
                .get_any_or_read_at(empty_index.into(), reader)?
                .unwrap();
            WithAddrDataSource::FromEmpty(empty_index, empty_data.into())
        }
    }))
}

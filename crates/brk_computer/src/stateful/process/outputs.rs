//! Output processing.
//!
//! Processes a block's outputs (new UTXOs), building:
//! - Transacted: aggregated supply by output type and amount range
//! - Address data for address cohort tracking (optional)

use brk_grouper::ByAddressType;
use brk_types::{AnyAddressDataIndexEnum, LoadedAddressData, OutputType, Sats, TxIndex, TypeIndex};
use vecdb::GenericStoredVec;

use crate::stateful::address::{
    AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs,
};
use crate::stateful::compute::VecsReaders;
use crate::stateful::states::Transacted;

use super::super::address::AddressTypeToVec;
use super::{AddressCache, LoadedAddressDataWithSource, TxIndexVec, WithAddressDataSource};

/// Result of processing outputs for a block.
pub struct OutputsResult {
    /// Aggregated supply transacted in this block.
    pub transacted: Transacted,
    /// Per-address-type received data: (typeindex, value) for each address.
    pub received_data: AddressTypeToVec<(TypeIndex, Sats)>,
    /// Address data looked up during processing, keyed by (address_type, typeindex).
    pub address_data: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    /// Transaction indexes per address for tx_count tracking.
    pub txindex_vecs: AddressTypeToTypeIndexMap<TxIndexVec>,
}

/// Process outputs (new UTXOs) for a block.
///
/// For each output:
/// 1. Read pre-collected value, output type, and typeindex
/// 2. Accumulate into Transacted by type and amount
/// 3. Look up address data if output is an address type
/// 4. Track address-specific data for address cohort processing
#[allow(clippy::too_many_arguments)]
pub fn process_outputs(
    output_count: usize,
    txoutindex_to_txindex: &[TxIndex],
    // Pre-collected output data (from reusable iterators with 16KB buffered reads)
    values: &[Sats],
    output_types: &[OutputType],
    typeindexes: &[TypeIndex],
    // Address lookup parameters
    first_addressindexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> OutputsResult {
    // Pre-allocate result structures
    let estimated_per_type = (output_count / 8).max(8);
    let mut transacted = Transacted::default();
    let mut received_data = AddressTypeToVec::with_capacity(estimated_per_type);
    let mut address_data =
        AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::with_capacity(estimated_per_type);
    let mut txindex_vecs =
        AddressTypeToTypeIndexMap::<TxIndexVec>::with_capacity(estimated_per_type);

    // Single pass: read from pre-collected vecs and accumulate
    for local_idx in 0..output_count {
        let txindex = txoutindex_to_txindex[local_idx];
        let value = values[local_idx];
        let output_type = output_types[local_idx];

        transacted.iterate(value, output_type);

        if output_type.is_not_address() {
            continue;
        }

        let typeindex = typeindexes[local_idx];

        received_data
            .get_mut(output_type)
            .unwrap()
            .push((typeindex, value));

        let addr_data_opt = get_address_data(
            output_type,
            typeindex,
            first_addressindexes,
            cache,
            vr,
            any_address_indexes,
            addresses_data,
        );

        if let Some(addr_data) = addr_data_opt {
            address_data.insert_for_type(output_type, typeindex, addr_data);
        }

        txindex_vecs
            .get_mut(output_type)
            .unwrap()
            .entry(typeindex)
            .or_default()
            .push(txindex);
    }

    OutputsResult {
        transacted,
        received_data,
        address_data,
        txindex_vecs,
    }
}

/// Look up address data from storage or determine if new.
///
/// Returns None if address is already in loaded or empty cache.
#[allow(clippy::too_many_arguments)]
fn get_address_data(
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

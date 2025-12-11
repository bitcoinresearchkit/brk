//! Parallel output processing.
//!
//! Processes a block's outputs (new UTXOs) in parallel, building:
//! - Transacted: aggregated supply by output type and amount range
//! - Address data for address cohort tracking (optional)

use brk_grouper::ByAddressType;
use brk_types::{
    AnyAddressDataIndexEnum, LoadedAddressData, OutputType, Sats, TxIndex, TxOutIndex, TypeIndex,
};
use rayon::prelude::*;
use smallvec::SmallVec;
use vecdb::{BytesVec, GenericStoredVec};

use crate::stateful_new::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};
use crate::stateful_new::compute::VecsReaders;
use crate::{stateful_new::IndexerReaders, states::Transacted};

use super::super::address::AddressTypeToVec;
use super::{EmptyAddressDataWithSource, LoadedAddressDataWithSource, WithAddressDataSource};

/// SmallVec for transaction indexes - most addresses have few transactions per block.
pub type TxIndexVec = SmallVec<[TxIndex; 4]>;

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

/// Process outputs (new UTXOs) for a block in parallel.
///
/// For each output:
/// 1. Read value and output type from indexer
/// 2. Accumulate into Transacted by type and amount
/// 3. Look up address data if output is an address type
/// 4. Track address-specific data for address cohort processing
#[allow(clippy::too_many_arguments)]
pub fn process_outputs(
    first_txoutindex: usize,
    output_count: usize,
    txoutindex_to_txindex: &[TxIndex],
    txoutindex_to_value: &BytesVec<TxOutIndex, Sats>,
    txoutindex_to_outputtype: &BytesVec<TxOutIndex, OutputType>,
    txoutindex_to_typeindex: &BytesVec<TxOutIndex, TypeIndex>,
    ir: &IndexerReaders,
    // Address lookup parameters
    first_addressindexes: &ByAddressType<TypeIndex>,
    loaded_cache: &AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    empty_cache: &AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> OutputsResult {
    let (transacted, received_data, address_data, txindex_vecs) = (first_txoutindex
        ..first_txoutindex + output_count)
        .into_par_iter()
        .map(|i| {
            let txoutindex = TxOutIndex::from(i);
            let local_idx = i - first_txoutindex;
            let txindex = txoutindex_to_txindex[local_idx];

            let value = txoutindex_to_value.read_unwrap(txoutindex, &ir.txoutindex_to_value);
            let output_type =
                txoutindex_to_outputtype.read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

            // Non-address outputs don't need typeindex or address lookup
            if output_type.is_not_address() {
                return (value, output_type, None);
            }

            let typeindex =
                txoutindex_to_typeindex.read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

            // Look up address data
            let addr_data_opt = get_address_data(
                output_type,
                typeindex,
                first_addressindexes,
                loaded_cache,
                empty_cache,
                vr,
                any_address_indexes,
                addresses_data,
            );

            (value, output_type, Some((typeindex, txindex, value, addr_data_opt)))
        })
        .fold(
            || {
                (
                    Transacted::default(),
                    AddressTypeToVec::default(),
                    AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::default(),
                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                )
            },
            |(mut transacted, mut received_data, mut address_data, mut txindex_vecs),
             (value, output_type, addr_info)| {
                transacted.iterate(value, output_type);

                if let Some((typeindex, txindex, value, addr_data_opt)) = addr_info {
                    received_data
                        .get_mut(output_type)
                        .unwrap()
                        .push((typeindex, value));

                    if let Some(addr_data) = addr_data_opt {
                        address_data.insert_for_type(output_type, typeindex, addr_data);
                    }

                    txindex_vecs
                        .get_mut(output_type)
                        .unwrap()
                        .entry(typeindex)
                        .or_insert_with(TxIndexVec::new)
                        .push(txindex);
                }

                (transacted, received_data, address_data, txindex_vecs)
            },
        )
        .reduce(
            || {
                (
                    Transacted::default(),
                    AddressTypeToVec::default(),
                    AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::default(),
                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                )
            },
            |(t1, r1, a1, tx1), (t2, r2, a2, tx2)| {
                (t1 + t2, r1.merge(r2), a1.merge(a2), tx1.merge_vec(tx2))
            },
        );

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
fn get_address_data(
    address_type: OutputType,
    typeindex: TypeIndex,
    first_addressindexes: &ByAddressType<TypeIndex>,
    loaded_cache: &AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    empty_cache: &AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
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
    if loaded_cache
        .get(address_type)
        .unwrap()
        .contains_key(&typeindex)
        || empty_cache
            .get(address_type)
            .unwrap()
            .contains_key(&typeindex)
    {
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

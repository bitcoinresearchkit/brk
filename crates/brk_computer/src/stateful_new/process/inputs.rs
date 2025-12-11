//! Parallel input processing.
//!
//! Processes a block's inputs (spent UTXOs) in parallel, building:
//! - height_to_sent: map from creation height -> Transacted for sends
//! - Address data for address cohort tracking (optional)

use brk_grouper::ByAddressType;
use brk_types::{
    AnyAddressDataIndexEnum, Height, LoadedAddressData, OutPoint, OutputType, Sats, TxInIndex,
    TxIndex, TxOutIndex, TypeIndex,
};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use vecdb::{BytesVec, GenericStoredVec, PcoVec};

use crate::stateful_new::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};
use crate::stateful_new::compute::VecsReaders;
use crate::{
    stateful_new::{IndexerReaders, process::RangeMap},
    states::Transacted,
};

use super::super::address::HeightToAddressTypeToVec;
use super::{EmptyAddressDataWithSource, LoadedAddressDataWithSource, TxIndexVec, WithAddressDataSource};

/// Result of processing inputs for a block.
pub struct InputsResult {
    /// Map from UTXO creation height -> aggregated sent supply.
    pub height_to_sent: FxHashMap<Height, Transacted>,
    /// Per-height, per-address-type sent data: (typeindex, value) for each address.
    pub sent_data: HeightToAddressTypeToVec<(TypeIndex, Sats)>,
    /// Address data looked up during processing, keyed by (address_type, typeindex).
    pub address_data: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    /// Transaction indexes per address for tx_count tracking.
    pub txindex_vecs: AddressTypeToTypeIndexMap<TxIndexVec>,
}

/// Process inputs (spent UTXOs) for a block in parallel.
///
/// For each input:
/// 1. Read outpoint, resolve to txoutindex
/// 2. Get the creation height from txoutindex_to_height map
/// 3. Read value and type from the referenced output
/// 4. Look up address data if input references an address type
/// 5. Accumulate into height_to_sent map
/// 6. Track address-specific data for address cohort processing
#[allow(clippy::too_many_arguments)]
pub fn process_inputs(
    first_txinindex: usize,
    input_count: usize,
    txinindex_to_txindex: &[TxIndex],
    txinindex_to_outpoint: &PcoVec<TxInIndex, OutPoint>,
    txindex_to_first_txoutindex: &BytesVec<TxIndex, TxOutIndex>,
    txoutindex_to_value: &BytesVec<TxOutIndex, Sats>,
    txoutindex_to_outputtype: &BytesVec<TxOutIndex, OutputType>,
    txoutindex_to_typeindex: &BytesVec<TxOutIndex, TypeIndex>,
    txoutindex_to_height: &RangeMap<TxOutIndex, Height>,
    ir: &IndexerReaders,
    // Address lookup parameters
    first_addressindexes: &ByAddressType<TypeIndex>,
    loaded_cache: &AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    empty_cache: &AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> InputsResult {
    let (height_to_sent, sent_data, address_data, txindex_vecs) = (first_txinindex
        ..first_txinindex + input_count)
        .into_par_iter()
        .map(|i| {
            let txinindex = TxInIndex::from(i);
            let local_idx = i - first_txinindex;
            let txindex = txinindex_to_txindex[local_idx];

            // Get outpoint and resolve to txoutindex
            let outpoint = txinindex_to_outpoint.read_unwrap(txinindex, &ir.txinindex_to_outpoint);
            let first_txoutindex = txindex_to_first_txoutindex
                .read_unwrap(outpoint.txindex(), &ir.txindex_to_first_txoutindex);
            let txoutindex = first_txoutindex + outpoint.vout();

            // Get creation height
            let prev_height = *txoutindex_to_height.get(txoutindex).unwrap();

            // Get value and type from the output being spent
            let value = txoutindex_to_value.read_unwrap(txoutindex, &ir.txoutindex_to_value);
            let input_type =
                txoutindex_to_outputtype.read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

            // Non-address inputs don't need typeindex or address lookup
            if input_type.is_not_address() {
                return (prev_height, value, input_type, None);
            }

            let typeindex =
                txoutindex_to_typeindex.read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

            // Look up address data
            let addr_data_opt = get_address_data(
                input_type,
                typeindex,
                first_addressindexes,
                loaded_cache,
                empty_cache,
                vr,
                any_address_indexes,
                addresses_data,
            );

            (
                prev_height,
                value,
                input_type,
                Some((typeindex, txindex, value, addr_data_opt)),
            )
        })
        .fold(
            || {
                (
                    FxHashMap::<Height, Transacted>::default(),
                    HeightToAddressTypeToVec::default(),
                    AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::default(),
                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                )
            },
            |(mut height_to_sent, mut sent_data, mut address_data, mut txindex_vecs),
             (prev_height, value, output_type, addr_info)| {
                height_to_sent
                    .entry(prev_height)
                    .or_default()
                    .iterate(value, output_type);

                if let Some((typeindex, txindex, value, addr_data_opt)) = addr_info {
                    sent_data
                        .entry(prev_height)
                        .or_default()
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

                (height_to_sent, sent_data, address_data, txindex_vecs)
            },
        )
        .reduce(
            || {
                (
                    FxHashMap::<Height, Transacted>::default(),
                    HeightToAddressTypeToVec::default(),
                    AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::default(),
                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                )
            },
            |(mut h1, mut s1, a1, tx1), (h2, s2, a2, tx2)| {
                // Merge height_to_sent maps
                for (k, v) in h2 {
                    *h1.entry(k).or_default() += v;
                }

                // Merge sent_data maps
                s1.merge_mut(s2);

                (h1, s1, a1.merge(a2), tx1.merge_vec(tx2))
            },
        );

    InputsResult {
        height_to_sent,
        sent_data,
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

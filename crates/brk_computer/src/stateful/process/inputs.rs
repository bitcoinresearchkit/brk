//! Parallel input processing.
//!
//! Processes a block's inputs (spent UTXOs) in parallel, building:
//! - height_to_sent: map from creation height -> Transacted for sends
//! - Address data for address cohort tracking (optional)

use brk_grouper::ByAddressType;
use brk_types::{Height, OutPoint, OutputType, Sats, TxInIndex, TxIndex, TxOutIndex, TypeIndex};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use vecdb::{BytesVec, GenericStoredVec};

use crate::stateful::address::{
    AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs,
};
use crate::stateful::compute::VecsReaders;
use crate::stateful::states::Transacted;
use crate::stateful::{IndexerReaders, process::RangeMap};

use super::super::address::HeightToAddressTypeToVec;
use super::{load_uncached_address_data, AddressCache, LoadedAddressDataWithSource, TxIndexVec};

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
    /// Updates to txoutindex_to_txinindex: (spent txoutindex, spending txinindex).
    pub txoutindex_to_txinindex_updates: Vec<(TxOutIndex, TxInIndex)>,
}

/// Process inputs (spent UTXOs) for a block.
///
/// For each input:
/// 1. Use pre-collected outpoint (from reusable iterator, avoids PcoVec re-decompression)
/// 2. Resolve outpoint to txoutindex
/// 3. Get the creation height from txoutindex_to_height map
/// 4. Read value and type from the referenced output (random access via mmap)
/// 5. Look up address data if input references an address type
/// 6. Accumulate into height_to_sent map
/// 7. Track address-specific data for address cohort processing
///
/// Uses parallel reads followed by sequential accumulation to avoid
/// expensive merge overhead from rayon's fold/reduce pattern.
#[allow(clippy::too_many_arguments)]
pub fn process_inputs(
    first_txinindex: usize,
    input_count: usize,
    txinindex_to_txindex: &[TxIndex],
    // Pre-collected outpoints (from reusable iterator with page caching)
    outpoints: &[OutPoint],
    txindex_to_first_txoutindex: &BytesVec<TxIndex, TxOutIndex>,
    txoutindex_to_value: &BytesVec<TxOutIndex, Sats>,
    txoutindex_to_outputtype: &BytesVec<TxOutIndex, OutputType>,
    txoutindex_to_typeindex: &BytesVec<TxOutIndex, TypeIndex>,
    txoutindex_to_height: &RangeMap<TxOutIndex, Height>,
    ir: &IndexerReaders,
    // Address lookup parameters
    first_addressindexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> InputsResult {
    // Parallel reads - collect all input data (outpoints already in memory)
    let items: Vec<_> = (0..input_count)
        .into_par_iter()
        .map(|local_idx| {
            let txinindex = TxInIndex::from(first_txinindex + local_idx);
            let txindex = txinindex_to_txindex[local_idx];

            // Get outpoint from pre-collected vec and resolve to txoutindex
            let outpoint = outpoints[local_idx];
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
                return (txinindex, txoutindex, prev_height, value, input_type, None);
            }

            let typeindex =
                txoutindex_to_typeindex.read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

            // Look up address data
            let addr_data_opt = load_uncached_address_data(
                input_type,
                typeindex,
                first_addressindexes,
                cache,
                vr,
                any_address_indexes,
                addresses_data,
            );

            (
                txinindex,
                txoutindex,
                prev_height,
                value,
                input_type,
                Some((typeindex, txindex, value, addr_data_opt)),
            )
        })
        .collect();

    // Phase 2: Sequential accumulation - no merge overhead
    // Estimate: unique heights bounded by block depth, addresses spread across ~8 types
    let estimated_unique_heights = (input_count / 4).max(16);
    let estimated_per_type = (input_count / 8).max(8);
    let mut height_to_sent = FxHashMap::<Height, Transacted>::with_capacity_and_hasher(
        estimated_unique_heights,
        Default::default(),
    );
    let mut sent_data = HeightToAddressTypeToVec::with_capacity(estimated_unique_heights);
    let mut address_data =
        AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::with_capacity(estimated_per_type);
    let mut txindex_vecs =
        AddressTypeToTypeIndexMap::<TxIndexVec>::with_capacity(estimated_per_type);
    let mut txoutindex_to_txinindex_updates = Vec::with_capacity(input_count);

    for (txinindex, txoutindex, prev_height, value, output_type, addr_info) in items {
        height_to_sent
            .entry(prev_height)
            .or_default()
            .iterate(value, output_type);

        txoutindex_to_txinindex_updates.push((txoutindex, txinindex));

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
                .or_default()
                .push(txindex);
        }
    }

    InputsResult {
        height_to_sent,
        sent_data,
        address_data,
        txindex_vecs,
        txoutindex_to_txinindex_updates,
    }
}


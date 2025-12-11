//! Parallel input processing.
//!
//! Processes a block's inputs (spent UTXOs) in parallel, building:
//! - height_to_sent: map from creation height -> Transacted for sends
//! - Address data for address cohort tracking (optional)

use brk_types::{Height, OutPoint, OutputType, Sats, TxInIndex, TxIndex, TxOutIndex, TypeIndex};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use vecdb::{BytesVec, GenericStoredVec, PcoVec};

use crate::{
    stateful_new::{IndexerReaders, process::RangeMap},
    states::Transacted,
};

use super::super::address::HeightToAddressTypeToVec;

/// Result of processing inputs for a block.
pub struct InputsResult {
    /// Map from UTXO creation height -> aggregated sent supply.
    pub height_to_sent: FxHashMap<Height, Transacted>,
    /// Per-height, per-address-type sent data: (typeindex, value) for each address.
    pub sent_data: HeightToAddressTypeToVec<(TypeIndex, Sats)>,
}

/// Process inputs (spent UTXOs) for a block in parallel.
///
/// For each input:
/// 1. Read outpoint, resolve to txoutindex
/// 2. Get the creation height from txoutindex_to_height map
/// 3. Read value and type from the referenced output
/// 4. Accumulate into height_to_sent map
/// 5. Track address-specific data if input references an address type
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
) -> InputsResult {
    let (height_to_sent, sent_data) = (first_txinindex..first_txinindex + input_count)
        .into_par_iter()
        .map(|i| {
            let txinindex = TxInIndex::from(i);
            let local_idx = i - first_txinindex;
            let _txindex = txinindex_to_txindex[local_idx];

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

            // Non-address inputs don't need typeindex
            if input_type.is_not_address() {
                return (prev_height, value, input_type, None);
            }

            let typeindex =
                txoutindex_to_typeindex.read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

            (prev_height, value, input_type, Some((typeindex, value)))
        })
        .fold(
            || {
                (
                    FxHashMap::<Height, Transacted>::default(),
                    HeightToAddressTypeToVec::default(),
                )
            },
            |(mut height_to_sent, mut sent_data), (prev_height, value, output_type, addr_data)| {
                height_to_sent
                    .entry(prev_height)
                    .or_default()
                    .iterate(value, output_type);

                if let Some((typeindex, value)) = addr_data {
                    sent_data
                        .entry(prev_height)
                        .or_default()
                        .get_mut(output_type)
                        .unwrap()
                        .push((typeindex, value));
                }

                (height_to_sent, sent_data)
            },
        )
        .reduce(
            || {
                (
                    FxHashMap::<Height, Transacted>::default(),
                    HeightToAddressTypeToVec::default(),
                )
            },
            |(mut h1, mut s1), (h2, s2)| {
                // Merge height_to_sent maps
                for (k, v) in h2 {
                    *h1.entry(k).or_default() += v;
                }

                // Merge sent_data maps
                s1.merge_mut(s2);

                (h1, s1)
            },
        );

    InputsResult {
        height_to_sent,
        sent_data,
    }
}

//! Parallel output processing.
//!
//! Processes a block's outputs (new UTXOs) in parallel, building:
//! - Transacted: aggregated supply by output type and amount range
//! - Address data for address cohort tracking (optional)

use brk_types::{OutputType, Sats, TxIndex, TxOutIndex, TypeIndex};
use rayon::prelude::*;
use vecdb::{BytesVec, GenericStoredVec};

use crate::{stateful_new::IndexerReaders, states::Transacted};

use super::super::address::AddressTypeToVec;

/// Result of processing outputs for a block.
pub struct OutputsResult {
    /// Aggregated supply transacted in this block.
    pub transacted: Transacted,
    /// Per-address-type received data: (typeindex, value) for each address.
    pub received_data: AddressTypeToVec<(TypeIndex, Sats)>,
}

/// Process outputs (new UTXOs) for a block in parallel.
///
/// For each output:
/// 1. Read value and output type from indexer
/// 2. Accumulate into Transacted by type and amount
/// 3. Track address-specific data if output is an address type
pub fn process_outputs(
    first_txoutindex: usize,
    output_count: usize,
    txoutindex_to_txindex: &[TxIndex],
    txoutindex_to_value: &BytesVec<TxOutIndex, Sats>,
    txoutindex_to_outputtype: &BytesVec<TxOutIndex, OutputType>,
    txoutindex_to_typeindex: &BytesVec<TxOutIndex, TypeIndex>,
    ir: &IndexerReaders,
) -> OutputsResult {
    let (transacted, received_data) = (first_txoutindex..first_txoutindex + output_count)
        .into_par_iter()
        .map(|i| {
            let txoutindex = TxOutIndex::from(i);
            let local_idx = i - first_txoutindex;
            let _txindex = txoutindex_to_txindex[local_idx];

            let value = txoutindex_to_value.read_unwrap(txoutindex, &ir.txoutindex_to_value);
            let output_type =
                txoutindex_to_outputtype.read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

            // Non-address outputs don't need typeindex
            if output_type.is_not_address() {
                return (value, output_type, None);
            }

            let typeindex =
                txoutindex_to_typeindex.read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

            (value, output_type, Some((typeindex, value)))
        })
        .fold(
            || (Transacted::default(), AddressTypeToVec::default()),
            |(mut transacted, mut received_data), (value, output_type, addr_data)| {
                transacted.iterate(value, output_type);

                if let Some((typeindex, value)) = addr_data {
                    received_data
                        .get_mut(output_type)
                        .unwrap()
                        .push((typeindex, value));
                }

                (transacted, received_data)
            },
        )
        .reduce(
            || (Transacted::default(), AddressTypeToVec::default()),
            |(t1, r1), (t2, r2)| (t1 + t2, r1.merge(r2)),
        );

    OutputsResult {
        transacted,
        received_data,
    }
}

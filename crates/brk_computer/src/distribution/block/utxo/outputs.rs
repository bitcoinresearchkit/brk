use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::{FundedAddrData, Sats, TxIndex, TypeIndex};
use rayon::prelude::*;
use smallvec::SmallVec;

use crate::distribution::{
    addr::{AddrTypeToTypeIndexMap, AddrTypeToVec, AddrsDataVecs, AnyAddrIndexesVecs},
    compute::{TxOutData, VecsReaders},
    state::Transacted,
};

use super::super::{
    cache::{AddrCache, load_uncached_addr_data},
    cohort::WithAddrDataSource,
};

/// Result of processing outputs for a block.
pub struct OutputsResult {
    /// Aggregated supply transacted in this block.
    pub transacted: Transacted,
    /// Per-address-type received data: (type_index, value) for each address.
    pub received_data: AddrTypeToVec<(TypeIndex, Sats)>,
    /// Address data looked up during processing, keyed by (addr_type, type_index).
    pub addr_data: AddrTypeToTypeIndexMap<WithAddrDataSource<FundedAddrData>>,
    /// Transaction indexes per address for tx_count tracking.
    pub tx_index_vecs: AddrTypeToTypeIndexMap<SmallVec<[TxIndex; 4]>>,
}

/// Process outputs (new UTXOs) for a block.
///
/// For each output:
/// 1. Read pre-collected value, output type, and type_index
/// 2. Accumulate into Transacted by type and amount
/// 3. Look up address data if output is an address type
/// 4. Track address-specific data for address cohort processing
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_outputs(
    txout_index_to_tx_index: &[TxIndex],
    txout_data_vec: &[TxOutData],
    first_addr_indexes: &ByAddrType<TypeIndex>,
    cache: &AddrCache,
    vr: &VecsReaders,
    any_addr_indexes: &AnyAddrIndexesVecs,
    addrs_data: &AddrsDataVecs,
) -> Result<OutputsResult> {
    let output_count = txout_data_vec.len();

    // Phase 1: Addr lookups (mmap reads) — parallel for large blocks, sequential for small
    let map_fn = |local_idx: usize| -> Result<_> {
        let txout_data = &txout_data_vec[local_idx];
        let value = txout_data.value;
        let output_type = txout_data.output_type;

        if output_type.is_not_addr() {
            return Ok((value, output_type, None));
        }

        let type_index = txout_data.type_index;
        let tx_index = txout_index_to_tx_index[local_idx];

        let addr_data_opt = load_uncached_addr_data(
            output_type,
            type_index,
            first_addr_indexes,
            cache,
            vr,
            any_addr_indexes,
            addrs_data,
        )?;

        Ok((
            value,
            output_type,
            Some((type_index, tx_index, value, addr_data_opt)),
        ))
    };

    let items: Vec<_> = if output_count < 128 {
        (0..output_count).map(map_fn).collect::<Result<Vec<_>>>()?
    } else {
        (0..output_count)
            .into_par_iter()
            .map(map_fn)
            .collect::<Result<Vec<_>>>()?
    };

    // Phase 2: Sequential accumulation
    let estimated_per_type = (output_count / 8).max(8);
    let mut transacted = Transacted::default();
    let mut received_data = AddrTypeToVec::with_capacity(estimated_per_type);
    let mut addr_data = AddrTypeToTypeIndexMap::<WithAddrDataSource<FundedAddrData>>::with_capacity(
        estimated_per_type,
    );
    let mut tx_index_vecs =
        AddrTypeToTypeIndexMap::<SmallVec<[TxIndex; 4]>>::with_capacity(estimated_per_type);

    for (value, output_type, addr_info) in items {
        transacted.iterate(value, output_type);

        if let Some((type_index, tx_index, value, addr_data_opt)) = addr_info {
            received_data
                .get_mut(output_type)
                .unwrap()
                .push((type_index, value));

            if let Some(source) = addr_data_opt {
                addr_data.insert_for_type(output_type, type_index, source);
            }

            tx_index_vecs
                .get_mut(output_type)
                .unwrap()
                .entry(type_index)
                .or_default()
                .push(tx_index);
        }
    }

    Ok(OutputsResult {
        transacted,
        received_data,
        addr_data,
        tx_index_vecs,
    })
}

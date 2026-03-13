use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_types::{FundedAddressData, Sats, TxIndex, TypeIndex};
use rayon::prelude::*;
use smallvec::SmallVec;

use crate::distribution::{
    address::{
        AddressTypeToTypeIndexMap, AddressTypeToVec, AddressesDataVecs, AnyAddressIndexesVecs,
    },
    compute::{TxOutData, VecsReaders},
    state::Transacted,
};

use super::super::{
    cache::{AddressCache, load_uncached_address_data},
    cohort::WithAddressDataSource,
};

/// Result of processing outputs for a block.
pub struct OutputsResult {
    /// Aggregated supply transacted in this block.
    pub transacted: Transacted,
    /// Per-address-type received data: (type_index, value) for each address.
    pub received_data: AddressTypeToVec<(TypeIndex, Sats)>,
    /// Address data looked up during processing, keyed by (address_type, type_index).
    pub address_data: AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
    /// Transaction indexes per address for tx_count tracking.
    pub tx_index_vecs: AddressTypeToTypeIndexMap<SmallVec<[TxIndex; 4]>>,
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
    first_address_indexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> Result<OutputsResult> {
    let output_count = txout_data_vec.len();

    // Phase 1: Address lookups (mmap reads) — parallel for large blocks, sequential for small
    let map_fn = |local_idx: usize| -> Result<_> {
        let txout_data = &txout_data_vec[local_idx];
        let value = txout_data.value;
        let output_type = txout_data.output_type;

        if output_type.is_not_address() {
            return Ok((value, output_type, None));
        }

        let type_index = txout_data.type_index;
        let tx_index = txout_index_to_tx_index[local_idx];

        let addr_data_opt = load_uncached_address_data(
            output_type,
            type_index,
            first_address_indexes,
            cache,
            vr,
            any_address_indexes,
            addresses_data,
        )?;

        Ok((
            value,
            output_type,
            Some((type_index, tx_index, value, addr_data_opt)),
        ))
    };

    let items: Vec<_> = if output_count < 128 {
        (0..output_count)
            .map(map_fn)
            .collect::<Result<Vec<_>>>()?
    } else {
        (0..output_count)
            .into_par_iter()
            .map(map_fn)
            .collect::<Result<Vec<_>>>()?
    };

    // Phase 2: Sequential accumulation
    let estimated_per_type = (output_count / 8).max(8);
    let mut transacted = Transacted::default();
    let mut received_data = AddressTypeToVec::with_capacity(estimated_per_type);
    let mut address_data =
        AddressTypeToTypeIndexMap::<WithAddressDataSource<FundedAddressData>>::with_capacity(
            estimated_per_type,
        );
    let mut tx_index_vecs =
        AddressTypeToTypeIndexMap::<SmallVec<[TxIndex; 4]>>::with_capacity(estimated_per_type);

    for (value, output_type, addr_info) in items {
        transacted.iterate(value, output_type);

        if let Some((type_index, tx_index, value, addr_data_opt)) = addr_info {
            received_data
                .get_mut(output_type)
                .unwrap()
                .push((type_index, value));

            if let Some(addr_data) = addr_data_opt {
                address_data.insert_for_type(output_type, type_index, addr_data);
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
        address_data,
        tx_index_vecs,
    })
}

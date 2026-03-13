use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_types::{FundedAddressData, Height, OutputType, Sats, TxIndex, TypeIndex};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::distribution::{
    address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs},
    compute::VecsReaders,
    state::Transacted,
};

use crate::distribution::address::HeightToAddressTypeToVec;

use super::super::{
    cache::{AddressCache, load_uncached_address_data},
    cohort::WithAddressDataSource,
};

/// Result of processing inputs for a block.
pub struct InputsResult {
    /// Map from UTXO creation height -> aggregated sent supply.
    pub height_to_sent: FxHashMap<Height, Transacted>,
    /// Per-height, per-address-type sent data: (type_index, value) for each address.
    pub sent_data: HeightToAddressTypeToVec<(TypeIndex, Sats)>,
    /// Address data looked up during processing, keyed by (address_type, type_index).
    pub address_data: AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
    /// Transaction indexes per address for tx_count tracking.
    pub tx_index_vecs: AddressTypeToTypeIndexMap<SmallVec<[TxIndex; 4]>>,
}

/// Process inputs (spent UTXOs) for a block.
///
/// For each input:
/// 1. Use pre-collected outpoint (from reusable iterator, avoids PcoVec re-decompression)
/// 2. Resolve outpoint to txout_index
/// 3. Get the creation height from txout_index_to_height map
/// 4. Read value and type from the referenced output (random access via mmap)
/// 5. Look up address data if input references an address type
/// 6. Accumulate into height_to_sent map
/// 7. Track address-specific data for address cohort processing
///
/// Uses parallel reads followed by sequential accumulation to avoid
/// expensive merge overhead from rayon's fold/reduce pattern.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_inputs(
    input_count: usize,
    txin_index_to_tx_index: &[TxIndex],
    txin_index_to_value: &[Sats],
    txin_index_to_output_type: &[OutputType],
    txin_index_to_type_index: &[TypeIndex],
    txin_index_to_prev_height: &[Height],
    first_address_indexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> Result<InputsResult> {
    let map_fn = |local_idx: usize| -> Result<_> {
        let tx_index = txin_index_to_tx_index[local_idx];

        let prev_height = txin_index_to_prev_height[local_idx];
        let value = txin_index_to_value[local_idx];
        let input_type = txin_index_to_output_type[local_idx];

        if input_type.is_not_address() {
            return Ok((prev_height, value, input_type, None));
        }

        let type_index = txin_index_to_type_index[local_idx];

        // Look up address data
        let addr_data_opt = load_uncached_address_data(
            input_type,
            type_index,
            first_address_indexes,
            cache,
            vr,
            any_address_indexes,
            addresses_data,
        )?;

        Ok((
            prev_height,
            value,
            input_type,
            Some((type_index, tx_index, value, addr_data_opt)),
        ))
    };

    let items: Vec<_> = if input_count < 128 {
        (0..input_count)
            .map(map_fn)
            .collect::<Result<Vec<_>>>()?
    } else {
        (0..input_count)
            .into_par_iter()
            .map(map_fn)
            .collect::<Result<Vec<_>>>()?
    };

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
        AddressTypeToTypeIndexMap::<WithAddressDataSource<FundedAddressData>>::with_capacity(
            estimated_per_type,
        );
    let mut tx_index_vecs =
        AddressTypeToTypeIndexMap::<SmallVec<[TxIndex; 4]>>::with_capacity(estimated_per_type);

    for (prev_height, value, output_type, addr_info) in items {
        height_to_sent
            .entry(prev_height)
            .or_default()
            .iterate(value, output_type);

        if let Some((type_index, tx_index, value, addr_data_opt)) = addr_info {
            sent_data
                .entry(prev_height)
                .or_default()
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

    Ok(InputsResult {
        height_to_sent,
        sent_data,
        address_data,
        tx_index_vecs,
    })
}

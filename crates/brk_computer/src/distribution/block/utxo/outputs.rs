use brk_cohort::ByAddressType;
use brk_types::{Sats, TxIndex, TypeIndex};

use crate::distribution::{
    address::{
        AddressTypeToTypeIndexMap, AddressTypeToVec, AddressesDataVecs, AnyAddressIndexesVecs,
    },
    compute::{TxOutData, VecsReaders},
    state::Transacted,
};

use super::super::{
    cache::{AddressCache, load_uncached_address_data},
    cohort::{LoadedAddressDataWithSource, TxIndexVec},
};

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
    txoutindex_to_txindex: &[TxIndex],
    txoutdata_vec: &[TxOutData],
    first_addressindexes: &ByAddressType<TypeIndex>,
    cache: &AddressCache,
    vr: &VecsReaders,
    any_address_indexes: &AnyAddressIndexesVecs,
    addresses_data: &AddressesDataVecs,
) -> OutputsResult {
    let output_count = txoutdata_vec.len();

    // Pre-allocate result structures
    let estimated_per_type = (output_count / 8).max(8);
    let mut transacted = Transacted::default();
    let mut received_data = AddressTypeToVec::with_capacity(estimated_per_type);
    let mut address_data =
        AddressTypeToTypeIndexMap::<LoadedAddressDataWithSource>::with_capacity(estimated_per_type);
    let mut txindex_vecs =
        AddressTypeToTypeIndexMap::<TxIndexVec>::with_capacity(estimated_per_type);

    // Single pass: read from pre-collected vecs and accumulate
    for (local_idx, txoutdata) in txoutdata_vec.iter().enumerate() {
        let txindex = txoutindex_to_txindex[local_idx];
        let value = txoutdata.value;
        let output_type = txoutdata.outputtype;

        transacted.iterate(value, output_type);

        if output_type.is_not_address() {
            continue;
        }

        let typeindex = txoutdata.typeindex;

        received_data
            .get_mut(output_type)
            .unwrap()
            .push((typeindex, value));

        let addr_data_opt = load_uncached_address_data(
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

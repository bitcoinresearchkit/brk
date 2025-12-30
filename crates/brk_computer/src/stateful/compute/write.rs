use std::time::Instant;

use brk_error::Result;
use brk_types::Height;
use log::info;
use rayon::prelude::*;
use vecdb::{AnyStoredVec, GenericStoredVec, Stamp};

use crate::stateful::{
    Vecs,
    block::{
        EmptyAddressDataWithSource, LoadedAddressDataWithSource, process_empty_addresses,
        process_loaded_addresses,
    },
    state::BlockState,
};

use super::super::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};

/// Process address updates from caches.
///
/// Applies all accumulated address changes to storage structures:
/// - Processes empty address transitions
/// - Processes loaded address transitions
/// - Updates address indexes
///
/// Call this before `flush()` to prepare data for writing.
pub fn process_address_updates(
    addresses_data: &mut AddressesDataVecs,
    address_indexes: &mut AnyAddressIndexesVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    loaded_updates: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
) -> Result<()> {
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let loaded_result = process_loaded_addresses(addresses_data, loaded_updates)?;
    let all_updates = empty_result.merge(loaded_result);

    for (address_type, sorted) in all_updates.into_sorted_iter() {
        for (typeindex, any_index) in sorted {
            address_indexes.update_or_push(address_type, typeindex, any_index)?;
        }
    }

    Ok(())
}

/// Flush checkpoint to disk (pure I/O, no processing).
///
/// Writes all accumulated data in parallel:
/// - Cohort stateful vectors (parallel internally)
/// - Height-indexed vectors
/// - Address indexes and data
/// - Transaction output index mappings
/// - Chain state
///
/// Set `with_changes=true` near chain tip to enable rollback support.
pub fn write(
    vecs: &mut Vecs,
    height: Height,
    chain_state: &[BlockState],
    with_changes: bool,
) -> Result<()> {
    info!("Writing to disk...");

    let i = Instant::now();

    let stamp = Stamp::from(height);

    // Prepare chain_state before parallel write
    vecs.chain_state.truncate_if_needed(Height::ZERO)?;
    for block_state in chain_state {
        vecs.chain_state.push(block_state.supply.clone());
    }

    vecs.any_address_indexes
        .par_iter_mut()
        .chain(vecs.addresses_data.par_iter_mut())
        .chain(vecs.addresstype_to_height_to_addr_count.par_iter_mut())
        .chain(
            vecs.addresstype_to_height_to_empty_addr_count
                .par_iter_mut(),
        )
        .chain(rayon::iter::once(
            &mut vecs.chain_state as &mut dyn AnyStoredVec,
        ))
        .chain(rayon::iter::once(
            &mut vecs.height_to_unspendable_supply as &mut dyn AnyStoredVec,
        ))
        .chain(rayon::iter::once(
            &mut vecs.height_to_opreturn_supply as &mut dyn AnyStoredVec,
        ))
        .chain(vecs.utxo_cohorts.par_iter_vecs_mut())
        .chain(vecs.address_cohorts.par_iter_vecs_mut())
        .try_for_each(|v| v.any_stamped_write_maybe_with_changes(stamp, with_changes))?;

    // Commit states after vec writes
    let cleanup = with_changes;
    vecs.utxo_cohorts.commit_all_states(height, cleanup)?;
    vecs.address_cohorts.commit_all_states(height, cleanup)?;

    info!("Wrote in {:?}", i.elapsed());

    Ok(())
}

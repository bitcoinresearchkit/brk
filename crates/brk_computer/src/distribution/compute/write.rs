use std::time::Instant;

use brk_error::Result;
use brk_types::Height;
use rayon::prelude::*;
use tracing::info;
use vecdb::{AnyStoredVec, GenericStoredVec, Stamp};

use crate::distribution::{
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
    info!("Processing address updates...");

    let i = Instant::now();
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let loaded_result = process_loaded_addresses(addresses_data, loaded_updates)?;
    address_indexes.par_batch_update(empty_result, loaded_result)?;

    info!("Processed address updates in {:?}", i.elapsed());

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
    vecs.supply_state.truncate_if_needed(Height::ZERO)?;
    for block_state in chain_state {
        vecs.supply_state.push(block_state.supply.clone());
    }

    vecs.any_address_indexes
        .par_iter_mut()
        .chain(vecs.addresses_data.par_iter_mut())
        .chain(vecs.addr_count.par_iter_height_mut())
        .chain(vecs.empty_addr_count.par_iter_height_mut())
        .chain(vecs.address_activity.par_iter_height_mut())
        .chain(rayon::iter::once(
            &mut vecs.supply_state as &mut dyn AnyStoredVec,
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

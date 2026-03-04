use std::time::Instant;

use brk_error::Result;
use brk_types::{EmptyAddressData, FundedAddressData, Height};
use rayon::prelude::*;
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Stamp, VecIndex, WritableVec};

use crate::distribution::{
    Vecs,
    block::{WithAddressDataSource, process_empty_addresses, process_funded_addresses},
    state::BlockState,
};

use super::super::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};

/// Process address updates from caches.
///
/// Applies all accumulated address changes to storage structures:
/// - Processes empty address transitions
/// - Processes funded address transitions
/// - Updates address indexes
///
/// Call this before `flush()` to prepare data for writing.
pub(crate) fn process_address_updates(
    addresses_data: &mut AddressesDataVecs,
    address_indexes: &mut AnyAddressIndexesVecs,
    empty_updates: AddressTypeToTypeIndexMap<WithAddressDataSource<EmptyAddressData>>,
    funded_updates: AddressTypeToTypeIndexMap<WithAddressDataSource<FundedAddressData>>,
) -> Result<()> {
    info!("Processing address updates...");

    let i = Instant::now();
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let funded_result = process_funded_addresses(addresses_data, funded_updates)?;
    address_indexes.par_batch_update(empty_result, funded_result)?;

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
pub(crate) fn write(
    vecs: &mut Vecs,
    height: Height,
    chain_state: &[BlockState],
    min_supply_modified: Option<Height>,
    with_changes: bool,
) -> Result<()> {
    info!("Writing to disk...");

    let i = Instant::now();

    let stamp = Stamp::from(height);

    // Incremental supply_state write: only rewrite from the earliest modified height
    let supply_state_len = vecs.supply_state.len();
    let truncate_to =
        min_supply_modified.map_or(supply_state_len, |h| h.to_usize().min(supply_state_len));
    vecs.supply_state
        .truncate_if_needed(Height::from(truncate_to))?;
    for block_state in &chain_state[truncate_to..] {
        vecs.supply_state.push(block_state.supply);
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

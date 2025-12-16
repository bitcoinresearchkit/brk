//! State flushing logic for checkpoints.
//!
//! Handles periodic flushing of all stateful data to disk,
//! including cohort states, address data, and chain state.

use brk_error::Result;
use brk_types::{AnyAddressIndex, Height};
use log::info;
use vecdb::{Exit, Stamp};

use crate::stateful::process::{
    EmptyAddressDataWithSource, LoadedAddressDataWithSource, process_empty_addresses,
    process_loaded_addresses,
};

use super::super::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};
use super::super::cohorts::DynCohortVecs;

/// Flush all cohort stateful vectors.
pub fn flush_cohort_states(
    height: Height,
    utxo_vecs: &mut [&mut dyn DynCohortVecs],
    address_vecs: &mut [&mut dyn DynCohortVecs],
    exit: &Exit,
) -> Result<()> {
    for v in utxo_vecs.iter_mut() {
        v.safe_flush_stateful_vecs(height, exit)?;
    }
    for v in address_vecs.iter_mut() {
        v.safe_flush_stateful_vecs(height, exit)?;
    }
    Ok(())
}

/// Apply address index updates to the index storage.
fn apply_address_index_updates(
    address_indexes: &mut AnyAddressIndexesVecs,
    updates: AddressTypeToTypeIndexMap<AnyAddressIndex>,
) -> Result<()> {
    for (address_type, sorted) in updates.into_sorted_iter() {
        for (typeindex, any_index) in sorted {
            address_indexes.update_or_push(address_type, typeindex, any_index)?;
        }
    }
    Ok(())
}

/// Full state flush at a checkpoint.
///
/// This is the main entry point for checkpoint flushing:
/// 1. Flush cohort stateful vectors
/// 2. Process address data updates (empty and loaded)
/// 3. Update address indexes
/// 4. Stamped flush address indexes and data
/// 5. Flush chain state
#[allow(clippy::too_many_arguments)]
pub fn flush_checkpoint(
    height: Height,
    utxo_vecs: &mut [&mut dyn DynCohortVecs],
    address_vecs: &mut [&mut dyn DynCohortVecs],
    address_indexes: &mut AnyAddressIndexesVecs,
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    loaded_updates: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    with_changes: bool,
    exit: &Exit,
) -> Result<()> {
    info!("Flushing at height {}...", height);

    // 1. Flush cohort states
    flush_cohort_states(height, utxo_vecs, address_vecs, exit)?;

    // 2. Process address updates - empty first, then loaded
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let loaded_result = process_loaded_addresses(addresses_data, loaded_updates)?;
    let all_updates = empty_result.merge(loaded_result);

    // 3. Apply index updates
    apply_address_index_updates(address_indexes, all_updates)?;

    // 4. Stamped flush
    let stamp = Stamp::from(height);
    address_indexes.flush(stamp, with_changes)?;
    addresses_data.flush(stamp, with_changes)?;

    Ok(())
}

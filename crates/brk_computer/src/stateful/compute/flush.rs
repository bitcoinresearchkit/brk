//! State flushing logic for checkpoints.
//!
//! Handles periodic flushing of all stateful data to disk,
//! including address data and chain state.

use std::mem;

use brk_error::Result;
use brk_types::{AnyAddressIndex, Height};
use log::info;
use vecdb::{AnyStoredVec, Exit, GenericStoredVec, Stamp};

use crate::{
    stateful::{
        Vecs,
        process::{
            EmptyAddressDataWithSource, LoadedAddressDataWithSource, process_empty_addresses,
            process_loaded_addresses,
        },
    },
    states::BlockState,
};

use super::super::address::{AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs};

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

/// Flush checkpoint to disk.
///
/// Flushes all accumulated data including:
/// - Cohort stateful vectors
/// - Height-indexed vectors
/// - Address data caches (loaded and empty)
/// - Chain state (synced from in-memory to persisted)
#[allow(clippy::too_many_arguments)]
pub fn flush(
    vecs: &mut Vecs,
    height: Height,
    chain_state: &[BlockState],
    loaded_cache: &mut AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    empty_cache: &mut AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    exit: &Exit,
) -> Result<()> {
    info!("Flushing checkpoint at height {}...", height);

    let _lock = exit.lock();

    // Flush cohort states (separate + aggregate)
    vecs.utxo_cohorts.safe_flush_stateful_vecs(height, exit)?;
    vecs.address_cohorts
        .safe_flush_stateful_vecs(height, exit)?;

    // Flush height-indexed vectors
    vecs.height_to_unspendable_supply.safe_write(exit)?;
    vecs.height_to_opreturn_supply.safe_write(exit)?;
    vecs.addresstype_to_height_to_addr_count.safe_flush(exit)?;
    vecs.addresstype_to_height_to_empty_addr_count
        .safe_flush(exit)?;

    // Process and flush address data updates
    let empty_updates = mem::take(empty_cache);
    let loaded_updates = mem::take(loaded_cache);
    flush_address_data(
        height,
        &mut vecs.any_address_indexes,
        &mut vecs.addresses_data,
        empty_updates,
        loaded_updates,
        true,
    )?;

    // Flush txoutindex_to_txinindex with stamp
    vecs.txoutindex_to_txinindex
        .stamped_flush_with_changes(height.into())?;

    // Sync in-memory chain_state to persisted and flush
    vecs.chain_state.truncate_if_needed(Height::ZERO)?;
    for block_state in chain_state {
        vecs.chain_state.push(block_state.supply.clone());
    }
    vecs.chain_state.stamped_flush_with_changes(height.into())?;

    Ok(())
}

/// Flush address data at a checkpoint.
///
/// Note: Cohort states are flushed separately before this is called.
///
/// 1. Process address data updates (empty and loaded)
/// 2. Update address indexes
/// 3. Stamped flush address indexes and data
fn flush_address_data(
    height: Height,
    address_indexes: &mut AnyAddressIndexesVecs,
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    loaded_updates: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    with_changes: bool,
) -> Result<()> {
    info!("Flushing address data at height {}...", height);

    // 1. Process address updates - empty first, then loaded
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let loaded_result = process_loaded_addresses(addresses_data, loaded_updates)?;
    let all_updates = empty_result.merge(loaded_result);

    // 2. Apply index updates
    apply_address_index_updates(address_indexes, all_updates)?;

    // 3. Stamped flush
    let stamp = Stamp::from(height);
    address_indexes.flush(stamp, with_changes)?;
    addresses_data.flush(stamp, with_changes)?;

    Ok(())
}

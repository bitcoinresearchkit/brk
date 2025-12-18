//! State flushing logic for checkpoints.
//!
//! Separates processing (mutations) from flushing (I/O):
//! - `process_address_updates`: applies cached address changes to storage
//! - `flush`: writes all data to disk

use std::time::Instant;

use brk_error::Result;
use brk_types::Height;
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
    let t0 = Instant::now();

    // Process address data transitions
    let empty_result = process_empty_addresses(addresses_data, empty_updates)?;
    let t1 = Instant::now();

    let loaded_result = process_loaded_addresses(addresses_data, loaded_updates)?;
    let t2 = Instant::now();

    let all_updates = empty_result.merge(loaded_result);
    let t3 = Instant::now();

    // Apply index updates
    for (address_type, sorted) in all_updates.into_sorted_iter() {
        for (typeindex, any_index) in sorted {
            address_indexes.update_or_push(address_type, typeindex, any_index)?;
        }
    }
    let t4 = Instant::now();

    info!(
        "process_address_updates: empty={:?} loaded={:?} merge={:?} indexes={:?} total={:?}",
        t1 - t0,
        t2 - t1,
        t3 - t2,
        t4 - t3,
        t4 - t0
    );

    Ok(())
}

/// Flush checkpoint to disk (pure I/O, no processing).
///
/// Writes all accumulated data:
/// - Cohort stateful vectors
/// - Height-indexed vectors
/// - Address indexes and data
/// - Transaction output index mappings
/// - Chain state
pub fn flush(
    vecs: &mut Vecs,
    height: Height,
    chain_state: &[BlockState],
    exit: &Exit,
) -> Result<()> {
    info!("Flushing at height {}...", height);

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

    // Flush address data
    let stamp = Stamp::from(height);
    vecs.any_address_indexes.flush(stamp, true)?;
    vecs.addresses_data.flush(stamp, true)?;

    // Flush txoutindex_to_txinindex
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

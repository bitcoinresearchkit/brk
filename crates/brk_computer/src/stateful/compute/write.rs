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
/// - Address indexes and data (parallel)
/// - Transaction output index mappings (parallel)
/// - Chain state
///
/// Set `with_changes=true` near chain tip to enable rollback support.
pub fn write(
    vecs: &mut Vecs,
    height: Height,
    chain_state: &[BlockState],
    with_changes: bool,
    exit: &Exit,
) -> Result<()> {
    let t0 = Instant::now();

    // Flush cohort states (separate + aggregate)
    vecs.utxo_cohorts.safe_flush_stateful_vecs(height, exit)?;
    let t1 = Instant::now();

    vecs.address_cohorts
        .safe_flush_stateful_vecs(height, exit)?;
    let t2 = Instant::now();

    // Flush height-indexed vectors
    vecs.height_to_unspendable_supply.safe_write(exit)?;
    vecs.height_to_opreturn_supply.safe_write(exit)?;
    vecs.addresstype_to_height_to_addr_count.safe_flush(exit)?;
    vecs.addresstype_to_height_to_empty_addr_count
        .safe_flush(exit)?;
    let t3 = Instant::now();

    // Flush large vecs in parallel
    let stamp = Stamp::from(height);
    let any_address_indexes = &mut vecs.any_address_indexes;
    let addresses_data = &mut vecs.addresses_data;
    let txoutindex_to_txinindex = &mut vecs.txoutindex_to_txinindex;

    let ((addr_result, addr_idx_time, addr_data_time), (txout_result, txout_time)) = rayon::join(
        || {
            let t0 = Instant::now();
            let r1 = any_address_indexes.write(stamp, with_changes);
            let t1 = Instant::now();
            let r2 = addresses_data.write(stamp, with_changes);
            let t2 = Instant::now();
            let r = r1.and(r2);
            (r, t1 - t0, t2 - t1)
        },
        || {
            let t = Instant::now();
            let r = txoutindex_to_txinindex.stamped_write_maybe_with_changes(stamp, with_changes);
            (r, t.elapsed())
        },
    );
    addr_result?;
    txout_result?;
    let t4 = Instant::now();
    info!(
        "  parallel breakdown: addr_idx={:?} addr_data={:?} txout={:?}",
        addr_idx_time, addr_data_time, txout_time
    );

    // Sync in-memory chain_state to persisted and flush
    vecs.chain_state.truncate_if_needed(Height::ZERO)?;
    for block_state in chain_state {
        vecs.chain_state.push(block_state.supply.clone());
    }
    vecs.chain_state
        .stamped_write_maybe_with_changes(stamp, with_changes)?;
    let t5 = Instant::now();

    info!(
        "flush: utxo={:?} addr={:?} height={:?} parallel={:?} chain={:?} total={:?} (with_changes={})",
        t1 - t0,
        t2 - t1,
        t3 - t2,
        t4 - t3,
        t5 - t4,
        t5 - t0,
        with_changes
    );

    Ok(())
}

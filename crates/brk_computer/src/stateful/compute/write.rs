//! State flushing logic for checkpoints.
//!
//! Separates processing (mutations) from flushing (I/O):
//! - `process_address_updates`: applies cached address changes to storage
//! - `flush`: writes all data to disk

use std::time::Instant;

use brk_error::Result;
use brk_types::Height;
use log::info;
use vecdb::{AnyStoredVec, GenericStoredVec, Stamp};

use crate::stateful::{
    Vecs,
    process::{
        EmptyAddressDataWithSource, LoadedAddressDataWithSource, process_empty_addresses,
        process_loaded_addresses,
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
) -> Result<()> {
    info!("Writing to disk...");
    let i = Instant::now();

    // Flush cohort states (separate + aggregate)
    vecs.utxo_cohorts.write_stateful_vecs(height)?;
    vecs.address_cohorts.write_stateful_vecs(height)?;

    // Flush height-indexed vectors
    vecs.height_to_unspendable_supply.write()?;
    vecs.height_to_opreturn_supply.write()?;
    vecs.addresstype_to_height_to_addr_count.write()?;
    vecs.addresstype_to_height_to_empty_addr_count.write()?;

    // Flush large vecs in parallel
    let stamp = Stamp::from(height);
    let any_address_indexes = &mut vecs.any_address_indexes;
    let addresses_data = &mut vecs.addresses_data;
    let txoutindex_to_txinindex = &mut vecs.txoutindex_to_txinindex;

    let (addr_result, txout_result) = rayon::join(
        || {
            any_address_indexes
                .write(stamp, with_changes)
                .and(addresses_data.write(stamp, with_changes))
        },
        || txoutindex_to_txinindex.stamped_write_maybe_with_changes(stamp, with_changes),
    );
    addr_result?;
    txout_result?;

    // Sync in-memory chain_state to persisted and flush
    vecs.chain_state.truncate_if_needed(Height::ZERO)?;
    for block_state in chain_state {
        vecs.chain_state.push(block_state.supply.clone());
    }
    vecs.chain_state
        .stamped_write_maybe_with_changes(stamp, with_changes)?;

    info!("Wrote in {:?}", i.elapsed());

    Ok(())
}

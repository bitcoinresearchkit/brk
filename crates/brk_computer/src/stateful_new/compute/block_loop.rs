//! Main block processing loop.
//!
//! Iterates through blocks and processes each one:
//! 1. Reset per-block state values
//! 2. Tick-tock age transitions
//! 3. Process outputs (receive) in parallel
//! 4. Process inputs (send) in parallel
//! 5. Push to height-indexed vectors
//! 6. Periodically flush checkpoints

use std::thread;

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_indexer::Indexer;
use brk_types::{DateIndex, Height, OutputType, Sats};
use log::info;
use vecdb::{Exit, GenericStoredVec, IterableVec, VecIndex};

use crate::states::{BlockState, Transacted};
use crate::{chain, indexes, price};

use super::super::cohorts::{AddressCohorts, DynCohortVecs, UTXOCohorts};
use super::super::vecs::Vecs;
use super::{
    FLUSH_INTERVAL, IndexerReaders, build_txinindex_to_txindex, build_txoutindex_to_height_map,
    build_txoutindex_to_txindex, process_inputs, process_outputs,
};

/// BIP30 duplicate coinbase heights - must handle specially.
const BIP30_DUPLICATE_HEIGHT_1: u32 = 91_842;
const BIP30_DUPLICATE_HEIGHT_2: u32 = 91_880;
const BIP30_ORIGINAL_HEIGHT_1: u32 = 91_812;
const BIP30_ORIGINAL_HEIGHT_2: u32 = 91_722;

/// Process all blocks from starting_height to last_height.
#[allow(clippy::too_many_arguments)]
pub fn process_blocks(
    vecs: &mut Vecs,
    indexer: &Indexer,
    indexes: &indexes::Vecs,
    chain: &chain::Vecs,
    price: Option<&price::Vecs>,
    starting_height: Height,
    last_height: Height,
    chain_state: &mut Vec<BlockState>,
    exit: &Exit,
) -> Result<()> {
    if starting_height > last_height {
        return Ok(());
    }

    info!(
        "Processing blocks {} to {}...",
        starting_height, last_height
    );

    // Pre-compute iterators for fast access
    let mut height_to_first_txindex = indexes.height_to_first_txindex.boxed_iter();
    let mut height_to_tx_count = chain.height_to_tx_count.boxed_iter();
    let mut height_to_first_txoutindex = indexes.height_to_first_txoutindex.boxed_iter();
    let mut height_to_output_count = chain.height_to_output_count.boxed_iter();
    let mut height_to_first_txinindex = indexes.height_to_first_txinindex.boxed_iter();
    let mut height_to_input_count = chain.height_to_input_count.boxed_iter();
    let mut height_to_timestamp = chain.height_to_timestamp.boxed_iter();
    let mut height_to_unclaimed_rewards = chain.height_to_unclaimed_reward.boxed_iter();
    let mut height_to_date = indexes.height_to_date.boxed_iter();
    let mut dateindex_to_first_height = indexes.dateindex_to_first_height.boxed_iter();
    let mut dateindex_to_height_count = indexes.dateindex_to_height_count.boxed_iter();
    let mut txindex_to_output_count = chain.txindex_to_output_count.boxed_iter();
    let mut txindex_to_input_count = chain.txindex_to_input_count.boxed_iter();

    let mut height_to_price = price.map(|p| p.height_to_close.boxed_iter());
    let mut dateindex_to_price = price.map(|p| p.dateindex_to_close.boxed_iter());

    // Build txoutindex -> height map for input processing
    let txoutindex_to_height = build_txoutindex_to_height_map(&indexes.height_to_first_txoutindex);

    // Create readers for parallel data access
    let ir = IndexerReaders::new(indexer);

    // Track running totals
    let mut unspendable_supply = Sats::ZERO;
    let mut opreturn_supply = Sats::ZERO;
    let mut addresstype_to_addr_count = ByAddressType::<u64>::default();
    let mut addresstype_to_empty_addr_count = ByAddressType::<u64>::default();

    // Recover initial values if resuming
    if starting_height > Height::ZERO {
        let prev_height = starting_height.decremented().unwrap();
        unspendable_supply = vecs
            .height_to_unspendable_supply
            .get(prev_height)
            .unwrap_or_default();
        opreturn_supply = vecs
            .height_to_opreturn_supply
            .get(prev_height)
            .unwrap_or_default();
    }

    // Main block iteration
    for height in starting_height.to_usize()..=last_height.to_usize() {
        let height = Height::from(height);

        if height.to_usize() % 10000 == 0 {
            info!("Processing chain at {}...", height);
        }

        // Get block metadata
        let first_txindex = height_to_first_txindex.get_unwrap(height);
        let tx_count = u64::from(height_to_tx_count.get_unwrap(height));
        let first_txoutindex = height_to_first_txoutindex.get_unwrap(height).to_usize();
        let output_count = u64::from(height_to_output_count.get_unwrap(height)) as usize;
        let first_txinindex = height_to_first_txinindex.get_unwrap(height).to_usize();
        let input_count = u64::from(height_to_input_count.get_unwrap(height)) as usize;
        let timestamp = height_to_timestamp.get_unwrap(height);
        let block_price = height_to_price.as_mut().map(|v| v.get_unwrap(height));

        // Build txindex mappings for this block
        let txoutindex_to_txindex =
            build_txoutindex_to_txindex(first_txindex, tx_count, &mut txindex_to_output_count);
        let txinindex_to_txindex =
            build_txinindex_to_txindex(first_txindex, tx_count, &mut txindex_to_input_count);

        // Reset per-block values for all separate cohorts
        reset_block_values(&mut vecs.utxo_cohorts, &mut vecs.address_cohorts);

        // Process outputs and inputs in parallel with tick-tock
        let (outputs_result, inputs_result) = thread::scope(|scope| {
            // Tick-tock age transitions in background
            scope.spawn(|| {
                vecs.utxo_cohorts
                    .tick_tock_next_block(chain_state, timestamp);
            });

            // Process outputs (receive)
            let outputs_result = process_outputs(
                first_txoutindex,
                output_count,
                &txoutindex_to_txindex,
                &indexer.vecs.txoutindex_to_value,
                &indexer.vecs.txoutindex_to_outputtype,
                &indexer.vecs.txoutindex_to_typeindex,
                &ir,
            );

            // Process inputs (send) - skip coinbase input
            let inputs_result = if input_count > 1 {
                process_inputs(
                    first_txinindex + 1, // Skip coinbase
                    input_count - 1,
                    &txinindex_to_txindex[1..], // Skip coinbase
                    &indexer.vecs.txinindex_to_outpoint,
                    &indexer.vecs.txindex_to_first_txoutindex,
                    &indexer.vecs.txoutindex_to_value,
                    &indexer.vecs.txoutindex_to_outputtype,
                    &indexer.vecs.txoutindex_to_typeindex,
                    &txoutindex_to_height,
                    &ir,
                )
            } else {
                super::InputsResult {
                    height_to_sent: Default::default(),
                    sent_data: Default::default(),
                }
            };

            (outputs_result, inputs_result)
        });

        let mut transacted = outputs_result.transacted;
        let mut height_to_sent = inputs_result.height_to_sent;

        // Update supply tracking
        unspendable_supply += transacted.by_type.unspendable.opreturn.value
            + height_to_unclaimed_rewards.get_unwrap(height);
        opreturn_supply += transacted.by_type.unspendable.opreturn.value;

        // Handle special cases
        if height == Height::ZERO {
            // Genesis block - reset transacted, add 50 BTC to unspendable
            transacted = Transacted::default();
            unspendable_supply += Sats::FIFTY_BTC;
        } else if height == Height::new(BIP30_DUPLICATE_HEIGHT_1)
            || height == Height::new(BIP30_DUPLICATE_HEIGHT_2)
        {
            // BIP30: Add 50 BTC to spent from original height
            let original_height = if height == Height::new(BIP30_DUPLICATE_HEIGHT_1) {
                Height::new(BIP30_ORIGINAL_HEIGHT_1)
            } else {
                Height::new(BIP30_ORIGINAL_HEIGHT_2)
            };
            height_to_sent
                .entry(original_height)
                .or_default()
                .iterate(Sats::FIFTY_BTC, OutputType::P2PK65);
        }

        // Push current block state before processing cohort updates
        chain_state.push(BlockState {
            supply: transacted.spendable_supply.clone(),
            price: block_price,
            timestamp,
        });

        // Update UTXO cohorts
        vecs.utxo_cohorts.receive(transacted, height, block_price);
        vecs.utxo_cohorts.send(height_to_sent, chain_state);

        // Push to height-indexed vectors
        vecs.height_to_unspendable_supply
            .truncate_push(height, unspendable_supply)?;
        vecs.height_to_opreturn_supply
            .truncate_push(height, opreturn_supply)?;
        vecs.addresstype_to_height_to_addr_count
            .truncate_push(height, &addresstype_to_addr_count)?;
        vecs.addresstype_to_height_to_empty_addr_count
            .truncate_push(height, &addresstype_to_empty_addr_count)?;

        // Get date info for unrealized state computation
        let date = height_to_date.get_unwrap(height);
        let dateindex = DateIndex::try_from(date).unwrap();
        let date_first_height = dateindex_to_first_height.get_unwrap(dateindex);
        let date_height_count = dateindex_to_height_count.get_unwrap(dateindex);
        let is_date_last_height =
            date_first_height + Height::from(date_height_count).decremented().unwrap() == height;
        let date_price = dateindex_to_price
            .as_mut()
            .map(|v| is_date_last_height.then(|| v.get_unwrap(dateindex)));
        let dateindex_opt = is_date_last_height.then_some(dateindex);

        // Push cohort states and compute unrealized
        push_cohort_states(
            &mut vecs.utxo_cohorts,
            &mut vecs.address_cohorts,
            height,
            block_price,
            dateindex_opt,
            date_price,
        )?;

        // Periodic checkpoint flush
        if height != last_height
            && height != Height::ZERO
            && height.to_usize() % FLUSH_INTERVAL == 0
        {
            let _lock = exit.lock();
            flush_checkpoint(vecs, height, exit)?;
        }
    }

    // Final flush
    let _lock = exit.lock();
    flush_checkpoint(vecs, last_height, exit)?;

    Ok(())
}

/// Reset per-block values for all separate cohorts.
fn reset_block_values(utxo_cohorts: &mut UTXOCohorts, address_cohorts: &mut AddressCohorts) {
    utxo_cohorts.par_iter_separate_mut().for_each(|v| {
        if let Some(state) = v.state.as_mut() {
            state.reset_single_iteration_values();
        }
    });

    address_cohorts.par_iter_separate_mut().for_each(|v| {
        if let Some(state) = v.state.as_mut() {
            state.inner.reset_single_iteration_values();
        }
    });
}

/// Push cohort states to height-indexed vectors.
fn push_cohort_states(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    height: Height,
    height_price: Option<brk_types::Dollars>,
    dateindex: Option<DateIndex>,
    date_price: Option<Option<brk_types::Dollars>>,
) -> Result<()> {
    utxo_cohorts
        .par_iter_separate_mut()
        .map(|v| v as &mut dyn DynCohortVecs)
        .chain(
            address_cohorts
                .par_iter_separate_mut()
                .map(|v| v as &mut dyn DynCohortVecs),
        )
        .try_for_each(|v| {
            v.truncate_push(height)?;
            v.compute_then_truncate_push_unrealized_states(
                height,
                height_price,
                dateindex,
                date_price,
            )
        })?;

    Ok(())
}

/// Flush checkpoint to disk.
fn flush_checkpoint(vecs: &mut Vecs, height: Height, exit: &Exit) -> Result<()> {
    info!("Flushing checkpoint at height {}...", height);

    // Flush cohort states
    vecs.utxo_cohorts.safe_flush_stateful_vecs(height, exit)?;
    vecs.address_cohorts.safe_flush_stateful_vecs(height, exit)?;

    // Flush height-indexed vectors
    vecs.height_to_unspendable_supply.safe_write(exit)?;
    vecs.height_to_opreturn_supply.safe_write(exit)?;
    vecs.addresstype_to_height_to_addr_count.safe_flush(exit)?;
    vecs.addresstype_to_height_to_empty_addr_count
        .safe_flush(exit)?;

    // Flush chain state with stamp
    vecs.chain_state.safe_write_with_stamp(height.into(), exit)?;

    Ok(())
}

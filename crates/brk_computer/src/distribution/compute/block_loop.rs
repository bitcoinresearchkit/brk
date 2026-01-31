use std::thread;

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CentsUnsigned, DateIndex, Dollars, Height, OutputType, Sats, TxIndex, TypeIndex};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use tracing::info;
use vecdb::{Exit, IterableVec, TypedVecIterator, VecIndex};

use crate::{
    blocks,
    distribution::{
        address::{AddressTypeToActivityCounts, AddressTypeToAddressCount},
        block::{
            AddressCache, InputsResult, process_inputs, process_outputs, process_received,
            process_sent,
        },
        compute::write::{process_address_updates, write},
        state::{BlockState, Transacted},
    },
    indexes, inputs, outputs, price, transactions,
};

use super::{
    super::{
        RangeMap,
        cohorts::{AddressCohorts, DynCohortVecs, UTXOCohorts},
        vecs::Vecs,
    },
    BIP30_DUPLICATE_HEIGHT_1, BIP30_DUPLICATE_HEIGHT_2, BIP30_ORIGINAL_HEIGHT_1,
    BIP30_ORIGINAL_HEIGHT_2, ComputeContext, FLUSH_INTERVAL, TxInIterators, TxOutIterators,
    VecsReaders, build_txinindex_to_txindex, build_txoutindex_to_txindex,
};

/// Process all blocks from starting_height to last_height.
#[allow(clippy::too_many_arguments)]
pub fn process_blocks(
    vecs: &mut Vecs,
    indexer: &Indexer,
    indexes: &indexes::Vecs,
    inputs: &inputs::Vecs,
    outputs: &outputs::Vecs,
    transactions: &transactions::Vecs,
    blocks: &blocks::Vecs,
    price: Option<&price::Vecs>,
    starting_height: Height,
    last_height: Height,
    chain_state: &mut Vec<BlockState>,
    exit: &Exit,
) -> Result<()> {
    // Create computation context with pre-computed vectors for thread-safe access
    let ctx = ComputeContext::new(starting_height, last_height, blocks, price);

    if ctx.starting_height > ctx.last_height {
        return Ok(());
    }

    // References to vectors using correct field paths
    // From indexer.vecs:
    let height_to_first_txindex = &indexer.vecs.transactions.first_txindex;
    let height_to_first_txoutindex = &indexer.vecs.outputs.first_txoutindex;
    let height_to_first_txinindex = &indexer.vecs.inputs.first_txinindex;

    // From transactions and inputs/outputs (via .height or .height.sum_cum.sum patterns):
    let height_to_tx_count = &transactions.count.tx_count.height;
    let height_to_output_count = &outputs.count.total_count.height.sum_cum.sum.0;
    let height_to_input_count = &inputs.count.height.sum_cum.sum.0;
    // From blocks:
    let height_to_timestamp = &blocks.time.timestamp_monotonic;
    let height_to_date = &blocks.time.date;
    let dateindex_to_first_height = &indexes.dateindex.first_height;
    let dateindex_to_height_count = &indexes.dateindex.height_count;
    let txindex_to_output_count = &indexes.txindex.output_count;
    let txindex_to_input_count = &indexes.txindex.input_count;

    // From price (optional) - use cents for computation:
    let height_to_price = price.map(|p| &p.cents.split.height.close);
    let dateindex_to_price = price.map(|p| &p.cents.split.dateindex.close);

    // Access pre-computed vectors from context for thread-safe access
    let height_to_price_vec = &ctx.height_to_price;
    let height_to_timestamp_vec = &ctx.height_to_timestamp;

    // Create iterators for sequential access
    let mut height_to_first_txindex_iter = height_to_first_txindex.into_iter();
    let mut height_to_first_txoutindex_iter = height_to_first_txoutindex.into_iter();
    let mut height_to_first_txinindex_iter = height_to_first_txinindex.into_iter();
    let mut height_to_tx_count_iter = height_to_tx_count.into_iter();
    let mut height_to_output_count_iter = height_to_output_count.into_iter();
    let mut height_to_input_count_iter = height_to_input_count.into_iter();
    let mut height_to_timestamp_iter = height_to_timestamp.into_iter();
    let mut height_to_date_iter = height_to_date.into_iter();
    let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
    let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();
    let mut txindex_to_output_count_iter = txindex_to_output_count.iter();
    let mut txindex_to_input_count_iter = txindex_to_input_count.iter();
    let mut height_to_price_iter = height_to_price.map(|v| v.into_iter());
    let mut dateindex_to_price_iter = dateindex_to_price.map(|v| v.into_iter());

    let mut vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);

    // Build txindex -> height lookup map for efficient prev_height computation
    let mut txindex_to_height: RangeMap<TxIndex, Height> = {
        let mut map = RangeMap::with_capacity(last_height.to_usize() + 1);
        for first_txindex in indexer.vecs.transactions.first_txindex.into_iter() {
            map.push(first_txindex);
        }
        map
    };

    // Create reusable iterators for sequential txout/txin reads (16KB buffered)
    let mut txout_iters = TxOutIterators::new(indexer);
    let mut txin_iters = TxInIterators::new(indexer, inputs, &mut txindex_to_height);

    // Create iterators for first address indexes per type
    let mut first_p2a_iter = indexer.vecs.addresses.first_p2aaddressindex.into_iter();
    let mut first_p2pk33_iter = indexer.vecs.addresses.first_p2pk33addressindex.into_iter();
    let mut first_p2pk65_iter = indexer.vecs.addresses.first_p2pk65addressindex.into_iter();
    let mut first_p2pkh_iter = indexer.vecs.addresses.first_p2pkhaddressindex.into_iter();
    let mut first_p2sh_iter = indexer.vecs.addresses.first_p2shaddressindex.into_iter();
    let mut first_p2tr_iter = indexer.vecs.addresses.first_p2traddressindex.into_iter();
    let mut first_p2wpkh_iter = indexer.vecs.addresses.first_p2wpkhaddressindex.into_iter();
    let mut first_p2wsh_iter = indexer.vecs.addresses.first_p2wshaddressindex.into_iter();

    // Track running totals - recover from previous height if resuming
    let (mut addr_counts, mut empty_addr_counts) = if starting_height > Height::ZERO {
        let addr_counts =
            AddressTypeToAddressCount::from((&vecs.addr_count.by_addresstype, starting_height));
        let empty_addr_counts = AddressTypeToAddressCount::from((
            &vecs.empty_addr_count.by_addresstype,
            starting_height,
        ));
        (addr_counts, empty_addr_counts)
    } else {
        (
            AddressTypeToAddressCount::default(),
            AddressTypeToAddressCount::default(),
        )
    };

    // Track activity counts - reset each block
    let mut activity_counts = AddressTypeToActivityCounts::default();

    let mut cache = AddressCache::new();

    // Main block iteration
    for height in starting_height.to_usize()..=last_height.to_usize() {
        let height = Height::from(height);

        info!("Processing chain at {}...", height);

        // Get block metadata
        let first_txindex = height_to_first_txindex_iter.get_unwrap(height);
        let tx_count = u64::from(height_to_tx_count_iter.get_unwrap(height));
        let first_txoutindex = height_to_first_txoutindex_iter
            .get_unwrap(height)
            .to_usize();
        let output_count = u64::from(height_to_output_count_iter.get_unwrap(height)) as usize;
        let first_txinindex = height_to_first_txinindex_iter.get_unwrap(height).to_usize();
        let input_count = u64::from(height_to_input_count_iter.get_unwrap(height)) as usize;
        let timestamp = height_to_timestamp_iter.get_unwrap(height);
        let block_price = height_to_price_iter.as_mut().map(|v| *v.get_unwrap(height));

        // Debug validation: verify context methods match iterator values
        debug_assert_eq!(ctx.timestamp_at(height), timestamp);
        debug_assert_eq!(ctx.price_at(height), block_price);

        // Build txindex mappings for this block
        let txoutindex_to_txindex =
            build_txoutindex_to_txindex(first_txindex, tx_count, &mut txindex_to_output_count_iter);
        let txinindex_to_txindex =
            build_txinindex_to_txindex(first_txindex, tx_count, &mut txindex_to_input_count_iter);

        // Get first address indexes for this height
        let first_addressindexes = ByAddressType {
            p2a: TypeIndex::from(first_p2a_iter.get_unwrap(height).to_usize()),
            p2pk33: TypeIndex::from(first_p2pk33_iter.get_unwrap(height).to_usize()),
            p2pk65: TypeIndex::from(first_p2pk65_iter.get_unwrap(height).to_usize()),
            p2pkh: TypeIndex::from(first_p2pkh_iter.get_unwrap(height).to_usize()),
            p2sh: TypeIndex::from(first_p2sh_iter.get_unwrap(height).to_usize()),
            p2tr: TypeIndex::from(first_p2tr_iter.get_unwrap(height).to_usize()),
            p2wpkh: TypeIndex::from(first_p2wpkh_iter.get_unwrap(height).to_usize()),
            p2wsh: TypeIndex::from(first_p2wsh_iter.get_unwrap(height).to_usize()),
        };

        // Reset per-block values for all separate cohorts
        reset_block_values(&mut vecs.utxo_cohorts, &mut vecs.address_cohorts);

        // Reset per-block activity counts
        activity_counts.reset();

        // Collect output/input data using reusable iterators (16KB buffered reads)
        // Must be done before thread::scope since iterators aren't Send
        let txoutdata_vec = txout_iters.collect_block_outputs(first_txoutindex, output_count);

        let (input_values, input_prev_heights, input_outputtypes, input_typeindexes) =
            if input_count > 1 {
                txin_iters.collect_block_inputs(first_txinindex + 1, input_count - 1, height)
            } else {
                (Vec::new(), Vec::new(), Vec::new(), Vec::new())
            };

        // Process outputs and inputs in parallel with tick-tock
        let (outputs_result, inputs_result) = thread::scope(|scope| {
            // Tick-tock age transitions in background
            scope.spawn(|| {
                vecs.utxo_cohorts
                    .tick_tock_next_block(chain_state, timestamp);
            });

            let outputs_handle = scope.spawn(|| {
                // Process outputs (receive)
                process_outputs(
                    &txoutindex_to_txindex,
                    &txoutdata_vec,
                    &first_addressindexes,
                    &cache,
                    &vr,
                    &vecs.any_address_indexes,
                    &vecs.addresses_data,
                )
            });

            // Process inputs (send) - skip coinbase input
            let inputs_result = if input_count > 1 {
                process_inputs(
                    input_count - 1,
                    &txinindex_to_txindex[1..], // Skip coinbase
                    &input_values,
                    &input_outputtypes,
                    &input_typeindexes,
                    &input_prev_heights,
                    &first_addressindexes,
                    &cache,
                    &vr,
                    &vecs.any_address_indexes,
                    &vecs.addresses_data,
                )
            } else {
                InputsResult {
                    height_to_sent: Default::default(),
                    sent_data: Default::default(),
                    address_data: Default::default(),
                    txindex_vecs: Default::default(),
                }
            };

            let outputs_result = outputs_handle.join().unwrap();

            (outputs_result, inputs_result)
        });

        // Merge new address data into current cache
        cache.merge_loaded(outputs_result.address_data);
        cache.merge_loaded(inputs_result.address_data);

        // Combine txindex_vecs from outputs and inputs, then update tx_count
        let combined_txindex_vecs = outputs_result
            .txindex_vecs
            .merge_vec(inputs_result.txindex_vecs);
        cache.update_tx_counts(combined_txindex_vecs);

        let mut transacted = outputs_result.transacted;
        let mut height_to_sent = inputs_result.height_to_sent;

        // Handle special cases
        if height == Height::ZERO {
            // Genesis block - reset transacted (50 BTC is unspendable, handled in supply module)
            transacted = Transacted::default();
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

        // Build set of addresses that received this block (for detecting "both" in sent)
        let received_addresses: ByAddressType<FxHashSet<TypeIndex>> = {
            let mut sets = ByAddressType::<FxHashSet<TypeIndex>>::default();
            for (output_type, vec) in outputs_result.received_data.iter() {
                let set = sets.get_mut_unwrap(output_type);
                for (type_index, _) in vec {
                    set.insert(*type_index);
                }
            }
            sets
        };

        // Process UTXO cohorts and Address cohorts in parallel
        // - Main thread: UTXO cohorts receive/send
        // - Spawned thread: Address cohorts process_received/process_sent
        thread::scope(|scope| {
            // Spawn address cohort processing in background thread
            scope.spawn(|| {
                let mut lookup = cache.as_lookup();

                // Process received outputs (addresses receiving funds)
                process_received(
                    outputs_result.received_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    &mut addr_counts,
                    &mut empty_addr_counts,
                    &mut activity_counts,
                );

                // Process sent inputs (addresses sending funds)
                // Uses separate price/timestamp vecs to avoid borrowing chain_state
                process_sent(
                    inputs_result.sent_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    ctx.price_range_max.as_ref(),
                    &mut addr_counts,
                    &mut empty_addr_counts,
                    &mut activity_counts,
                    &received_addresses,
                    height_to_price_vec.as_deref(),
                    height_to_timestamp_vec,
                    height,
                    timestamp,
                )
                .unwrap();
            });

            // Main thread: Update UTXO cohorts
            vecs.utxo_cohorts
                .receive(transacted, height, timestamp, block_price);
            vecs.utxo_cohorts
                .send(height_to_sent, chain_state, ctx.price_range_max.as_ref());
        });

        // Push to height-indexed vectors
        vecs.addr_count
            .truncate_push_height(height, addr_counts.sum(), &addr_counts)?;
        vecs.empty_addr_count.truncate_push_height(
            height,
            empty_addr_counts.sum(),
            &empty_addr_counts,
        )?;
        vecs.address_activity
            .truncate_push_height(height, &activity_counts)?;

        // Get date info for unrealized state computation
        let date = height_to_date_iter.get_unwrap(height);
        let dateindex = DateIndex::try_from(date).unwrap();
        let date_first_height = dateindex_to_first_height_iter.get_unwrap(dateindex);
        let date_height_count = dateindex_to_height_count_iter.get_unwrap(dateindex);
        let is_date_last_height =
            date_first_height + Height::from(date_height_count).decremented().unwrap() == height;
        let date_price = dateindex_to_price_iter
            .as_mut()
            .map(|v| is_date_last_height.then(|| *v.get_unwrap(dateindex)));
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

        // Compute and push percentiles for aggregate cohorts (all, sth, lth)
        if let Some(dateindex) = dateindex_opt {
            let spot = date_price
                .flatten()
                .map(|c| c.to_dollars())
                .unwrap_or(Dollars::NAN);
            vecs.utxo_cohorts
                .truncate_push_aggregate_percentiles(dateindex, spot)?;
        }

        // Periodic checkpoint flush
        if height != last_height
            && height != Height::ZERO
            && height.to_usize() % FLUSH_INTERVAL == 0
        {
            // Drop readers to release mmap handles
            drop(vr);

            let (empty_updates, loaded_updates) = cache.take();

            // Process address updates (mutations)
            process_address_updates(
                &mut vecs.addresses_data,
                &mut vecs.any_address_indexes,
                empty_updates,
                loaded_updates,
            )?;

            let _lock = exit.lock();

            // Write to disk (pure I/O) - no changes saved for periodic flushes
            write(vecs, height, chain_state, false)?;
            vecs.flush()?;

            // Recreate readers
            vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);
        }
    }

    // Final write - always save changes for rollback support

    let _lock = exit.lock();
    drop(vr);

    let (empty_updates, loaded_updates) = cache.take();

    // Process address updates (mutations)
    process_address_updates(
        &mut vecs.addresses_data,
        &mut vecs.any_address_indexes,
        empty_updates,
        loaded_updates,
    )?;

    // Write to disk (pure I/O) - save changes for rollback
    write(vecs, last_height, chain_state, true)?;

    Ok(())
}

/// Reset per-block values for all separate cohorts.
fn reset_block_values(utxo_cohorts: &mut UTXOCohorts, address_cohorts: &mut AddressCohorts) {
    utxo_cohorts.iter_separate_mut().for_each(|v| {
        if let Some(state) = v.state.as_mut() {
            state.reset_single_iteration_values();
        }
    });

    address_cohorts.iter_separate_mut().for_each(|v| {
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
    height_price: Option<CentsUnsigned>,
    dateindex: Option<DateIndex>,
    date_price: Option<Option<CentsUnsigned>>,
) -> Result<()> {
    // utxo_cohorts.iter_separate_mut().try_for_each(|v| {
    utxo_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price, dateindex, date_price)
    })?;

    // address_cohorts.iter_separate_mut().try_for_each(|v| {
    address_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price, dateindex, date_price)
    })?;

    Ok(())
}

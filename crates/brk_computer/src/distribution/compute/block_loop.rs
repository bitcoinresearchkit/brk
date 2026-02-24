use std::thread;

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Cents, Date, Day1, Height, OutputType, Sats, StoredU64, TxIndex, TypeIndex};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use tracing::{debug, info};
use vecdb::{AnyVec, Exit, ReadableVec, VecIndex};

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
    indexes, inputs, outputs, prices, transactions,
};

use super::{
    super::{
        RangeMap,
        cohorts::{AddressCohorts, DynCohortVecs, UTXOCohorts},
        vecs::Vecs,
    },
    BIP30_DUPLICATE_HEIGHT_1, BIP30_DUPLICATE_HEIGHT_2, BIP30_ORIGINAL_HEIGHT_1,
    BIP30_ORIGINAL_HEIGHT_2, ComputeContext, FLUSH_INTERVAL, TxInReaders, TxOutReaders,
    VecsReaders, build_txinindex_to_txindex, build_txoutindex_to_txindex,
};

/// Process all blocks from starting_height to last_height.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_blocks(
    vecs: &mut Vecs,
    indexer: &Indexer,
    indexes: &indexes::Vecs,
    inputs: &inputs::Vecs,
    outputs: &outputs::Vecs,
    transactions: &transactions::Vecs,
    blocks: &blocks::Vecs,
    prices: &prices::Vecs,
    starting_height: Height,
    last_height: Height,
    chain_state: &mut Vec<BlockState>,
    exit: &Exit,
) -> Result<()> {
    // Create computation context with pre-computed vectors for thread-safe access
    debug!("creating ComputeContext");
    let ctx = ComputeContext::new(starting_height, last_height, blocks, prices);
    debug!("ComputeContext created");

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
    let height_to_output_count = &outputs.count.total_count.sum_cum.sum.0;
    let height_to_input_count = &inputs.count.height.sum_cum.sum.0;
    // From blocks:
    let height_to_timestamp = &blocks.time.timestamp_monotonic;
    let height_to_date = &blocks.time.date;
    let day1_to_first_height = &indexes.day1.first_height;
    let day1_to_height_count = &indexes.day1.height_count;
    let txindex_to_output_count = &indexes.txindex.output_count;
    let txindex_to_input_count = &indexes.txindex.input_count;

    // From price - use cents for computation:
    let height_to_price = &prices.cents.price;

    // Access pre-computed vectors from context for thread-safe access
    let height_to_price_vec = &ctx.height_to_price;
    let height_to_timestamp_vec = &ctx.height_to_timestamp;

    // Range for pre-collecting height-indexed vecs
    let start_usize = starting_height.to_usize();
    let end_usize = last_height.to_usize() + 1;

    // Pre-collect height-indexed vecs for the block range (bulk read before hot loop)
    let height_to_first_txindex_vec: Vec<TxIndex> =
        height_to_first_txindex.collect_range_at(start_usize, end_usize);
    let height_to_first_txoutindex_vec: Vec<_> =
        height_to_first_txoutindex.collect_range_at(start_usize, end_usize);
    let height_to_first_txinindex_vec: Vec<_> =
        height_to_first_txinindex.collect_range_at(start_usize, end_usize);
    let height_to_tx_count_vec: Vec<_> =
        height_to_tx_count.collect_range_at(start_usize, end_usize);
    let height_to_output_count_vec: Vec<_> =
        height_to_output_count.collect_range_at(start_usize, end_usize);
    let height_to_input_count_vec: Vec<_> =
        height_to_input_count.collect_range_at(start_usize, end_usize);
    let height_to_timestamp_collected: Vec<_> =
        height_to_timestamp.collect_range_at(start_usize, end_usize);
    let height_to_price_collected: Vec<_> =
        height_to_price.collect_range_at(start_usize, end_usize);

    debug!("creating VecsReaders");
    let mut vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);
    debug!("VecsReaders created");

    // Build txindex -> height lookup map for efficient prev_height computation
    debug!("building txindex_to_height RangeMap");
    let mut txindex_to_height: RangeMap<TxIndex, Height> = {
        let first_txindex_len = indexer.vecs.transactions.first_txindex.len();
        let all_first_txindexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_range_at(0, first_txindex_len);
        let mut map = RangeMap::with_capacity(first_txindex_len);
        for first_txindex in all_first_txindexes {
            map.push(first_txindex);
        }
        map
    };
    debug!("txindex_to_height RangeMap built");

    // Create reusable iterators for sequential txout/txin reads (16KB buffered)
    let txout_iters = TxOutReaders::new(indexer);
    let mut txin_iters = TxInReaders::new(indexer, inputs, &mut txindex_to_height);

    // Pre-collect first address indexes per type for the block range
    let first_p2a_vec = indexer
        .vecs
        .addresses
        .first_p2aaddressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2pk33_vec = indexer
        .vecs
        .addresses
        .first_p2pk33addressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2pk65_vec = indexer
        .vecs
        .addresses
        .first_p2pk65addressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2pkh_vec = indexer
        .vecs
        .addresses
        .first_p2pkhaddressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2sh_vec = indexer
        .vecs
        .addresses
        .first_p2shaddressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2tr_vec = indexer
        .vecs
        .addresses
        .first_p2traddressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2wpkh_vec = indexer
        .vecs
        .addresses
        .first_p2wpkhaddressindex
        .collect_range_at(start_usize, end_usize);
    let first_p2wsh_vec = indexer
        .vecs
        .addresses
        .first_p2wshaddressindex
        .collect_range_at(start_usize, end_usize);

    // Track running totals - recover from previous height if resuming
    debug!("recovering addr_counts from height {}", starting_height);
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
    debug!("addr_counts recovered");

    // Track activity counts - reset each block
    let mut activity_counts = AddressTypeToActivityCounts::default();

    // Pre-collect lazy vecs that don't support iterators
    let height_to_date_vec: Vec<Date> = height_to_date.collect_range_at(start_usize, end_usize);

    debug!("creating AddressCache");
    let mut cache = AddressCache::new();
    debug!("AddressCache created, entering main loop");

    // Cache for day1 lookups - same day1 repeats ~140 times per day
    let mut cached_day1 = Day1::default();
    let mut cached_date_first_height = Height::ZERO;
    let mut cached_date_height_count = StoredU64::default();

    // Reusable hashsets for received addresses (avoid per-block allocation)
    let mut received_addresses = ByAddressType::<FxHashSet<TypeIndex>>::default();

    // Main block iteration
    for height in starting_height.to_usize()..=last_height.to_usize() {
        let height = Height::from(height);

        info!("Processing chain at {}...", height);

        // Get block metadata from pre-collected vecs
        let offset = height.to_usize() - start_usize;
        let first_txindex = height_to_first_txindex_vec[offset];
        let tx_count = u64::from(height_to_tx_count_vec[offset]);
        let first_txoutindex = height_to_first_txoutindex_vec[offset].to_usize();
        let output_count = u64::from(height_to_output_count_vec[offset]) as usize;
        let first_txinindex = height_to_first_txinindex_vec[offset].to_usize();
        let input_count = u64::from(height_to_input_count_vec[offset]) as usize;
        let timestamp = height_to_timestamp_collected[offset];
        let block_price = height_to_price_collected[offset];

        // Debug validation: verify context methods match pre-collected values
        debug_assert_eq!(ctx.timestamp_at(height), timestamp);
        debug_assert_eq!(ctx.price_at(height), block_price);

        // Build txindex mappings for this block (pass ReadableVec refs directly)
        let txoutindex_to_txindex =
            build_txoutindex_to_txindex(first_txindex, tx_count, txindex_to_output_count);
        let txinindex_to_txindex =
            build_txinindex_to_txindex(first_txindex, tx_count, txindex_to_input_count);

        // Get first address indexes for this height from pre-collected vecs
        let first_addressindexes = ByAddressType {
            p2a: TypeIndex::from(first_p2a_vec[offset].to_usize()),
            p2pk33: TypeIndex::from(first_p2pk33_vec[offset].to_usize()),
            p2pk65: TypeIndex::from(first_p2pk65_vec[offset].to_usize()),
            p2pkh: TypeIndex::from(first_p2pkh_vec[offset].to_usize()),
            p2sh: TypeIndex::from(first_p2sh_vec[offset].to_usize()),
            p2tr: TypeIndex::from(first_p2tr_vec[offset].to_usize()),
            p2wpkh: TypeIndex::from(first_p2wpkh_vec[offset].to_usize()),
            p2wsh: TypeIndex::from(first_p2wsh_vec[offset].to_usize()),
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
        let (outputs_result, inputs_result) = thread::scope(|scope| -> Result<_> {
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
                )?
            } else {
                InputsResult {
                    height_to_sent: Default::default(),
                    sent_data: Default::default(),
                    address_data: Default::default(),
                    txindex_vecs: Default::default(),
                }
            };

            let outputs_result = outputs_handle.join().unwrap()?;

            Ok((outputs_result, inputs_result))
        })?;

        // Merge new address data into current cache
        cache.merge_funded(outputs_result.address_data);
        cache.merge_funded(inputs_result.address_data);

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
        // Reuse pre-allocated hashsets: clear preserves capacity, avoiding reallocation
        received_addresses.values_mut().for_each(|set| set.clear());
        for (output_type, vec) in outputs_result.received_data.iter() {
            let set = received_addresses.get_mut_unwrap(output_type);
            for (type_index, _) in vec {
                set.insert(*type_index);
            }
        }

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
                    &ctx.price_range_max,
                    &mut addr_counts,
                    &mut empty_addr_counts,
                    &mut activity_counts,
                    &received_addresses,
                    height_to_price_vec,
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
                .send(height_to_sent, chain_state, &ctx.price_range_max);
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

        // Get date info for unrealized state computation (cold path - once per day)
        // Cache day1 lookups: same day1 repeats ~140 times per day,
        // avoiding redundant PcoVec page decompressions.
        let date = height_to_date_vec[offset];
        let day1 = Day1::try_from(date).unwrap();
        let (date_first_height, date_height_count) = if day1 == cached_day1 {
            (cached_date_first_height, cached_date_height_count)
        } else {
            let fh: Height = day1_to_first_height.collect_one(day1).unwrap();
            let hc = day1_to_height_count.collect_one(day1).unwrap();
            cached_day1 = day1;
            cached_date_first_height = fh;
            cached_date_height_count = hc;
            (fh, hc)
        };
        let is_date_last_height =
            date_first_height + Height::from(date_height_count).decremented().unwrap() == height;
        let day1_opt = is_date_last_height.then_some(day1);

        // Push cohort states and compute unrealized
        push_cohort_states(
            &mut vecs.utxo_cohorts,
            &mut vecs.address_cohorts,
            height,
            block_price,
        )?;

        // Compute and push percentiles for aggregate cohorts (all, sth, lth)
        let spot = block_price.to_dollars();
        vecs.utxo_cohorts.truncate_push_aggregate_percentiles(
            height,
            spot,
            day1_opt,
            &vecs.states_path,
        )?;

        // Compute unrealized peak regret by age range (once per day)
        if day1_opt.is_some() {
            vecs.utxo_cohorts.compute_and_push_peak_regret(
                chain_state,
                height,
                timestamp,
                block_price,
                &ctx.price_range_max,
            )?;
        }

        // Periodic checkpoint flush
        if height != last_height
            && height != Height::ZERO
            && height.to_usize() % FLUSH_INTERVAL == 0
        {
            // Drop readers to release mmap handles
            drop(vr);

            let (empty_updates, funded_updates) = cache.take();

            // Process address updates (mutations)
            process_address_updates(
                &mut vecs.addresses_data,
                &mut vecs.any_address_indexes,
                empty_updates,
                funded_updates,
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

    let (empty_updates, funded_updates) = cache.take();

    // Process address updates (mutations)
    process_address_updates(
        &mut vecs.addresses_data,
        &mut vecs.any_address_indexes,
        empty_updates,
        funded_updates,
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
    height_price: Cents,
) -> Result<()> {
    utxo_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price)
    })?;

    address_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price)
    })?;

    Ok(())
}

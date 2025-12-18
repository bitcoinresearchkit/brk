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
use brk_types::{DateIndex, Height, OutputType, Sats, TypeIndex};
use log::info;
use vecdb::{Exit, GenericStoredVec, IterableVec, TypedVecIterator, VecIndex};

use crate::{
    chain, indexes, price,
    stateful::{
        address::AddressTypeToAddressCount,
        compute::flush::{flush, process_address_updates},
        process::{
            AddressLookup, EmptyAddressDataWithSource, InputsResult, LoadedAddressDataWithSource,
            build_txoutindex_to_height_map, process_inputs, process_outputs, process_received,
            process_sent, update_tx_counts,
        },
    },
    states::{BlockState, Transacted},
    utils::OptionExt,
};

use super::{
    super::{
        address::AddressTypeToTypeIndexMap,
        cohorts::{AddressCohorts, DynCohortVecs, UTXOCohorts},
        vecs::Vecs,
    },
    BIP30_DUPLICATE_HEIGHT_1, BIP30_DUPLICATE_HEIGHT_2, BIP30_ORIGINAL_HEIGHT_1,
    BIP30_ORIGINAL_HEIGHT_2, ComputeContext, FLUSH_INTERVAL, IndexerReaders, TxInIterators,
    TxOutIterators, VecsReaders, build_txinindex_to_txindex, build_txoutindex_to_txindex,
};

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
    // Create computation context with pre-computed vectors for thread-safe access
    let ctx = ComputeContext::new(starting_height, last_height, indexes, price);

    if ctx.starting_height > ctx.last_height {
        return Ok(());
    }

    info!(
        "Processing blocks {} to {} (compute_dollars: {}, price_data: {})...",
        ctx.starting_height,
        ctx.last_height,
        ctx.compute_dollars,
        ctx.price.is_some()
    );

    info!("Setting up references...");

    // References to vectors using correct field paths
    // From indexer.vecs:
    let height_to_first_txindex = &indexer.vecs.tx.height_to_first_txindex;
    let height_to_first_txoutindex = &indexer.vecs.txout.height_to_first_txoutindex;
    let height_to_first_txinindex = &indexer.vecs.txin.height_to_first_txinindex;

    // From chain (via .height.u() or .height.unwrap_sum() patterns):
    let height_to_tx_count = chain.indexes_to_tx_count.height.u();
    let height_to_output_count = chain.indexes_to_output_count.height.unwrap_sum();
    let height_to_input_count = chain.indexes_to_input_count.height.unwrap_sum();
    let height_to_unclaimed_rewards = chain
        .indexes_to_unclaimed_rewards
        .sats
        .height
        .as_ref()
        .unwrap();

    // From indexes:
    let height_to_timestamp = &indexes.height_to_timestamp_fixed;
    let height_to_date = &indexes.height_to_date_fixed;
    let dateindex_to_first_height = &indexes.dateindex_to_first_height;
    let dateindex_to_height_count = &indexes.dateindex_to_height_count;
    let txindex_to_output_count = &indexes.txindex_to_output_count;
    let txindex_to_input_count = &indexes.txindex_to_input_count;

    // From price (optional):
    let height_to_price = price.map(|p| &p.chainindexes_to_price_close.height);
    let dateindex_to_price = price.map(|p| p.timeindexes_to_price_close.dateindex.u());

    // Access pre-computed vectors from context for thread-safe access
    let height_to_price_vec = &ctx.height_to_price;
    let height_to_timestamp_vec = &ctx.height_to_timestamp;

    info!("Creating iterators...");

    // Create iterators for sequential access
    let mut height_to_first_txindex_iter = height_to_first_txindex.into_iter();
    let mut height_to_first_txoutindex_iter = height_to_first_txoutindex.into_iter();
    let mut height_to_first_txinindex_iter = height_to_first_txinindex.into_iter();
    let mut height_to_tx_count_iter = height_to_tx_count.into_iter();
    let mut height_to_output_count_iter = height_to_output_count.into_iter();
    let mut height_to_input_count_iter = height_to_input_count.into_iter();
    let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
    let mut height_to_timestamp_iter = height_to_timestamp.into_iter();
    let mut height_to_date_iter = height_to_date.into_iter();
    let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
    let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();
    let mut txindex_to_output_count_iter = txindex_to_output_count.iter();
    let mut txindex_to_input_count_iter = txindex_to_input_count.iter();
    let mut height_to_price_iter = height_to_price.map(|v| v.into_iter());
    let mut dateindex_to_price_iter = dateindex_to_price.map(|v| v.into_iter());

    info!("Building txoutindex_to_height map...");

    // Build txoutindex -> height map for input processing
    let txoutindex_to_height = build_txoutindex_to_height_map(height_to_first_txoutindex);

    info!("Creating readers...");

    // Create readers for parallel data access
    let ir = IndexerReaders::new(indexer);
    let mut vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);

    // Create reusable iterators for sequential txout/txin reads (16KB buffered)
    let mut txout_iters = TxOutIterators::new(indexer);
    let mut txin_iters = TxInIterators::new(indexer);

    info!("Creating address iterators...");

    // Create iterators for first address indexes per type
    let mut first_p2a_iter = indexer
        .vecs
        .address
        .height_to_first_p2aaddressindex
        .into_iter();
    let mut first_p2pk33_iter = indexer
        .vecs
        .address
        .height_to_first_p2pk33addressindex
        .into_iter();
    let mut first_p2pk65_iter = indexer
        .vecs
        .address
        .height_to_first_p2pk65addressindex
        .into_iter();
    let mut first_p2pkh_iter = indexer
        .vecs
        .address
        .height_to_first_p2pkhaddressindex
        .into_iter();
    let mut first_p2sh_iter = indexer
        .vecs
        .address
        .height_to_first_p2shaddressindex
        .into_iter();
    let mut first_p2tr_iter = indexer
        .vecs
        .address
        .height_to_first_p2traddressindex
        .into_iter();
    let mut first_p2wpkh_iter = indexer
        .vecs
        .address
        .height_to_first_p2wpkhaddressindex
        .into_iter();
    let mut first_p2wsh_iter = indexer
        .vecs
        .address
        .height_to_first_p2wshaddressindex
        .into_iter();

    info!("Recovering running totals...");

    // Track running totals - recover from previous height if resuming
    let (
        mut unspendable_supply,
        mut opreturn_supply,
        mut addresstype_to_addr_count,
        mut addresstype_to_empty_addr_count,
    ) = if starting_height > Height::ZERO {
        info!("Reading unspendable_supply...");
        let prev_height = starting_height.decremented().unwrap();
        let unspendable = vecs
            .height_to_unspendable_supply
            .into_iter()
            .get_unwrap(prev_height);
        info!("Reading opreturn_supply...");
        let opreturn = vecs
            .height_to_opreturn_supply
            .into_iter()
            .get_unwrap(prev_height);
        info!("Reading addresstype_to_addr_count...");
        let addr_count = AddressTypeToAddressCount::from((
            &vecs.addresstype_to_height_to_addr_count,
            starting_height,
        ));
        info!("Reading addresstype_to_empty_addr_count...");
        let empty_addr_count = AddressTypeToAddressCount::from((
            &vecs.addresstype_to_height_to_empty_addr_count,
            starting_height,
        ));
        info!("Recovery complete.");
        (unspendable, opreturn, addr_count, empty_addr_count)
    } else {
        (
            Sats::ZERO,
            Sats::ZERO,
            AddressTypeToAddressCount::default(),
            AddressTypeToAddressCount::default(),
        )
    };

    // Persistent address data caches (accumulate across blocks, flushed at checkpoints)
    let mut loaded_cache: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource> =
        AddressTypeToTypeIndexMap::default();
    let mut empty_cache: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource> =
        AddressTypeToTypeIndexMap::default();

    info!("Starting main block iteration...");

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

        // Collect output/input data using reusable iterators (16KB buffered reads)
        // Must be done before thread::scope since iterators aren't Send
        let (output_values, output_types, output_typeindexes) =
            txout_iters.collect_block_outputs(first_txoutindex, output_count);

        let input_outpoints = if input_count > 1 {
            txin_iters.collect_block_outpoints(first_txinindex + 1, input_count - 1)
        } else {
            Vec::new()
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
                    output_count,
                    &txoutindex_to_txindex,
                    &output_values,
                    &output_types,
                    &output_typeindexes,
                    &first_addressindexes,
                    &loaded_cache,
                    &empty_cache,
                    &vr,
                    &vecs.any_address_indexes,
                    &vecs.addresses_data,
                )
            });

            // Process inputs (send) - skip coinbase input
            let inputs_result = if input_count > 1 {
                process_inputs(
                    first_txinindex + 1, // Skip coinbase
                    input_count - 1,
                    &txinindex_to_txindex[1..], // Skip coinbase
                    &input_outpoints,
                    &indexer.vecs.tx.txindex_to_first_txoutindex,
                    &indexer.vecs.txout.txoutindex_to_value,
                    &indexer.vecs.txout.txoutindex_to_outputtype,
                    &indexer.vecs.txout.txoutindex_to_typeindex,
                    &txoutindex_to_height,
                    &ir,
                    &first_addressindexes,
                    &loaded_cache,
                    &empty_cache,
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
                    txoutindex_to_txinindex_updates: Default::default(),
                }
            };

            let outputs_result = outputs_handle.join().unwrap();

            (outputs_result, inputs_result)
        });

        // Merge new address data into caches
        loaded_cache.merge_mut(outputs_result.address_data);
        loaded_cache.merge_mut(inputs_result.address_data);

        // Combine txindex_vecs from outputs and inputs, then update tx_count
        let combined_txindex_vecs = outputs_result
            .txindex_vecs
            .merge_vec(inputs_result.txindex_vecs);
        update_tx_counts(&mut loaded_cache, &mut empty_cache, combined_txindex_vecs);

        let mut transacted = outputs_result.transacted;
        let mut height_to_sent = inputs_result.height_to_sent;

        // Update supply tracking
        unspendable_supply += transacted.by_type.unspendable.opreturn.value
            + height_to_unclaimed_rewards_iter.get_unwrap(height);
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

        // Process UTXO cohorts and Address cohorts in parallel
        // - Main thread: UTXO cohorts receive/send
        // - Spawned thread: Address cohorts process_received/process_sent
        thread::scope(|scope| {
            // Spawn address cohort processing in background thread
            scope.spawn(|| {
                // Create lookup closure that returns None (data was pre-fetched in parallel phase)
                let get_address_data =
                    |_output_type, _type_index| -> Option<LoadedAddressDataWithSource> { None };

                let mut lookup = AddressLookup {
                    get_address_data,
                    loaded: &mut loaded_cache,
                    empty: &mut empty_cache,
                };

                // Process received outputs (addresses receiving funds)
                process_received(
                    outputs_result.received_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    &mut addresstype_to_addr_count,
                    &mut addresstype_to_empty_addr_count,
                );

                // Process sent inputs (addresses sending funds)
                // Uses separate price/timestamp vecs to avoid borrowing chain_state
                process_sent(
                    inputs_result.sent_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    &mut addresstype_to_addr_count,
                    &mut addresstype_to_empty_addr_count,
                    height_to_price_vec.as_deref(),
                    height_to_timestamp_vec,
                    height,
                    timestamp,
                )
                .unwrap();
            });

            // Main thread: Update UTXO cohorts
            vecs.utxo_cohorts.receive(transacted, height, block_price);
            vecs.utxo_cohorts.send(height_to_sent, chain_state);
        });

        // Update txoutindex_to_txinindex
        vecs.update_txoutindex_to_txinindex(
            output_count,
            inputs_result.txoutindex_to_txinindex_updates,
        )?;

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
            vecs.utxo_cohorts
                .truncate_push_aggregate_percentiles(dateindex)?;
        }

        // Periodic checkpoint flush
        if height != last_height
            && height != Height::ZERO
            && height.to_usize() % FLUSH_INTERVAL == 0
        {
            let _lock = exit.lock();

            // Drop readers before flush to release mmap handles
            drop(vr);

            // Process address updates (mutations)
            process_address_updates(
                &mut vecs.addresses_data,
                &mut vecs.any_address_indexes,
                std::mem::take(&mut empty_cache),
                std::mem::take(&mut loaded_cache),
            )?;

            // Flush to disk (pure I/O)
            flush(vecs, height, chain_state, exit)?;

            // Recreate readers after flush to pick up new data
            vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);
        }
    }

    // Final flush
    let _lock = exit.lock();
    drop(vr);

    // Process address updates (mutations)
    process_address_updates(
        &mut vecs.addresses_data,
        &mut vecs.any_address_indexes,
        std::mem::take(&mut empty_cache),
        std::mem::take(&mut loaded_cache),
    )?;

    // Flush to disk (pure I/O)
    flush(vecs, last_height, chain_state, exit)?;

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
    height_price: Option<brk_types::Dollars>,
    dateindex: Option<DateIndex>,
    date_price: Option<Option<brk_types::Dollars>>,
) -> Result<()> {
    utxo_cohorts.iter_separate_mut().try_for_each(|v| {
        // utxo_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price, dateindex, date_price)
    })?;

    address_cohorts.iter_separate_mut().try_for_each(|v| {
        // address_cohorts.par_iter_separate_mut().try_for_each(|v| {
        v.truncate_push(height)?;
        v.compute_then_truncate_push_unrealized_states(height, height_price, dateindex, date_price)
    })?;

    Ok(())
}

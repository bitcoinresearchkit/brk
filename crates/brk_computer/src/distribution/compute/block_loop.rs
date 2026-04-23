use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{
    Cents, Date, Height, ONE_DAY_IN_SEC, OutputType, Sats, StoredF64, Timestamp, TxIndex, TypeIndex,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use tracing::{debug, info};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec, unlikely};

use crate::{
    distribution::{
        addr::AddrMetricsState,
        block::{
            AddrCache, InputsResult, process_inputs, process_outputs, process_received,
            process_sent,
        },
        compute::write::{process_addr_updates, write},
        state::{BlockState, Transacted},
    },
    indexes, inputs, outputs, transactions,
};

use super::{
    super::{
        RangeMap,
        cohorts::{AddrCohorts, DynCohortVecs, UTXOCohorts},
        vecs::Vecs,
    },
    BIP30_DUPLICATE_HEIGHT_1, BIP30_DUPLICATE_HEIGHT_2, BIP30_ORIGINAL_HEIGHT_1,
    BIP30_ORIGINAL_HEIGHT_2, ComputeContext, FLUSH_INTERVAL, IndexToTxIndexBuf, PriceRangeMax,
    TxInReaders, TxOutReaders, VecsReaders,
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
    starting_height: Height,
    last_height: Height,
    chain_state: &mut Vec<BlockState>,
    tx_index_to_height: &mut RangeMap<TxIndex, Height>,
    cached_prices: &[Cents],
    cached_timestamps: &[Timestamp],
    cached_price_range_max: &PriceRangeMax,
    exit: &Exit,
) -> Result<()> {
    let ctx = ComputeContext {
        starting_height,
        last_height,
        height_to_timestamp: cached_timestamps,
        height_to_price: cached_prices,
        price_range_max: cached_price_range_max,
    };

    if ctx.starting_height > ctx.last_height {
        return Ok(());
    }

    let height_to_first_tx_index = &indexer.vecs.transactions.first_tx_index;
    let height_to_first_txout_index = &indexer.vecs.outputs.first_txout_index;
    let height_to_first_txin_index = &indexer.vecs.inputs.first_txin_index;
    let height_to_tx_count = &transactions.count.total.block;
    let height_to_output_count = &outputs.count.total.sum;
    let height_to_input_count = &inputs.count.sum;
    let tx_index_to_output_count = &indexes.tx_index.output_count;
    let tx_index_to_input_count = &indexes.tx_index.input_count;

    let height_to_price_vec = cached_prices;
    let height_to_timestamp_vec = cached_timestamps;

    let start_usize = starting_height.to_usize();
    let end_usize = last_height.to_usize() + 1;

    let height_to_first_tx_index_vec: Vec<TxIndex> =
        height_to_first_tx_index.collect_range_at(start_usize, end_usize);
    let height_to_first_txout_index_vec: Vec<_> =
        height_to_first_txout_index.collect_range_at(start_usize, end_usize);
    let height_to_first_txin_index_vec: Vec<_> =
        height_to_first_txin_index.collect_range_at(start_usize, end_usize);
    let height_to_tx_count_vec: Vec<_> =
        height_to_tx_count.collect_range_at(start_usize, end_usize);
    let height_to_output_count_vec: Vec<_> =
        height_to_output_count.collect_range_at(start_usize, end_usize);
    let height_to_input_count_vec: Vec<_> =
        height_to_input_count.collect_range_at(start_usize, end_usize);
    let height_to_timestamp_collected = &cached_timestamps[start_usize..end_usize];
    let height_to_price_collected = &cached_prices[start_usize..end_usize];

    // Pre-compute day boundaries to avoid per-block division in the hot loop
    let is_last_of_day: Vec<bool> = (start_usize..end_usize)
        .map(|h| {
            h == end_usize - 1
                || *cached_timestamps[h] / ONE_DAY_IN_SEC
                    != *cached_timestamps[h + 1] / ONE_DAY_IN_SEC
        })
        .collect();

    debug!("creating VecsReaders");
    let mut vr = VecsReaders::new(&vecs.any_addr_indexes, &vecs.addrs_data);
    debug!("VecsReaders created");

    // Extend tx_index_to_height RangeMap with new entries (incremental, O(new_blocks))
    let target_len = indexer.vecs.transactions.first_tx_index.len();
    let current_len = tx_index_to_height.len();
    if current_len < target_len {
        debug!(
            "extending tx_index_to_height RangeMap from {} to {}",
            current_len, target_len
        );
        let new_entries: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_range_at(current_len, target_len);
        for first_tx_index in new_entries {
            tx_index_to_height.push(first_tx_index);
        }
    } else if current_len > target_len {
        debug!(
            "truncating tx_index_to_height RangeMap from {} to {}",
            current_len, target_len
        );
        tx_index_to_height.truncate(target_len);
    }
    debug!(
        "tx_index_to_height RangeMap ready ({} entries)",
        tx_index_to_height.len()
    );

    // Create reusable iterators and buffers for per-block reads
    let mut txout_iters = TxOutReaders::new(indexer);
    let mut txin_iters = TxInReaders::new(indexer, inputs, tx_index_to_height);
    let mut txout_to_tx_index_buf = IndexToTxIndexBuf::new();
    let mut txin_to_tx_index_buf = IndexToTxIndexBuf::new();

    // Pre-collect first address indexes per type for the block range
    let first_p2a_vec = indexer
        .vecs
        .addrs
        .p2a
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pk33_vec = indexer
        .vecs
        .addrs
        .p2pk33
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pk65_vec = indexer
        .vecs
        .addrs
        .p2pk65
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pkh_vec = indexer
        .vecs
        .addrs
        .p2pkh
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2sh_vec = indexer
        .vecs
        .addrs
        .p2sh
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2tr_vec = indexer
        .vecs
        .addrs
        .p2tr
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2wpkh_vec = indexer
        .vecs
        .addrs
        .p2wpkh
        .first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2wsh_vec = indexer
        .vecs
        .addrs
        .p2wsh
        .first_index
        .collect_range_at(start_usize, end_usize);

    debug!("recovering addr metrics state from height {}", starting_height);
    let mut state = AddrMetricsState::from((&vecs.addrs, starting_height));
    debug!("addr metrics state recovered");

    debug!("creating AddrCache");
    let mut cache = AddrCache::new();
    debug!("AddrCache created, entering main loop");

    // Initialize Fenwick tree from imported BTreeMap state (one-time)
    vecs.utxo_cohorts.init_fenwick_if_needed();

    // Pre-truncate all stored vecs to starting_height (one-time).
    // This eliminates per-push truncation checks inside the block loop.
    {
        let start = starting_height.to_usize();
        vecs.utxo_cohorts
            .par_iter_vecs_mut()
            .chain(vecs.addr_cohorts.par_iter_vecs_mut())
            .chain(vecs.addrs.par_iter_height_mut())
            .chain(rayon::iter::once(
                &mut vecs.coinblocks_destroyed.block as &mut dyn AnyStoredVec,
            ))
            .try_for_each(|v| v.any_truncate_if_needed_at(start))?;
    }

    // Reusable hashsets (avoid per-block allocation)
    let mut received_addrs = ByAddrType::<FxHashSet<TypeIndex>>::default();
    let mut seen_senders = ByAddrType::<FxHashSet<TypeIndex>>::default();

    // Track earliest chain_state modification from sends (for incremental supply_state writes)
    let mut min_supply_modified: Option<Height> = None;

    // Main block iteration
    for height in starting_height.to_usize()..=last_height.to_usize() {
        let height = Height::from(height);

        if unlikely(height.is_multiple_of(100)) {
            info!("Processing chain at {}...", height);
        } else {
            debug!("Processing chain at {}...", height);
        }

        // Get block metadata from pre-collected vecs
        let offset = height.to_usize() - start_usize;
        let first_tx_index = height_to_first_tx_index_vec[offset];
        let tx_count = u64::from(height_to_tx_count_vec[offset]);
        let first_txout_index = height_to_first_txout_index_vec[offset].to_usize();
        let output_count = u64::from(height_to_output_count_vec[offset]) as usize;
        let first_txin_index = height_to_first_txin_index_vec[offset].to_usize();
        let input_count = u64::from(height_to_input_count_vec[offset]) as usize;
        let timestamp = height_to_timestamp_collected[offset];
        let block_price = height_to_price_collected[offset];

        // Debug validation: verify context methods match pre-collected values
        debug_assert_eq!(ctx.timestamp_at(height), timestamp);
        debug_assert_eq!(ctx.price_at(height), block_price);

        // Get first address indexes for this height from pre-collected vecs
        let first_addr_indexes = ByAddrType {
            p2a: TypeIndex::from(first_p2a_vec[offset].to_usize()),
            p2pk33: TypeIndex::from(first_p2pk33_vec[offset].to_usize()),
            p2pk65: TypeIndex::from(first_p2pk65_vec[offset].to_usize()),
            p2pkh: TypeIndex::from(first_p2pkh_vec[offset].to_usize()),
            p2sh: TypeIndex::from(first_p2sh_vec[offset].to_usize()),
            p2tr: TypeIndex::from(first_p2tr_vec[offset].to_usize()),
            p2wpkh: TypeIndex::from(first_p2wpkh_vec[offset].to_usize()),
            p2wsh: TypeIndex::from(first_p2wsh_vec[offset].to_usize()),
        };

        state.reset_per_block();

        // Process outputs, inputs, and tick-tock in parallel via rayon::join.
        // Collection (build tx_index mappings + bulk mmap reads) is merged into the
        // processing closures so outputs and inputs collection overlap each other
        // and tick-tock, instead of running sequentially before the join.
        let (matured, oi_result) = rayon::join(
            || {
                vecs.utxo_cohorts
                    .tick_tock_next_block(chain_state, timestamp)
            },
            || -> Result<_> {
                let (outputs_result, inputs_result) = rayon::join(
                    || {
                        let txout_index_to_tx_index = txout_to_tx_index_buf.build(
                            first_tx_index,
                            tx_count,
                            tx_index_to_output_count,
                        );
                        let txout_data_vec =
                            txout_iters.collect_block_outputs(first_txout_index, output_count);
                        process_outputs(
                            txout_index_to_tx_index,
                            txout_data_vec,
                            &first_addr_indexes,
                            &cache,
                            &vr,
                            &vecs.any_addr_indexes,
                            &vecs.addrs_data,
                        )
                    },
                    || -> Result<_> {
                        if input_count > 1 {
                            let txin_index_to_tx_index = txin_to_tx_index_buf.build(
                                first_tx_index,
                                tx_count,
                                tx_index_to_input_count,
                            );
                            let (
                                input_values,
                                input_prev_heights,
                                input_output_types,
                                input_type_indexes,
                            ) = txin_iters.collect_block_inputs(
                                first_txin_index + 1,
                                input_count - 1,
                                height,
                            );
                            process_inputs(
                                input_count - 1,
                                &txin_index_to_tx_index[1..],
                                input_values,
                                input_output_types,
                                input_type_indexes,
                                input_prev_heights,
                                &first_addr_indexes,
                                &cache,
                                &vr,
                                &vecs.any_addr_indexes,
                                &vecs.addrs_data,
                            )
                        } else {
                            Ok(InputsResult {
                                height_to_sent: Default::default(),
                                sent_data: Default::default(),
                                addr_data: Default::default(),
                                tx_index_vecs: Default::default(),
                            })
                        }
                    },
                );
                Ok((outputs_result?, inputs_result?))
            },
        );
        let (outputs_result, inputs_result) = oi_result?;

        // Merge new address data into current cache
        cache.merge_funded(outputs_result.addr_data);
        cache.merge_funded(inputs_result.addr_data);

        // Combine tx_index_vecs from outputs and inputs, then update tx_count
        let combined_tx_index_vecs = outputs_result
            .tx_index_vecs
            .merge_vec(inputs_result.tx_index_vecs);
        cache.update_tx_counts(combined_tx_index_vecs);

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
            supply: transacted.spendable_supply,
            price: block_price,
            timestamp,
        });

        // Compute total coinblocks destroyed (once globally, before send() consumes height_to_sent)
        {
            let h = height.to_usize();
            let total_satblocks: u128 = height_to_sent
                .iter()
                .filter(|(rh, _)| rh.to_usize() < h)
                .map(|(rh, sent)| {
                    let blocks_old = h - rh.to_usize();
                    blocks_old as u128 * u64::from(sent.spendable_supply.value) as u128
                })
                .sum();
            vecs.coinblocks_destroyed.block.push(StoredF64::from(
                total_satblocks as f64 / Sats::ONE_BTC_U128 as f64,
            ));
        }

        // Record maturation (sats crossing age boundaries)
        vecs.utxo_cohorts.push_maturation(&matured);

        // Build set of addresses that received this block (for detecting "both" in sent)
        // Reuse pre-allocated hashsets: clear preserves capacity, avoiding reallocation
        received_addrs.values_mut().for_each(|set| set.clear());
        for (output_type, vec) in outputs_result.received_data.iter() {
            let set = received_addrs.get_mut_unwrap(output_type);
            for (type_index, _) in vec {
                set.insert(*type_index);
            }
        }

        // Process UTXO cohorts and Addr cohorts in parallel
        let (_, addr_result) = rayon::join(
            || {
                // UTXO cohorts receive/send
                vecs.utxo_cohorts
                    .receive(transacted, height, timestamp, block_price);
                if let Some(min_h) =
                    vecs.utxo_cohorts
                        .send(height_to_sent, chain_state, ctx.price_range_max)
                {
                    min_supply_modified =
                        Some(min_supply_modified.map_or(min_h, |cur| cur.min(min_h)));
                }
            },
            || -> Result<()> {
                let mut lookup = cache.as_lookup();

                process_received(
                    outputs_result.received_data,
                    &mut vecs.addr_cohorts,
                    &mut lookup,
                    block_price,
                    &mut state,
                );

                process_sent(
                    inputs_result.sent_data,
                    &mut vecs.addr_cohorts,
                    &mut lookup,
                    block_price,
                    ctx.price_range_max,
                    &mut state,
                    &received_addrs,
                    height_to_price_vec,
                    height_to_timestamp_vec,
                    height,
                    timestamp,
                    &mut seen_senders,
                )
            },
        );
        addr_result?;

        // Update Fenwick tree from pending deltas (must happen before push_cohort_states drains pending)
        vecs.utxo_cohorts.update_fenwick_from_pending();

        let activity_totals = state.activity.totals();
        let active_addr_count =
            activity_totals.sending + activity_totals.receiving - activity_totals.bidirectional;
        vecs.addrs.push_height(&state, active_addr_count);

        let is_last_of_day = is_last_of_day[offset];
        let date_opt = is_last_of_day.then(|| Date::from(timestamp));

        push_cohort_states(
            &mut vecs.utxo_cohorts,
            &mut vecs.addr_cohorts,
            height,
            block_price,
        );

        vecs.utxo_cohorts
            .push_aggregate_percentiles(block_price, date_opt, &vecs.states_path)?;

        // Periodic checkpoint flush
        if height != last_height
            && height != Height::ZERO
            && height.to_usize() % FLUSH_INTERVAL == 0
        {
            // Drop readers to release mmap handles
            drop(vr);

            let (empty_updates, funded_updates) = cache.take();

            // Process address updates (mutations)
            process_addr_updates(
                &mut vecs.addrs_data,
                &mut vecs.any_addr_indexes,
                empty_updates,
                funded_updates,
            )?;

            let _lock = exit.lock();

            // Write to disk (pure I/O) - no changes saved for periodic flushes
            write(vecs, height, chain_state, min_supply_modified, false)?;
            min_supply_modified = None;
            vecs.flush()?;

            // Recreate readers
            vr = VecsReaders::new(&vecs.any_addr_indexes, &vecs.addrs_data);
        }
    }

    // Final write - always save changes for rollback support

    let _lock = exit.lock();
    drop(vr);

    let (empty_updates, funded_updates) = cache.take();

    // Process address updates (mutations)
    process_addr_updates(
        &mut vecs.addrs_data,
        &mut vecs.any_addr_indexes,
        empty_updates,
        funded_updates,
    )?;

    // Write to disk (pure I/O) - save changes for rollback
    write(vecs, last_height, chain_state, min_supply_modified, true)?;

    Ok(())
}

/// Push cohort states to height-indexed vectors, then reset per-block values.
fn push_cohort_states(
    utxo_cohorts: &mut UTXOCohorts,
    addr_cohorts: &mut AddrCohorts,
    height: Height,
    height_price: Cents,
) {
    // Phase 1: push + unrealized (no reset yet, states still needed for aggregation)
    rayon::join(
        || {
            utxo_cohorts.par_iter_separate_mut().for_each(|v| {
                v.push_state(height);
                v.push_unrealized_state(height_price);
            })
        },
        || {
            addr_cohorts.par_iter_separate_mut().for_each(|v| {
                v.push_state(height);
                v.push_unrealized_state(height_price);
            })
        },
    );

    // Phase 2: aggregate age_range states → push to overlapping cohorts
    utxo_cohorts.push_overlapping(height_price);

    // Phase 3: reset per-block values
    utxo_cohorts
        .iter_separate_mut()
        .for_each(|v| v.reset_single_iteration_values());
    addr_cohorts
        .iter_separate_mut()
        .for_each(|v| v.reset_single_iteration_values());
}

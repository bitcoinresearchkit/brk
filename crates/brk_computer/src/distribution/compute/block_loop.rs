use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{
    Cents, Date, Height, ONE_DAY_IN_SEC, OutputType, Sats, StoredF64, Timestamp, TxIndex, TypeIndex,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use tracing::{debug, info};
use vecdb::{AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use crate::{
    distribution::{
        address::{AddressTypeToActivityCounts, AddressTypeToAddressCount},
        block::{
            AddressCache, InputsResult, process_inputs, process_outputs, process_received,
            process_sent,
        },
        compute::write::{process_address_updates, write},
        state::{BlockState, Transacted},
    },
    indexes, inputs, outputs, transactions,
};

use super::{
    super::{
        RangeMap,
        cohorts::{AddressCohorts, DynCohortVecs, UTXOCohorts},
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
    txindex_to_height: &mut RangeMap<TxIndex, Height>,
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

    let height_to_first_txindex = &indexer.vecs.transactions.first_txindex;
    let height_to_first_txoutindex = &indexer.vecs.outputs.first_txoutindex;
    let height_to_first_txinindex = &indexer.vecs.inputs.first_txinindex;
    let height_to_tx_count = &transactions.count.total.base.height;
    let height_to_output_count = &outputs.count.total.full.sum;
    let height_to_input_count = &inputs.count.full.sum;
    let txindex_to_output_count = &indexes.txindex.output_count;
    let txindex_to_input_count = &indexes.txindex.input_count;

    let height_to_price_vec = cached_prices;
    let height_to_timestamp_vec = cached_timestamps;

    let start_usize = starting_height.to_usize();
    let end_usize = last_height.to_usize() + 1;

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
    let mut vr = VecsReaders::new(&vecs.any_address_indexes, &vecs.addresses_data);
    debug!("VecsReaders created");

    // Extend txindex_to_height RangeMap with new entries (incremental, O(new_blocks))
    let target_len = indexer.vecs.transactions.first_txindex.len();
    let current_len = txindex_to_height.len();
    if current_len < target_len {
        debug!(
            "extending txindex_to_height RangeMap from {} to {}",
            current_len, target_len
        );
        let new_entries: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_range_at(current_len, target_len);
        for first_txindex in new_entries {
            txindex_to_height.push(first_txindex);
        }
    } else if current_len > target_len {
        debug!(
            "truncating txindex_to_height RangeMap from {} to {}",
            current_len, target_len
        );
        txindex_to_height.truncate(target_len);
    }
    debug!(
        "txindex_to_height RangeMap ready ({} entries)",
        txindex_to_height.len()
    );

    // Create reusable iterators and buffers for per-block reads
    let mut txout_iters = TxOutReaders::new(indexer);
    let mut txin_iters = TxInReaders::new(indexer, inputs, txindex_to_height);
    let mut txout_to_txindex_buf = IndexToTxIndexBuf::new();
    let mut txin_to_txindex_buf = IndexToTxIndexBuf::new();

    // Pre-collect first address indexes per type for the block range
    let first_p2a_vec = indexer
        .vecs
        .addresses
        .p2a.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pk33_vec = indexer
        .vecs
        .addresses
        .p2pk33.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pk65_vec = indexer
        .vecs
        .addresses
        .p2pk65.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2pkh_vec = indexer
        .vecs
        .addresses
        .p2pkh.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2sh_vec = indexer
        .vecs
        .addresses
        .p2sh.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2tr_vec = indexer
        .vecs
        .addresses
        .p2tr.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2wpkh_vec = indexer
        .vecs
        .addresses
        .p2wpkh.first_index
        .collect_range_at(start_usize, end_usize);
    let first_p2wsh_vec = indexer
        .vecs
        .addresses
        .p2wsh.first_index
        .collect_range_at(start_usize, end_usize);

    // Track running totals - recover from previous height if resuming
    debug!("recovering address_counts from height {}", starting_height);
    let (mut address_counts, mut empty_address_counts) = if starting_height > Height::ZERO {
        let address_counts =
            AddressTypeToAddressCount::from((&vecs.addresses.funded.by_addresstype, starting_height));
        let empty_address_counts = AddressTypeToAddressCount::from((
            &vecs.addresses.empty.by_addresstype,
            starting_height,
        ));
        (address_counts, empty_address_counts)
    } else {
        (
            AddressTypeToAddressCount::default(),
            AddressTypeToAddressCount::default(),
        )
    };
    debug!("address_counts recovered");

    // Track activity counts - reset each block
    let mut activity_counts = AddressTypeToActivityCounts::default();

    debug!("creating AddressCache");
    let mut cache = AddressCache::new();
    debug!("AddressCache created, entering main loop");

    // Initialize Fenwick tree from imported BTreeMap state (one-time)
    vecs.utxo_cohorts.init_fenwick_if_needed();

    // Reusable hashsets (avoid per-block allocation)
    let mut received_addresses = ByAddressType::<FxHashSet<TypeIndex>>::default();
    let mut seen_senders = ByAddressType::<FxHashSet<TypeIndex>>::default();

    // Track earliest chain_state modification from sends (for incremental supply_state writes)
    let mut min_supply_modified: Option<Height> = None;

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

        // Reset per-block activity counts
        activity_counts.reset();

        // Process outputs, inputs, and tick-tock in parallel via rayon::join.
        // Collection (build txindex mappings + bulk mmap reads) is merged into the
        // processing closures so outputs and inputs collection overlap each other
        // and tick-tock, instead of running sequentially before the join.
        let (matured, oi_result) = rayon::join(
            || vecs.utxo_cohorts.tick_tock_next_block(chain_state, timestamp),
            || -> Result<_> {
                let (outputs_result, inputs_result) = rayon::join(
                    || {
                        let txoutindex_to_txindex = txout_to_txindex_buf
                            .build(first_txindex, tx_count, txindex_to_output_count);
                        let txoutdata_vec =
                            txout_iters.collect_block_outputs(first_txoutindex, output_count);
                        process_outputs(
                            txoutindex_to_txindex,
                            txoutdata_vec,
                            &first_addressindexes,
                            &cache,
                            &vr,
                            &vecs.any_address_indexes,
                            &vecs.addresses_data,
                        )
                    },
                    || -> Result<_> {
                        if input_count > 1 {
                            let txinindex_to_txindex = txin_to_txindex_buf
                                .build(first_txindex, tx_count, txindex_to_input_count);
                            let (input_values, input_prev_heights, input_outputtypes, input_typeindexes) =
                                txin_iters.collect_block_inputs(
                                    first_txinindex + 1,
                                    input_count - 1,
                                    height,
                                );
                            process_inputs(
                                input_count - 1,
                                &txinindex_to_txindex[1..],
                                input_values,
                                input_outputtypes,
                                input_typeindexes,
                                input_prev_heights,
                                &first_addressindexes,
                                &cache,
                                &vr,
                                &vecs.any_address_indexes,
                                &vecs.addresses_data,
                            )
                        } else {
                            Ok(InputsResult {
                                height_to_sent: Default::default(),
                                sent_data: Default::default(),
                                address_data: Default::default(),
                                txindex_vecs: Default::default(),
                            })
                        }
                    },
                );
                Ok((outputs_result?, inputs_result?))
            },
        );
        let (outputs_result, inputs_result) = oi_result?;

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
            vecs.coinblocks_destroyed.base.height.truncate_push(
                height,
                StoredF64::from(total_satblocks as f64 / Sats::ONE_BTC_U128 as f64),
            )?;
        }

        // Record maturation (sats crossing age boundaries)
        vecs.utxo_cohorts.push_maturation(height, &matured)?;

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

                // Process received outputs (addresses receiving funds)
                process_received(
                    outputs_result.received_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    &mut address_counts,
                    &mut empty_address_counts,
                    &mut activity_counts,
                );

                // Process sent inputs (addresses sending funds)
                process_sent(
                    inputs_result.sent_data,
                    &mut vecs.address_cohorts,
                    &mut lookup,
                    block_price,
                    ctx.price_range_max,
                    &mut address_counts,
                    &mut empty_address_counts,
                    &mut activity_counts,
                    &received_addresses,
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

        // Push to height-indexed vectors
        vecs.addresses.funded
            .truncate_push_height(height, address_counts.sum(), &address_counts)?;
        vecs.addresses.empty.truncate_push_height(
            height,
            empty_address_counts.sum(),
            &empty_address_counts,
        )?;
        vecs.addresses.activity
            .truncate_push_height(height, &activity_counts)?;

        let is_last_of_day = is_last_of_day[offset];
        let date_opt = is_last_of_day.then(|| Date::from(timestamp));

        push_cohort_states(
            &mut vecs.utxo_cohorts,
            &mut vecs.address_cohorts,
            height,
            block_price,
            date_opt.is_some(),
        )?;

        vecs.utxo_cohorts.truncate_push_aggregate_percentiles(
            height,
            block_price,
            date_opt,
            &vecs.states_path,
        )?;

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
            write(vecs, height, chain_state, min_supply_modified, false)?;
            min_supply_modified = None;
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
    write(vecs, last_height, chain_state, min_supply_modified, true)?;

    Ok(())
}

/// Push cohort states to height-indexed vectors, then reset per-block values.
fn push_cohort_states(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    height: Height,
    height_price: Cents,
    is_day_boundary: bool,
) -> Result<()> {
    // Phase 1: push + unrealized (no reset yet — states still needed for aggregation)
    let (r1, r2) = rayon::join(
        || {
            utxo_cohorts
                .par_iter_separate_mut()
                .try_for_each(|v| -> Result<()> {
                    v.truncate_push(height)?;
                    v.compute_then_truncate_push_unrealized_states(
                        height,
                        height_price,
                        is_day_boundary,
                    )?;
                    Ok(())
                })
        },
        || {
            address_cohorts
                .par_iter_separate_mut()
                .try_for_each(|v| -> Result<()> {
                    v.truncate_push(height)?;
                    v.compute_then_truncate_push_unrealized_states(
                        height,
                        height_price,
                        is_day_boundary,
                    )?;
                    Ok(())
                })
        },
    );
    r1?;
    r2?;

    // Phase 2: aggregate age_range realized states → push to overlapping cohorts' RealizedFull
    utxo_cohorts.push_overlapping_realized_full(height)?;

    // Phase 3: reset per-block values
    utxo_cohorts
        .iter_separate_mut()
        .for_each(|v| v.reset_single_iteration_values());
    address_cohorts
        .iter_separate_mut()
        .for_each(|v| v.reset_single_iteration_values());

    Ok(())
}

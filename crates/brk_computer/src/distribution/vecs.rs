use std::path::{Path, PathBuf};

use brk_cohort::{ByAddrType, Filter};
use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Cents, EmptyAddrData, EmptyAddrIndex, FundedAddrData, FundedAddrIndex, Height, Indexes,
    StoredF64, SupplyState, Timestamp, TxIndex, Version,
};
use rayon::prelude::*;
use tracing::{debug, info};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, ImportableVec, LazyVecFrom1, ReadOnlyClone,
    ReadableCloneableVec, ReadableVec, Rw, Stamp, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::{
        compute::{
            PriceRangeMax, StartMode, determine_start_mode, process_blocks, recover_state,
            reset_state,
        },
        state::BlockState,
    },
    indexes, inputs,
    internal::{
        PerBlockCumulativeRolling, WindowStartVec, Windows, WithAddrTypes,
        db_utils::{finalize_db, open_db},
    },
    outputs, prices, transactions,
};

use super::{
    AddrCohorts, AddrsDataVecs, AnyAddrIndexesVecs, RangeMap, UTXOCohorts,
    addr::{
        AddrActivityVecs, AddrCountsVecs, AddrMetricsState, DeltaVecs, ExposedAddrVecs,
        NewAddrCountVecs, ReusedAddrVecs, TotalAddrCountVecs,
    },
    metrics::AvgAmountMetrics,
};

const VERSION: Version = Version::new(24);

#[derive(Traversable)]
pub struct AddrMetricsVecs<M: StorageMode = Rw> {
    pub funded: AddrCountsVecs<M>,
    pub empty: AddrCountsVecs<M>,
    pub activity: AddrActivityVecs<M>,
    pub total: TotalAddrCountVecs<M>,
    pub new: NewAddrCountVecs<M>,
    pub reused: ReusedAddrVecs<M>,
    pub respent: ReusedAddrVecs<M>,
    pub exposed: ExposedAddrVecs<M>,
    pub delta: DeltaVecs,
    pub avg_amount: WithAddrTypes<AvgAmountMetrics<M>>,
    #[traversable(wrap = "indexes", rename = "funded")]
    pub funded_index:
        LazyVecFrom1<FundedAddrIndex, FundedAddrIndex, FundedAddrIndex, FundedAddrData>,
    #[traversable(wrap = "indexes", rename = "empty")]
    pub empty_index: LazyVecFrom1<EmptyAddrIndex, EmptyAddrIndex, EmptyAddrIndex, EmptyAddrData>,
}

impl AddrMetricsVecs {
    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.funded.reset_height()?;
        self.empty.reset_height()?;
        self.activity.reset_height()?;
        self.reused.reset_height()?;
        self.respent.reset_height()?;
        self.exposed.reset_height()?;
        self.avg_amount.reset_height()?;
        Ok(())
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.funded
            .min_stateful_len()
            .min(self.empty.min_stateful_len())
            .min(self.activity.min_stateful_len())
            .min(self.reused.min_stateful_len())
            .min(self.respent.min_stateful_len())
            .min(self.exposed.min_stateful_len())
    }

    /// Stateful vecs pushed per block. Mirrors [`Self::push_height`] and
    /// [`Self::min_stateful_len`]. Used by the stamped write path.
    pub(crate) fn par_iter_stateful_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.funded
            .par_iter_height_mut()
            .chain(self.empty.par_iter_height_mut())
            .chain(self.activity.par_iter_height_mut())
            .chain(self.reused.par_iter_height_mut())
            .chain(self.respent.par_iter_height_mut())
            .chain(self.exposed.par_iter_height_mut())
    }

    /// All height-indexed vecs including derived (`avg_amount`). Used for
    /// bulk truncation, where derived vecs must follow the stateful ones.
    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.funded
            .par_iter_height_mut()
            .chain(self.empty.par_iter_height_mut())
            .chain(self.activity.par_iter_height_mut())
            .chain(self.reused.par_iter_height_mut())
            .chain(self.respent.par_iter_height_mut())
            .chain(self.exposed.par_iter_height_mut())
            .chain(self.avg_amount.par_iter_height_mut())
    }

    /// Push one block's worth of per-addr-type running totals to all
    /// height-indexed vecs. `active_addr_count` is the block-level total
    /// of active addresses (sending + receiving - bidirectional).
    #[inline(always)]
    pub(crate) fn push_height(&mut self, state: &AddrMetricsState, active_addr_count: u32) {
        self.funded.push_counts(&state.funded);
        self.empty.push_counts(&state.empty);
        self.activity.push_height(&state.activity);
        self.exposed.push_height(&state.exposed);
        self.reused.push_height(&state.reused, active_addr_count);
        self.respent.push_height(&state.respent, active_addr_count);
    }
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,
    #[traversable(skip)]
    pub states_path: PathBuf,

    #[traversable(wrap = "supply", rename = "state")]
    pub supply_state: M::Stored<BytesVec<Height, SupplyState>>,
    #[traversable(wrap = "addrs", rename = "indexes")]
    pub any_addr_indexes: AnyAddrIndexesVecs<M>,
    #[traversable(wrap = "addrs", rename = "data")]
    pub addrs_data: AddrsDataVecs<M>,
    #[traversable(wrap = "cohorts", rename = "utxo")]
    pub utxo_cohorts: UTXOCohorts<M>,
    #[traversable(wrap = "cohorts", rename = "addr")]
    pub addr_cohorts: AddrCohorts<M>,
    #[traversable(wrap = "cointime/activity")]
    pub coinblocks_destroyed: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
    pub addrs: AddrMetricsVecs<M>,

    /// In-memory state that does NOT survive rollback.
    /// Grouped so that adding a new field automatically gets it reset.
    #[traversable(skip)]
    caches: DistributionTransientState,
}

/// In-memory state that does NOT survive rollback.
/// On rollback, the entire struct is replaced with `Default::default()`.
#[derive(Clone, Default)]
struct DistributionTransientState {
    /// Block state for UTXO processing. Persisted via supply_state.
    chain_state: Vec<BlockState>,
    /// tx_index→height reverse lookup.
    tx_index_to_height: RangeMap<TxIndex, Height>,
    /// Height→price mapping. Incrementally extended.
    prices: Vec<Cents>,
    /// Height→timestamp mapping. Incrementally extended.
    timestamps: Vec<Timestamp>,
    /// Sparse table for O(1) range-max price queries. Incrementally extended.
    price_range_max: PriceRangeMax,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let db_path = parent.join(super::DB_NAME);
        let states_path = db_path.join("states");

        let db = open_db(parent, super::DB_NAME, 20_000_000)?;
        db.set_min_regions(50_000)?;

        let version = parent_version + VERSION;

        let utxo_cohorts =
            UTXOCohorts::forced_import(&db, version, indexes, &states_path, cached_starts)?;

        let addr_cohorts =
            AddrCohorts::forced_import(&db, version, indexes, &states_path, cached_starts)?;

        // Create address data BytesVecs first so we can also use them for identity mappings
        let funded_addr_index_to_funded_addr_data = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "funded_addr_data", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let empty_addr_index_to_empty_addr_data = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "empty_addr_data", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;

        // Identity mappings for traversable
        let funded_addr_index = LazyVecFrom1::init(
            "funded_addr_index",
            version,
            funded_addr_index_to_funded_addr_data.read_only_boxed_clone(),
            |index, _| index,
        );
        let empty_addr_index = LazyVecFrom1::init(
            "empty_addr_index",
            version,
            empty_addr_index_to_empty_addr_data.read_only_boxed_clone(),
            |index, _| index,
        );

        let addr_count = AddrCountsVecs::forced_import(&db, "addr_count", version, indexes)?;
        let empty_addr_count =
            AddrCountsVecs::forced_import(&db, "empty_addr_count", version, indexes)?;
        let addr_activity = AddrActivityVecs::forced_import(&db, version, indexes, cached_starts)?;

        // Stored total = addr_count + empty_addr_count (global + per-type, with all derived indexes)
        let total_addr_count = TotalAddrCountVecs::forced_import(&db, version, indexes)?;

        // Per-block delta of total (global + per-type)
        let new_addr_count = NewAddrCountVecs::forced_import(&db, version, indexes, cached_starts)?;

        // Reused address tracking (counts + per-block uses + percent).
        // `reused_*` uses the receive-side predicate (funded_txo_count > 1,
        // industry standard). `respent_*` uses the spend-side counterpart
        // (spent_txo_count > 1, strictly more restrictive).
        let reused_addr_count =
            ReusedAddrVecs::forced_import(&db, "reused", version, indexes, cached_starts)?;
        let respent_addr_count =
            ReusedAddrVecs::forced_import(&db, "respent", version, indexes, cached_starts)?;

        // Exposed address tracking (counts + supply) - quantum / pubkey-exposure sense
        let exposed_addr_vecs = ExposedAddrVecs::forced_import(&db, version, indexes)?;

        // Growth rate: delta change + rate (global + per-type)
        let delta = DeltaVecs::new(version, &addr_count, cached_starts, indexes);

        // Average amount (supply / utxo_count, supply / funded_addr_count) for `all` and per addr type.
        let avg_amount = WithAddrTypes::<AvgAmountMetrics>::forced_import(&db, version, indexes)?;

        let this = Self {
            supply_state: BytesVec::forced_import_with(
                vecdb::ImportOptions::new(&db, "supply_state", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            addrs: AddrMetricsVecs {
                funded: addr_count,
                empty: empty_addr_count,
                activity: addr_activity,
                total: total_addr_count,
                new: new_addr_count,
                reused: reused_addr_count,
                respent: respent_addr_count,
                exposed: exposed_addr_vecs,
                delta,
                avg_amount,
                funded_index: funded_addr_index,
                empty_index: empty_addr_index,
            },

            utxo_cohorts,
            addr_cohorts,

            coinblocks_destroyed: PerBlockCumulativeRolling::forced_import(
                &db,
                "coinblocks_destroyed",
                version + Version::TWO,
                indexes,
                cached_starts,
            )?,

            any_addr_indexes: AnyAddrIndexesVecs::forced_import(&db, version)?,
            addrs_data: AddrsDataVecs {
                funded: funded_addr_index_to_funded_addr_data,
                empty: empty_addr_index_to_empty_addr_data,
            },
            caches: DistributionTransientState::default(),

            db,
            states_path,
        };

        finalize_db(&this.db, &this)?;
        Ok(this)
    }

    /// Reset in-memory caches that become stale after rollback.
    fn reset_in_memory_caches(&mut self) {
        self.utxo_cohorts.reset_caches();
        self.caches = DistributionTransientState::default();
    }

    /// Main computation loop.
    ///
    /// Processes blocks to compute UTXO and address cohort metrics:
    /// 1. Recovers state from checkpoints or starts fresh
    /// 2. Iterates through blocks, processing outputs/inputs in parallel
    /// 3. Flushes checkpoints periodically
    /// 4. Computes aggregate cohorts from separate cohorts
    /// 5. Computes derived metrics
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        transactions: &transactions::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &mut Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // 1. Find minimum height we have data for across stateful vecs
        let current_height = Height::from(self.supply_state.len());
        let min_stateful = self.min_stateful_len();

        // 2. Determine start mode and recover/reset state
        // Clamp to starting_indexes.height to handle reorg (indexer may require earlier start)
        let resume_target = current_height.min(starting_indexes.height);
        if resume_target < current_height {
            info!(
                "Reorg detected: rolling back from {} to {}",
                current_height, resume_target
            );
        }
        let start_mode = determine_start_mode(min_stateful.min(resume_target), resume_target);

        // Try to resume from checkpoint, fall back to fresh start if needed
        let recovered_height = match start_mode {
            StartMode::Resume(height) => {
                let stamp = Stamp::from(height);

                // Rollback BytesVec state and capture results for validation
                let chain_state_rollback = self.supply_state.rollback_before(stamp);

                // Validate all rollbacks and imports are consistent
                let recovered = recover_state(
                    height,
                    chain_state_rollback,
                    &mut self.any_addr_indexes,
                    &mut self.addrs_data,
                    &mut self.utxo_cohorts,
                    &mut self.addr_cohorts,
                )?;

                debug!(
                    "recover_state completed, starting_height={}",
                    recovered.starting_height
                );
                recovered.starting_height
            }
            StartMode::Fresh => Height::ZERO,
        };

        debug!("recovered_height={}", recovered_height);

        let needs_fresh_start = recovered_height.is_zero();
        let needs_rollback = recovered_height < current_height;

        if needs_fresh_start || needs_rollback {
            self.reset_in_memory_caches();
        }

        if needs_fresh_start {
            self.supply_state.reset()?;
            self.addrs.reset_height()?;
            reset_state(
                &mut self.any_addr_indexes,
                &mut self.addrs_data,
                &mut self.utxo_cohorts,
                &mut self.addr_cohorts,
            )?;
            info!("State recovery: fresh start");
        }

        // Populate price/timestamp caches from the prices module.
        // Must happen AFTER rollback/reset (which clears caches) but BEFORE
        // chain_state rebuild (which reads from them).
        let cache_target_len = prices
            .spot
            .cents
            .height
            .len()
            .min(indexes.timestamp.monotonic.len());
        let cache_current_len = self.caches.prices.len();
        if cache_target_len < cache_current_len {
            self.caches.prices.truncate(cache_target_len);
            self.caches.timestamps.truncate(cache_target_len);
            self.caches.price_range_max.truncate(cache_target_len);
        } else if cache_target_len > cache_current_len {
            let new_prices = prices
                .spot
                .cents
                .height
                .collect_range_at(cache_current_len, cache_target_len);
            let new_timestamps = indexes
                .timestamp
                .monotonic
                .collect_range_at(cache_current_len, cache_target_len);
            self.caches.prices.extend(new_prices);
            self.caches.timestamps.extend(new_timestamps);
        }
        self.caches.price_range_max.extend(&self.caches.prices);

        // Take chain_state and tx_index_to_height out of self to avoid borrow conflicts
        let mut chain_state = std::mem::take(&mut self.caches.chain_state);
        let mut tx_index_to_height = std::mem::take(&mut self.caches.tx_index_to_height);

        // Recover or reuse chain_state
        let starting_height = if recovered_height.is_zero() {
            Height::ZERO
        } else if chain_state.len() == usize::from(recovered_height) {
            // Normal resume: chain_state already matches, reuse as-is
            debug!(
                "reusing in-memory chain_state ({} entries)",
                chain_state.len()
            );
            recovered_height
        } else {
            debug!("rebuilding chain_state from stored values");

            let end = usize::from(recovered_height);
            debug!("building supply_state vec for {} heights", recovered_height);
            let supply_state_data: Vec<_> = self.supply_state.collect_range_at(0, end);
            chain_state = supply_state_data
                .into_iter()
                .enumerate()
                .map(|(h, supply)| BlockState {
                    supply,
                    price: self.caches.prices[h],
                    timestamp: self.caches.timestamps[h],
                })
                .collect();
            debug!("chain_state rebuilt");

            // Truncate RangeMap to match (entries are immutable, safe to keep)
            tx_index_to_height.truncate(end);

            recovered_height
        };

        // Update starting_indexes if we need to recompute from an earlier point
        if starting_height < starting_indexes.height {
            starting_indexes.height = starting_height;
        }

        // 2c. Validate computed versions
        debug!("validating computed versions");
        let base_version = VERSION;
        self.utxo_cohorts.validate_computed_versions(base_version)?;
        self.addr_cohorts.validate_computed_versions(base_version)?;
        debug!("computed versions validated");

        // 3. Get last height from indexer
        let last_height = Height::from(indexer.vecs.blocks.blockhash.len().saturating_sub(1));
        debug!(
            "last_height={}, starting_height={}",
            last_height, starting_height
        );

        // 4. Process blocks
        if starting_height <= last_height {
            debug!("calling process_blocks");

            let prices = std::mem::take(&mut self.caches.prices);
            let timestamps = std::mem::take(&mut self.caches.timestamps);
            let price_range_max = std::mem::take(&mut self.caches.price_range_max);

            process_blocks(
                self,
                indexer,
                indexes,
                inputs,
                outputs,
                transactions,
                starting_height,
                last_height,
                &mut chain_state,
                &mut tx_index_to_height,
                &prices,
                &timestamps,
                &price_range_max,
                exit,
            )?;

            self.caches.prices = prices;
            self.caches.timestamps = timestamps;
            self.caches.price_range_max = price_range_max;
        }

        // Put chain_state and tx_index_to_height back
        self.caches.chain_state = chain_state;
        self.caches.tx_index_to_height = tx_index_to_height;

        // 5. Compute aggregates (overlapping cohorts from separate cohorts)
        info!("Computing overlapping cohorts...");
        {
            let (r1, r2) = rayon::join(
                || {
                    self.utxo_cohorts
                        .compute_overlapping_vecs(starting_indexes, exit)
                },
                || {
                    self.addr_cohorts
                        .compute_overlapping_vecs(starting_indexes, exit)
                },
            );
            r1?;
            r2?;
        }

        // 5b. Compute coinblocks_destroyed cumulative from raw
        self.coinblocks_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        // 6. Compute rest part1 (day1 mappings)
        info!("Computing rest part 1...");
        {
            let (r1, r2) = rayon::join(
                || {
                    self.utxo_cohorts
                        .compute_rest_part1(prices, starting_indexes, exit)
                },
                || {
                    self.addr_cohorts
                        .compute_rest_part1(prices, starting_indexes, exit)
                },
            );
            r1?;
            r2?;
        }

        // 6b. Compute address count sum (by addr_type -> all)
        self.addrs.funded.compute_rest(starting_indexes, exit)?;
        self.addrs.empty.compute_rest(starting_indexes, exit)?;
        let t = &self.utxo_cohorts.type_;
        let type_supply_sats = ByAddrType::new(|filter| {
            let Filter::Type(ot) = filter else {
                unreachable!()
            };
            &t.get(ot).metrics.supply.total.sats.height
        });
        let all_supply_sats = &self.utxo_cohorts.all.metrics.supply.total.sats.height;
        self.addrs.reused.compute_rest(
            starting_indexes,
            &outputs.by_type,
            &inputs.by_type,
            prices,
            all_supply_sats,
            &type_supply_sats,
            exit,
        )?;
        self.addrs.respent.compute_rest(
            starting_indexes,
            &outputs.by_type,
            &inputs.by_type,
            prices,
            all_supply_sats,
            &type_supply_sats,
            exit,
        )?;
        self.addrs.exposed.compute_rest(
            starting_indexes,
            prices,
            all_supply_sats,
            &type_supply_sats,
            exit,
        )?;

        // Average amount (supply / utxo_count, supply / funded_addr_count) for `all` and per addr type.
        let all_m = &self.utxo_cohorts.all.metrics;
        self.addrs.avg_amount.all.compute(
            prices,
            &all_m.supply.total.sats.height,
            &all_m.outputs.unspent_count.height,
            &self.addrs.funded.all.height,
            starting_indexes.height,
            exit,
        )?;
        for ((ot, avg), (_, funded)) in self
            .addrs
            .avg_amount
            .by_addr_type
            .iter_mut()
            .zip(self.addrs.funded.by_addr_type.iter())
        {
            let type_m = &t.get(ot).metrics;
            avg.compute(
                prices,
                &type_m.supply.total.sats.height,
                &type_m.outputs.unspent_count.height,
                &funded.height,
                starting_indexes.height,
                exit,
            )?;
        }

        // 6c. Compute total_addr_count = addr_count + empty_addr_count
        self.addrs.total.compute(
            starting_indexes.height,
            &self.addrs.funded,
            &self.addrs.empty,
            exit,
        )?;

        self.addrs
            .activity
            .compute_rest(starting_indexes.height, exit)?;
        self.addrs
            .new
            .compute(starting_indexes.height, &self.addrs.total, exit)?;

        // 7. Compute rest part2 (relative metrics)
        let height_to_market_cap = self
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .usd
            .height
            .read_only_clone();

        info!("Computing rest part 2...");
        self.utxo_cohorts.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            &height_to_market_cap,
            exit,
        )?;

        let all_supply_sats = self
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .sats
            .height
            .read_only_clone();
        let all_utxo_count = self
            .utxo_cohorts
            .all
            .metrics
            .outputs
            .unspent_count
            .height
            .read_only_clone();
        self.addr_cohorts.compute_rest_part2(
            prices,
            starting_indexes,
            &all_supply_sats,
            &all_utxo_count,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }

    pub(crate) fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    fn min_stateful_len(&self) -> Height {
        self.utxo_cohorts
            .min_stateful_len()
            .min(self.addr_cohorts.min_stateful_len())
            .min(Height::from(self.supply_state.len()))
            .min(self.any_addr_indexes.min_stamped_len())
            .min(self.addrs_data.min_stamped_len())
            .min(Height::from(self.addrs.min_stateful_len()))
            .min(Height::from(self.coinblocks_destroyed.block.len()))
    }
}

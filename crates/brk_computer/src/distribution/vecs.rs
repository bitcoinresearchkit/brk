use std::path::{Path, PathBuf};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Cents, EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex, Height,
    Indexes, StoredF64, SupplyState, Timestamp, TxIndex, Version,
};
use tracing::{debug, info};
use vecdb::{
    AnyVec, BytesVec, Database, Exit, ImportableVec, LazyVecFrom1, ReadOnlyClone,
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
    internal::{finalize_db, open_db, ComputedPerBlockCumulative},
    outputs, prices, transactions,
};

use super::{
    AddressCohorts, AddressesDataVecs, AnyAddressIndexesVecs, RangeMap, UTXOCohorts,
    address::{
        AddrCountsVecs, AddressActivityVecs, DeltaVecs, NewAddrCountVecs, TotalAddrCountVecs,
    },
    compute::aggregates,
};

const VERSION: Version = Version::new(22);
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,
    #[traversable(skip)]
    pub states_path: PathBuf,

    pub supply_state: M::Stored<BytesVec<Height, SupplyState>>,
    pub any_address_indexes: AnyAddressIndexesVecs<M>,
    pub addresses_data: AddressesDataVecs<M>,
    pub utxo_cohorts: UTXOCohorts<M>,
    pub address_cohorts: AddressCohorts<M>,

    pub coinblocks_destroyed: ComputedPerBlockCumulative<StoredF64, M>,

    pub addr_count: AddrCountsVecs<M>,
    pub empty_addr_count: AddrCountsVecs<M>,
    pub address_activity: AddressActivityVecs<M>,

    /// Total addresses ever seen (addr_count + empty_addr_count) - stored, global + per-type
    pub total_addr_count: TotalAddrCountVecs<M>,
    /// New addresses per block (delta of total) - stored height + cumulative + rolling, global + per-type
    pub new_addr_count: NewAddrCountVecs<M>,
    /// Windowed change + growth rate for addr_count, global + per-type
    pub delta: DeltaVecs<M>,

    pub funded_address_index:
        LazyVecFrom1<FundedAddressIndex, FundedAddressIndex, FundedAddressIndex, FundedAddressData>,
    pub empty_address_index:
        LazyVecFrom1<EmptyAddressIndex, EmptyAddressIndex, EmptyAddressIndex, EmptyAddressData>,

    /// In-memory block state for UTXO processing. Persisted via supply_state.
    /// Kept across compute() calls to avoid O(n) rebuild on resume.
    #[traversable(skip)]
    chain_state: Vec<BlockState>,
    /// In-memory txindex→height reverse lookup. Kept across compute() calls.
    #[traversable(skip)]
    txindex_to_height: RangeMap<TxIndex, Height>,

    /// Cached height→price mapping. Incrementally extended, O(new_blocks) on resume.
    #[traversable(skip)]
    cached_prices: Vec<Cents>,
    /// Cached height→timestamp mapping. Incrementally extended, O(new_blocks) on resume.
    #[traversable(skip)]
    cached_timestamps: Vec<Timestamp>,
    /// Cached sparse table for O(1) range-max price queries. Incrementally extended.
    #[traversable(skip)]
    cached_price_range_max: PriceRangeMax,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db_path = parent.join(super::DB_NAME);
        let states_path = db_path.join("states");

        let db = open_db(parent, super::DB_NAME, 20_000_000)?;
        db.set_min_regions(50_000)?;

        let version = parent_version + VERSION;

        let utxo_cohorts = UTXOCohorts::forced_import(&db, version, indexes, &states_path)?;

        let address_cohorts = AddressCohorts::forced_import(&db, version, indexes, &states_path)?;

        // Create address data BytesVecs first so we can also use them for identity mappings
        let fundedaddressindex_to_fundedaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "fundedaddressdata", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let emptyaddressindex_to_emptyaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "emptyaddressdata", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;

        // Identity mappings for traversable
        let funded_address_index = LazyVecFrom1::init(
            "funded_address_index",
            version,
            fundedaddressindex_to_fundedaddressdata.read_only_boxed_clone(),
            |index, _| index,
        );
        let empty_address_index = LazyVecFrom1::init(
            "empty_address_index",
            version,
            emptyaddressindex_to_emptyaddressdata.read_only_boxed_clone(),
            |index, _| index,
        );

        let addr_count = AddrCountsVecs::forced_import(&db, "addr_count", version, indexes)?;
        let empty_addr_count =
            AddrCountsVecs::forced_import(&db, "empty_addr_count", version, indexes)?;
        let address_activity =
            AddressActivityVecs::forced_import(&db, "address_activity", version, indexes)?;

        // Stored total = addr_count + empty_addr_count (global + per-type, with all derived indexes)
        let total_addr_count = TotalAddrCountVecs::forced_import(&db, version, indexes)?;

        // Per-block delta of total (global + per-type)
        let new_addr_count = NewAddrCountVecs::forced_import(&db, version, indexes)?;

        // Growth rate: new / addr_count (global + per-type)
        let delta = DeltaVecs::forced_import(&db, version, indexes)?;

        let this = Self {
            supply_state: BytesVec::forced_import_with(
                vecdb::ImportOptions::new(&db, "supply_state", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            addr_count,
            empty_addr_count,
            address_activity,
            total_addr_count,
            new_addr_count,
            delta,

            utxo_cohorts,
            address_cohorts,

            coinblocks_destroyed: ComputedPerBlockCumulative::forced_import(
                &db,
                "coinblocks_destroyed",
                version + Version::TWO,
                indexes,
            )?,

            any_address_indexes: AnyAddressIndexesVecs::forced_import(&db, version)?,
            addresses_data: AddressesDataVecs {
                funded: fundedaddressindex_to_fundedaddressdata,
                empty: emptyaddressindex_to_emptyaddressdata,
            },
            funded_address_index,
            empty_address_index,

            chain_state: Vec::new(),
            txindex_to_height: RangeMap::default(),

            cached_prices: Vec::new(),
            cached_timestamps: Vec::new(),
            cached_price_range_max: PriceRangeMax::default(),

            db,
            states_path,
        };

        finalize_db(&this.db, &this)?;
        Ok(this)
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
        let cache_target_len = prices
            .price
            .cents
            .height
            .len()
            .min(blocks.time.timestamp_monotonic.len());
        let cache_current_len = self.cached_prices.len();
        if cache_target_len < cache_current_len {
            self.cached_prices.truncate(cache_target_len);
            self.cached_timestamps.truncate(cache_target_len);
            self.cached_price_range_max.truncate(cache_target_len);
        } else if cache_target_len > cache_current_len {
            let new_prices = prices
                .price
                .cents
                .height
                .collect_range_at(cache_current_len, cache_target_len);
            let new_timestamps = blocks
                .time
                .timestamp_monotonic
                .collect_range_at(cache_current_len, cache_target_len);
            self.cached_prices.extend(new_prices);
            self.cached_timestamps.extend(new_timestamps);
        }
        self.cached_price_range_max.extend(&self.cached_prices);

        // 1. Find minimum height we have data for across stateful vecs
        let current_height = Height::from(self.supply_state.len());
        debug!("supply_state.len={}", self.supply_state.len());
        debug!("utxo_cohorts.min={}", self.utxo_cohorts.min_separate_stateful_height_len());
        debug!("address_cohorts.min={}", self.address_cohorts.min_separate_stateful_height_len());
        debug!("address_indexes.min={}", self.any_address_indexes.min_stamped_height());
        debug!("addresses_data.min={}", self.addresses_data.min_stamped_height());
        debug!("addr_count.min={}", self.addr_count.min_stateful_height());
        debug!("empty_addr_count.min={}", self.empty_addr_count.min_stateful_height());
        debug!("address_activity.min={}", self.address_activity.min_stateful_height());
        debug!("coinblocks_destroyed.raw.height.len={}", self.coinblocks_destroyed.raw.height.len());
        let min_stateful = self.min_stateful_height_len();

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
                    &mut self.any_address_indexes,
                    &mut self.addresses_data,
                    &mut self.utxo_cohorts,
                    &mut self.address_cohorts,
                )?;

                if recovered.starting_height.is_zero() {
                    info!("State recovery validation failed, falling back to fresh start");
                }
                debug!(
                    "recover_state completed, starting_height={}",
                    recovered.starting_height
                );
                recovered.starting_height
            }
            StartMode::Fresh => Height::ZERO,
        };

        debug!("recovered_height={}", recovered_height);

        // Take chain_state and txindex_to_height out of self to avoid borrow conflicts
        let mut chain_state = std::mem::take(&mut self.chain_state);
        let mut txindex_to_height = std::mem::take(&mut self.txindex_to_height);

        // Recover or reuse chain_state
        let starting_height = if recovered_height.is_zero() {
            self.supply_state.reset()?;
            self.addr_count.reset_height()?;
            self.empty_addr_count.reset_height()?;
            self.address_activity.reset_height()?;
            reset_state(
                &mut self.any_address_indexes,
                &mut self.addresses_data,
                &mut self.utxo_cohorts,
                &mut self.address_cohorts,
            )?;

            chain_state.clear();
            txindex_to_height.truncate(0);

            info!("State recovery: fresh start");
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
                    price: self.cached_prices[h],
                    timestamp: self.cached_timestamps[h],
                })
                .collect();
            debug!("chain_state rebuilt");

            // Truncate RangeMap to match (entries are immutable, safe to keep)
            txindex_to_height.truncate(end);

            recovered_height
        };

        // Update starting_indexes if we need to recompute from an earlier point
        if starting_height < starting_indexes.height {
            starting_indexes.height = starting_height;
        }

        // 2b. Validate computed versions
        debug!("validating computed versions");
        let base_version = VERSION;
        self.utxo_cohorts.validate_computed_versions(base_version)?;
        self.address_cohorts
            .validate_computed_versions(base_version)?;
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

            let cached_prices = std::mem::take(&mut self.cached_prices);
            let cached_timestamps = std::mem::take(&mut self.cached_timestamps);
            let cached_price_range_max = std::mem::take(&mut self.cached_price_range_max);

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
                &mut txindex_to_height,
                &cached_prices,
                &cached_timestamps,
                &cached_price_range_max,
                exit,
            )?;

            self.cached_prices = cached_prices;
            self.cached_timestamps = cached_timestamps;
            self.cached_price_range_max = cached_price_range_max;
        }

        // Put chain_state and txindex_to_height back
        self.chain_state = chain_state;
        self.txindex_to_height = txindex_to_height;

        // 5. Compute aggregates (overlapping cohorts from separate cohorts)
        aggregates::compute_overlapping(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            starting_indexes,
            exit,
        )?;

        // 5b. Compute coinblocks_destroyed cumulative from raw
        self.coinblocks_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        // 6. Compute rest part1 (day1 mappings)
        aggregates::compute_rest_part1(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            blocks,
            prices,
            starting_indexes,
            exit,
        )?;

        // 6b. Compute address count sum (by addresstype → all)
        self.addr_count
            .compute_rest(starting_indexes, exit)?;
        self.empty_addr_count
            .compute_rest(starting_indexes, exit)?;

        // 6c. Compute total_addr_count = addr_count + empty_addr_count
        self.total_addr_count.compute(
            starting_indexes.height,
            &self.addr_count,
            &self.empty_addr_count,
            exit,
        )?;

        let window_starts = blocks.lookback.window_starts();

        self.address_activity
            .compute_rest(starting_indexes.height, &window_starts, exit)?;
        self.new_addr_count.compute(
            starting_indexes.height,
            &window_starts,
            &self.total_addr_count,
            exit,
        )?;

        self.delta.compute(
            starting_indexes.height,
            &window_starts,
            &self.addr_count,
            exit,
        )?;

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

        aggregates::compute_rest_part2(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            blocks,
            prices,
            starting_indexes,
            &height_to_market_cap,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    pub(crate) fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Get minimum length across all height-indexed stateful vectors.
    fn min_stateful_height_len(&self) -> Height {
        self.utxo_cohorts
            .min_separate_stateful_height_len()
            .min(self.address_cohorts.min_separate_stateful_height_len())
            .min(Height::from(self.supply_state.len()))
            .min(self.any_address_indexes.min_stamped_height())
            .min(self.addresses_data.min_stamped_height())
            .min(Height::from(self.addr_count.min_stateful_height()))
            .min(Height::from(self.empty_addr_count.min_stateful_height()))
            .min(Height::from(self.address_activity.min_stateful_height()))
            .min(Height::from(self.coinblocks_destroyed.raw.height.len()))
    }
}

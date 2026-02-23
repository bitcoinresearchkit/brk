use std::path::{Path, PathBuf};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Day1, EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex, Height,
    SupplyState, Version,
};
use tracing::{debug, info};
use vecdb::{
    AnyVec, BytesVec, Database, Exit, WritableVec, ImportableVec, ReadableCloneableVec,
    ReadableVec, Rw, StorageMode, LazyVecFrom1, PAGE_SIZE, Stamp,
};

use crate::{
    ComputeIndexes, blocks,
    distribution::{
        compute::{StartMode, determine_start_mode, process_blocks, recover_state, reset_state},
        state::BlockState,
    },
    indexes, inputs, outputs, prices, transactions,
};

use super::{
    AddressCohorts, AddressesDataVecs, AnyAddressIndexesVecs, UTXOCohorts,
    address::{
        AddrCountsVecs, AddressActivityVecs, GrowthRateVecs, NewAddrCountVecs, TotalAddrCountVecs,
    },
    compute::aggregates,
};

const VERSION: Version = Version::new(22);

/// Main struct holding all computed vectors and state for stateful computation.
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

    pub addr_count: AddrCountsVecs<M>,
    pub empty_addr_count: AddrCountsVecs<M>,
    pub address_activity: AddressActivityVecs<M>,

    /// Total addresses ever seen (addr_count + empty_addr_count) - lazy, global + per-type
    pub total_addr_count: TotalAddrCountVecs,
    /// New addresses per block (delta of total) - lazy height, stored day1 stats, global + per-type
    pub new_addr_count: NewAddrCountVecs<M>,
    /// Growth rate (new / addr_count) - lazy ratio with distribution stats, global + per-type
    pub growth_rate: GrowthRateVecs,

    pub fundedaddressindex:
        LazyVecFrom1<FundedAddressIndex, FundedAddressIndex, FundedAddressIndex, FundedAddressData>,
    pub emptyaddressindex:
        LazyVecFrom1<EmptyAddressIndex, EmptyAddressIndex, EmptyAddressIndex, EmptyAddressData>,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let db_path = parent.join(super::DB_NAME);
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let version = parent_version + VERSION;

        let utxo_cohorts = UTXOCohorts::forced_import(&db, version, indexes, prices, &states_path)?;

        // Create address cohorts with reference to utxo "all" cohort's supply for global ratios
        let address_cohorts = AddressCohorts::forced_import(
            &db,
            version,
            indexes,
            prices,
            &states_path,
            Some(&utxo_cohorts.all.metrics.supply),
        )?;

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
        let fundedaddressindex = LazyVecFrom1::init(
            "fundedaddressindex",
            version,
            fundedaddressindex_to_fundedaddressdata.read_only_boxed_clone(),
            |index, _| index,
        );
        let emptyaddressindex = LazyVecFrom1::init(
            "emptyaddressindex",
            version,
            emptyaddressindex_to_emptyaddressdata.read_only_boxed_clone(),
            |index, _| index,
        );

        let addr_count = AddrCountsVecs::forced_import(&db, "addr_count", version, indexes)?;
        let empty_addr_count =
            AddrCountsVecs::forced_import(&db, "empty_addr_count", version, indexes)?;
        let address_activity =
            AddressActivityVecs::forced_import(&db, "address_activity", version, indexes)?;

        // Lazy total = addr_count + empty_addr_count (global + per-type, with all derived indexes)
        let total_addr_count = TotalAddrCountVecs::forced_import(
            version,
            indexes,
            &addr_count,
            &empty_addr_count,
        )?;

        // Lazy delta of total (global + per-type)
        let new_addr_count =
            NewAddrCountVecs::forced_import(&db, version, indexes, &total_addr_count)?;

        // Growth rate: new / addr_count (global + per-type)
        let growth_rate =
            GrowthRateVecs::forced_import(version, indexes, &new_addr_count, &addr_count)?;

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
            growth_rate,

            utxo_cohorts,
            address_cohorts,

            any_address_indexes: AnyAddressIndexesVecs::forced_import(&db, version)?,
            addresses_data: AddressesDataVecs {
                funded: fundedaddressindex_to_fundedaddressdata,
                empty: emptyaddressindex_to_emptyaddressdata,
            },
            fundedaddressindex,
            emptyaddressindex,

            db,
            states_path,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

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
        starting_indexes: &mut ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Find minimum height we have data for across stateful vecs
        let current_height = Height::from(self.supply_state.len());
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

        // Fresh start: reset all state
        let (starting_height, mut chain_state) = if recovered_height.is_zero() {
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

            info!("State recovery: fresh start");
            (Height::ZERO, vec![])
        } else {
            // Recover chain_state from stored values
            debug!("recovering chain_state from stored values");
            let height_to_timestamp = &blocks.time.timestamp_monotonic;
            let height_to_price = &prices.cents.price;

            let end = usize::from(recovered_height);
            let timestamp_data: Vec<_> = height_to_timestamp.collect_range_at(0, end);
            let price_data: Vec<_> = height_to_price.collect_range_at(0, end);

            debug!("building supply_state vec for {} heights", recovered_height);
            let supply_state_data: Vec<_> = self.supply_state.collect_range_at(0, end);
            let chain_state = supply_state_data
                .into_iter()
                .enumerate()
                .map(|(h, supply)| BlockState {
                    supply,
                    price: price_data[h],
                    timestamp: timestamp_data[h],
                })
                .collect();
            debug!("chain_state vec built");

            (recovered_height, chain_state)
        };

        // Update starting_indexes if we need to recompute from an earlier point
        if starting_height < starting_indexes.height {
            starting_indexes.height = starting_height;
            // Also update day1 to match
            if starting_height.is_zero() {
                starting_indexes.day1 = Day1::from(0);
            } else {
                starting_indexes.day1 = indexes
                    .height
                    .day1
                    .collect_one(starting_height.decremented().unwrap())
                    .unwrap();
            }
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
            process_blocks(
                self,
                indexer,
                indexes,
                inputs,
                outputs,
                transactions,
                blocks,
                prices,
                starting_height,
                last_height,
                &mut chain_state,
                exit,
            )?;
        }

        // 5. Compute aggregates (overlapping cohorts from separate cohorts)
        aggregates::compute_overlapping(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            starting_indexes,
            exit,
        )?;

        // 6. Compute rest part1 (day1 mappings)
        aggregates::compute_rest_part1(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            blocks,
            prices,
            starting_indexes,
            exit,
        )?;

        // 6b. Compute address count day1 vecs (by addresstype + all)
        self.addr_count
            .compute_rest(blocks, starting_indexes, exit)?;
        self.empty_addr_count
            .compute_rest(blocks, starting_indexes, exit)?;

        // 6d. Compute new_addr_count cumulative (height is lazy delta)
        self.new_addr_count
            .compute_cumulative(starting_indexes, exit)?;

        // 7. Compute rest part2 (relative metrics)
        let supply_metrics = &self.utxo_cohorts.all.metrics.supply;

        let height_to_market_cap = supply_metrics.total.usd.height.clone();

        aggregates::compute_rest_part2(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            blocks,
            prices,
            starting_indexes,
            Some(&height_to_market_cap),
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
    }

}

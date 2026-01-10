use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    EmptyAddressData, EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex,
    SupplyState, Version,
};
use tracing::info;
use vecdb::{
    AnyVec, BytesVec, Database, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    LazyVecFrom1, PAGE_SIZE, Stamp, TypedVecIterator, VecIndex,
};

use crate::{
    ComputeIndexes, blocks,
    distribution::{
        compute::{StartMode, determine_start_mode, process_blocks, recover_state, reset_state},
        state::BlockState,
    },
    indexes, inputs, outputs, price, transactions,
};

use super::{
    AddressCohorts, AddressesDataVecs, AnyAddressIndexesVecs, UTXOCohorts, address::AddrCountVecs,
    compute::aggregates,
};

const VERSION: Version = Version::new(21);

/// Main struct holding all computed vectors and state for stateful computation.
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    db: Database,

    pub chain_state: BytesVec<Height, SupplyState>,
    pub any_address_indexes: AnyAddressIndexesVecs,
    pub addresses_data: AddressesDataVecs,
    pub utxo_cohorts: UTXOCohorts,
    pub address_cohorts: AddressCohorts,

    pub addr_count: AddrCountVecs,
    pub empty_addr_count: AddrCountVecs,
    pub loadedaddressindex:
        LazyVecFrom1<LoadedAddressIndex, LoadedAddressIndex, LoadedAddressIndex, LoadedAddressData>,
    pub emptyaddressindex:
        LazyVecFrom1<EmptyAddressIndex, EmptyAddressIndex, EmptyAddressIndex, EmptyAddressData>,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db_path = parent.join(super::DB_NAME);
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let version = parent_version + VERSION;

        let utxo_cohorts = UTXOCohorts::forced_import(&db, version, indexes, price, &states_path)?;

        // Create address cohorts with reference to utxo "all" cohort's supply for global ratios
        let address_cohorts = AddressCohorts::forced_import(
            &db,
            version,
            indexes,
            price,
            &states_path,
            Some(&utxo_cohorts.all.metrics.supply),
        )?;

        // Create address data BytesVecs first so we can also use them for identity mappings
        let loadedaddressindex_to_loadedaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "loadedaddressdata", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let emptyaddressindex_to_emptyaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "emptyaddressdata", version)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;

        // Identity mappings for traversable
        let loadedaddressindex = LazyVecFrom1::init(
            "loadedaddressindex",
            version,
            loadedaddressindex_to_loadedaddressdata.boxed_clone(),
            |index, _| Some(index),
        );
        let emptyaddressindex = LazyVecFrom1::init(
            "emptyaddressindex",
            version,
            emptyaddressindex_to_emptyaddressdata.boxed_clone(),
            |index, _| Some(index),
        );

        let this = Self {
            chain_state: BytesVec::forced_import_with(
                vecdb::ImportOptions::new(&db, "chain", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            addr_count: AddrCountVecs::forced_import(&db, "addr_count", version, indexes)?,
            empty_addr_count: AddrCountVecs::forced_import(
                &db,
                "empty_addr_count",
                version,
                indexes,
            )?,

            utxo_cohorts,
            address_cohorts,

            any_address_indexes: AnyAddressIndexesVecs::forced_import(&db, version)?,
            addresses_data: AddressesDataVecs {
                loaded: loadedaddressindex_to_loadedaddressdata,
                empty: emptyaddressindex_to_emptyaddressdata,
            },
            loadedaddressindex,
            emptyaddressindex,

            db,
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
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        transactions: &transactions::Vecs,
        blocks: &blocks::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &mut ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Find minimum height we have data for across stateful vecs
        let current_height = Height::from(self.chain_state.len());
        let height_based_min = self.min_stateful_height_len();
        let dateindex_min = self.min_stateful_dateindex_len();
        let min_stateful = adjust_for_dateindex_gap(height_based_min, dateindex_min, indexes)?;

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
                let chain_state_rollback = self.chain_state.rollback_before(stamp);

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
                recovered.starting_height
            }
            StartMode::Fresh => Height::ZERO,
        };

        // Fresh start: reset all state
        let (starting_height, mut chain_state) = if recovered_height.is_zero() {
            self.chain_state.reset()?;
            self.addr_count.reset_height()?;
            self.empty_addr_count.reset_height()?;
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
            let height_to_timestamp = &blocks.time.timestamp_fixed;
            let height_to_price = price.map(|p| &p.usd.split.close.height);

            let mut height_to_timestamp_iter = height_to_timestamp.into_iter();
            let mut height_to_price_iter = height_to_price.map(|v| v.into_iter());
            let mut chain_state_iter = self.chain_state.into_iter();

            let chain_state = (0..recovered_height.to_usize())
                .map(|h| {
                    let h = Height::from(h);
                    BlockState {
                        supply: chain_state_iter.get_unwrap(h),
                        price: height_to_price_iter.as_mut().map(|v| *v.get_unwrap(h)),
                        timestamp: height_to_timestamp_iter.get_unwrap(h),
                    }
                })
                .collect();

            (recovered_height, chain_state)
        };

        // 2b. Validate computed versions
        let base_version = VERSION;
        self.utxo_cohorts.validate_computed_versions(base_version)?;
        self.address_cohorts
            .validate_computed_versions(base_version)?;

        // 3. Get last height from indexer
        let last_height = Height::from(indexer.vecs.blocks.blockhash.len().saturating_sub(1));

        // 4. Process blocks
        if starting_height <= last_height {
            process_blocks(
                self,
                indexer,
                indexes,
                inputs,
                outputs,
                transactions,
                blocks,
                price,
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

        // 6. Compute rest part1 (dateindex mappings)
        aggregates::compute_rest_part1(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            indexes,
            price,
            starting_indexes,
            exit,
        )?;

        // 6b. Compute address count dateindex vecs (by addresstype + all)
        self.addr_count
            .compute_rest(indexes, starting_indexes, exit)?;
        self.empty_addr_count
            .compute_rest(indexes, starting_indexes, exit)?;

        // 7. Compute rest part2 (relative metrics)
        let supply_metrics = &self.utxo_cohorts.all.metrics.supply;

        let height_to_market_cap = supply_metrics
            .total
            .dollars
            .as_ref()
            .map(|d| d.height.clone());

        let dateindex_to_market_cap = supply_metrics
            .total
            .dollars
            .as_ref()
            .map(|d| d.dateindex.0.clone());

        let height_to_market_cap_ref = height_to_market_cap.as_ref();
        let dateindex_to_market_cap_ref = dateindex_to_market_cap.as_ref();

        aggregates::compute_rest_part2(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            indexes,
            price,
            starting_indexes,
            height_to_market_cap_ref,
            dateindex_to_market_cap_ref,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Get minimum length across all height-indexed stateful vectors.
    fn min_stateful_height_len(&self) -> Height {
        self.utxo_cohorts
            .min_separate_stateful_height_len()
            .min(self.address_cohorts.min_separate_stateful_height_len())
            .min(Height::from(self.chain_state.len()))
            .min(self.any_address_indexes.min_stamped_height())
            .min(self.addresses_data.min_stamped_height())
            .min(Height::from(self.addr_count.min_len()))
            .min(Height::from(self.empty_addr_count.min_len()))
    }

    /// Get minimum length across all dateindex-indexed stateful vectors.
    fn min_stateful_dateindex_len(&self) -> usize {
        self.utxo_cohorts
            .min_separate_stateful_dateindex_len()
            .min(self.utxo_cohorts.min_aggregate_stateful_dateindex_len())
            .min(self.address_cohorts.min_separate_stateful_dateindex_len())
    }
}

/// Adjust start height if dateindex vecs are behind where they should be.
///
/// To resume at height H (in day D), we need days 0..D-1 complete in dateindex vecs.
/// If dateindex vecs only have length N < D, restart from the first height of day N.
fn adjust_for_dateindex_gap(
    height_based_min: Height,
    dateindex_min: usize,
    indexes: &indexes::Vecs,
) -> Result<Height> {
    // Skip check if no dateindex vecs exist or starting from zero
    if dateindex_min == usize::MAX || height_based_min.is_zero() {
        return Ok(height_based_min);
    }

    // Skip if height.dateindex doesn't cover height_based_min yet
    if height_based_min.to_usize() >= indexes.height.dateindex.len() {
        return Ok(height_based_min);
    }

    // Get the dateindex at the height we want to resume at
    let required_dateindex: usize = indexes.height.dateindex.read_once(height_based_min)?.into();

    // If dateindex vecs are behind, restart from first height of the missing day
    if dateindex_min < required_dateindex && dateindex_min < indexes.dateindex.first_height.len() {
        Ok(indexes
            .dateindex
            .first_height
            .read_once(dateindex_min.into())?)
    } else {
        Ok(height_based_min)
    }
}

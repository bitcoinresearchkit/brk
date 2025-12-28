use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Dollars, EmptyAddressData, EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex,
    Sats, StoredU64, Version,
};
use log::info;
use vecdb::{
    AnyVec, BytesVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec,
    IterableCloneableVec, LazyVecFrom1, PAGE_SIZE, PcoVec, Stamp, TypedVecIterator, VecIndex,
};

use crate::{
    Indexes, chain,
    grouped::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes, price,
    stateful::{
        compute::{StartMode, determine_start_mode, process_blocks, recover_state, reset_state},
        states::BlockState,
    },
    txins,
    utils::OptionExt,
};

use super::{
    AddressCohorts, AddressesDataVecs, AnyAddressIndexesVecs, SupplyState, UTXOCohorts,
    address::{AddressTypeToHeightToAddressCount, AddressTypeToIndexesToAddressCount},
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

    pub height_to_unspendable_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_opreturn_supply: EagerVec<PcoVec<Height, Sats>>,
    pub addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount,
    pub addresstype_to_height_to_empty_addr_count: AddressTypeToHeightToAddressCount,

    pub addresstype_to_indexes_to_addr_count: AddressTypeToIndexesToAddressCount,
    pub addresstype_to_indexes_to_empty_addr_count: AddressTypeToIndexesToAddressCount,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    pub indexes_to_opreturn_supply: ComputedValueVecsFromHeight,
    pub indexes_to_addr_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_empty_addr_count: ComputedVecsFromHeight<StoredU64>,
    pub height_to_market_cap: Option<LazyVecFrom1<Height, Dollars, Height, Dollars>>,
    pub indexes_to_market_cap: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub loadedaddressindex_to_loadedaddressindex:
        LazyVecFrom1<LoadedAddressIndex, LoadedAddressIndex, LoadedAddressIndex, LoadedAddressData>,
    pub emptyaddressindex_to_emptyaddressindex:
        LazyVecFrom1<EmptyAddressIndex, EmptyAddressIndex, EmptyAddressIndex, EmptyAddressData>,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db_path = parent.join(super::DB_NAME);
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let compute_dollars = price.is_some();
        let v0 = version + VERSION + Version::ZERO;
        let v2 = version + VERSION + Version::TWO;

        let utxo_cohorts = UTXOCohorts::forced_import(&db, version, indexes, price, &states_path)?;

        // Create address data BytesVecs first so we can also use them for identity mappings
        let loadedaddressindex_to_loadedaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "loadedaddressdata", v0)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let emptyaddressindex_to_emptyaddressdata = BytesVec::forced_import_with(
            vecdb::ImportOptions::new(&db, "emptyaddressdata", v0)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;

        // Identity mappings for traversable
        let loadedaddressindex_to_loadedaddressindex = LazyVecFrom1::init(
            "loadedaddressindex",
            v0,
            loadedaddressindex_to_loadedaddressdata.boxed_clone(),
            |index, _| Some(index),
        );
        let emptyaddressindex_to_emptyaddressindex = LazyVecFrom1::init(
            "emptyaddressindex",
            v0,
            emptyaddressindex_to_emptyaddressdata.boxed_clone(),
            |index, _| Some(index),
        );

        let height_to_unspendable_supply: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(&db, "unspendable_supply", v0)?;
        let indexes_to_unspendable_supply = ComputedValueVecsFromHeight::forced_import(
            &db,
            "unspendable_supply",
            Source::Vec(height_to_unspendable_supply.boxed_clone()),
            v0,
            VecBuilderOptions::default().add_last(),
            compute_dollars,
            indexes,
        )?;

        let height_to_opreturn_supply: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(&db, "opreturn_supply", v0)?;
        let indexes_to_opreturn_supply = ComputedValueVecsFromHeight::forced_import(
            &db,
            "opreturn_supply",
            Source::Vec(height_to_opreturn_supply.boxed_clone()),
            v0,
            VecBuilderOptions::default().add_last(),
            compute_dollars,
            indexes,
        )?;

        let this = Self {
            chain_state: BytesVec::forced_import_with(
                vecdb::ImportOptions::new(&db, "chain", v0)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            height_to_unspendable_supply,
            indexes_to_unspendable_supply,
            height_to_opreturn_supply,
            indexes_to_opreturn_supply,

            indexes_to_addr_count: ComputedVecsFromHeight::forced_import(
                &db,
                "addr_count",
                Source::Compute,
                v0,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_empty_addr_count: ComputedVecsFromHeight::forced_import(
                &db,
                "empty_addr_count",
                Source::Compute,
                v0,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            height_to_market_cap: compute_dollars.then(|| {
                LazyVecFrom1::init(
                    "market_cap",
                    v0,
                    utxo_cohorts
                        .all
                        .metrics
                        .supply
                        .height_to_supply_value
                        .dollars
                        .as_ref()
                        .unwrap()
                        .boxed_clone(),
                    |height: Height, iter| iter.get(height),
                )
            }),

            indexes_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    "market_cap",
                    Source::Compute,
                    v2,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),

            addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount::forced_import(
                &db,
                "addr_count",
                v0,
            )?,
            addresstype_to_height_to_empty_addr_count:
                AddressTypeToHeightToAddressCount::forced_import(&db, "empty_addr_count", v0)?,
            addresstype_to_indexes_to_addr_count:
                AddressTypeToIndexesToAddressCount::forced_import(&db, "addr_count", v0, indexes)?,
            addresstype_to_indexes_to_empty_addr_count:
                AddressTypeToIndexesToAddressCount::forced_import(
                    &db,
                    "empty_addr_count",
                    v0,
                    indexes,
                )?,

            utxo_cohorts,

            address_cohorts: AddressCohorts::forced_import(
                &db,
                version,
                indexes,
                price,
                &states_path,
            )?,

            any_address_indexes: AnyAddressIndexesVecs::forced_import(&db, v0)?,
            addresses_data: AddressesDataVecs {
                loaded: loadedaddressindex_to_loadedaddressdata,
                empty: emptyaddressindex_to_emptyaddressdata,
            },
            loadedaddressindex_to_loadedaddressindex,
            emptyaddressindex_to_emptyaddressindex,

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
        txins: &txins::Vecs,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &mut Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Find minimum computed height for recovery
        let chain_state_height = Height::from(self.chain_state.len());

        // Get minimum heights without holding mutable references
        let utxo_min = self.utxo_cohorts.min_separate_height_vecs_len();
        let address_min = self.address_cohorts.min_separate_height_vecs_len();

        let stateful_min = utxo_min
            .min(address_min)
            .min(Height::from(self.chain_state.len()))
            .min(self.any_address_indexes.min_stamped_height())
            .min(self.addresses_data.min_stamped_height())
            .min(Height::from(self.height_to_unspendable_supply.len()))
            .min(Height::from(self.height_to_opreturn_supply.len()))
            .min(Height::from(
                self.addresstype_to_height_to_addr_count.min_len(),
            ))
            .min(Height::from(
                self.addresstype_to_height_to_empty_addr_count.min_len(),
            ));

        // 2. Determine start mode and recover/reset state
        let start_mode = determine_start_mode(stateful_min, chain_state_height);

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
            self.height_to_unspendable_supply.reset()?;
            self.height_to_opreturn_supply.reset()?;
            self.addresstype_to_height_to_addr_count.reset()?;
            self.addresstype_to_height_to_empty_addr_count.reset()?;
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
            let height_to_timestamp = &indexes.height_to_timestamp_fixed;
            let height_to_price = price.map(|p| &p.chainindexes_to_price_close.height);

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

            info!(
                "State recovery: resumed from checkpoint at height {}",
                recovered_height
            );
            (recovered_height, chain_state)
        };

        // 2b. Validate computed versions
        let base_version = VERSION;
        self.utxo_cohorts.validate_computed_versions(base_version)?;
        self.address_cohorts
            .validate_computed_versions(base_version)?;

        // 3. Get last height from indexer
        let last_height = Height::from(
            indexer
                .vecs
                .block
                .height_to_blockhash
                .len()
                .saturating_sub(1),
        );

        // 4. Process blocks
        if starting_height <= last_height {
            process_blocks(
                self,
                indexer,
                indexes,
                txins,
                chain,
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

        // 7. Compute indexes_to_market_cap from dateindex supply
        if let Some(indexes_to_market_cap) = self.indexes_to_market_cap.as_mut() {
            indexes_to_market_cap.compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.utxo_cohorts
                        .all
                        .metrics
                        .supply
                        .indexes_to_supply
                        .dollars
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, v),
                    exit,
                )?;
                Ok(())
            })?;
        }

        // 7b. Compute indexes for unspendable and opreturn supply
        self.indexes_to_unspendable_supply.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_unspendable_supply),
        )?;
        self.indexes_to_opreturn_supply.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_opreturn_supply),
        )?;

        // 8. Compute rest part2 (relative metrics)
        let height_to_supply = &self
            .utxo_cohorts
            .all
            .metrics
            .supply
            .height_to_supply_value
            .bitcoin
            .clone();

        let dateindex_to_supply = self
            .utxo_cohorts
            .all
            .metrics
            .supply
            .indexes_to_supply
            .bitcoin
            .dateindex
            .clone();

        let height_to_market_cap = self.height_to_market_cap.clone();

        let dateindex_to_market_cap = self
            .indexes_to_market_cap
            .as_ref()
            .map(|v| v.dateindex.u().clone());

        let dateindex_to_supply_ref = dateindex_to_supply.u();
        let height_to_market_cap_ref = height_to_market_cap.as_ref();
        let dateindex_to_market_cap_ref = dateindex_to_market_cap.as_ref();

        aggregates::compute_rest_part2(
            &mut self.utxo_cohorts,
            &mut self.address_cohorts,
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply_ref,
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
}

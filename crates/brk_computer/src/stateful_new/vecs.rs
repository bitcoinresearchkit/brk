//! Main Vecs struct for stateful computation.

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, StoredU64, Version};
use vecdb::{
    BytesVec, Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, LazyVecFrom1,
    PAGE_SIZE, PcoVec,
};

use crate::{
    Indexes, SupplyState, chain,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes, price,
    utils::OptionExt,
};

use super::{
    AddressCohorts, AddressesDataVecs, AnyAddressIndexesVecs, UTXOCohorts,
    address::{AddressTypeToHeightToAddressCount, AddressTypeToIndexesToAddressCount},
};

const VERSION: Version = Version::new(21);

/// Main struct holding all computed vectors and state for stateful computation.
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    db: Database,

    // ---
    // States
    // ---
    pub chain_state: BytesVec<Height, SupplyState>,
    pub any_address_indexes: AnyAddressIndexesVecs,
    pub addresses_data: AddressesDataVecs,
    pub utxo_cohorts: UTXOCohorts,
    pub address_cohorts: AddressCohorts,

    pub height_to_unspendable_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_opreturn_supply: EagerVec<PcoVec<Height, Sats>>,
    pub addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount,
    pub addresstype_to_height_to_empty_addr_count: AddressTypeToHeightToAddressCount,

    // ---
    // Computed
    // ---
    pub addresstype_to_indexes_to_addr_count: AddressTypeToIndexesToAddressCount,
    pub addresstype_to_indexes_to_empty_addr_count: AddressTypeToIndexesToAddressCount,
    pub indexes_to_addr_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_empty_addr_count: ComputedVecsFromHeight<StoredU64>,
    pub height_to_market_cap: Option<LazyVecFrom1<Height, Dollars, Height, Dollars>>,
    pub indexes_to_market_cap: Option<ComputedVecsFromDateIndex<Dollars>>,
}

const SAVED_STAMPED_CHANGES: u16 = 10;

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db_path = parent.join("stateful");
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let compute_dollars = price.is_some();
        let v0 = version + VERSION + Version::ZERO;
        let v2 = version + VERSION + Version::TWO;

        let utxo_cohorts = UTXOCohorts::forced_import(&db, version, indexes, price, &states_path)?;

        Ok(Self {
            chain_state: BytesVec::forced_import_with(
                vecdb::ImportOptions::new(&db, "chain", v0)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            height_to_unspendable_supply: EagerVec::forced_import(&db, "unspendable_supply", v0)?,
            height_to_opreturn_supply: EagerVec::forced_import(&db, "opreturn_supply", v0)?,

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
            addresses_data: AddressesDataVecs::forced_import(&db, v0)?,

            db,
        })
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
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &mut Indexes,
        exit: &Exit,
    ) -> Result<()> {
        use super::compute::{
            StartMode, determine_start_mode, process_blocks,
        };
        use crate::states::BlockState;
        use vecdb::{AnyVec, GenericStoredVec, Stamp, TypedVecIterator, VecIndex};

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
            .min(Height::from(self.height_to_opreturn_supply.len()));

        // 2. Determine start mode and recover state
        let start_mode = determine_start_mode(stateful_min, chain_state_height);

        let (starting_height, mut chain_state) = match start_mode {
            StartMode::Resume(height) => {
                let stamp = Stamp::from(height);

                // Rollback state vectors
                let _ = self.chain_state.rollback_before(stamp);
                let _ = self.any_address_indexes.rollback_before(stamp);
                let _ = self.addresses_data.rollback_before(stamp);

                // Import cohort states
                self.utxo_cohorts.import_separate_states(height);
                self.address_cohorts.import_separate_states(height);

                // Import aggregate price_to_amount
                let _ = self.utxo_cohorts.import_aggregate_price_to_amount(height);

                // Recover chain_state from stored values
                let chain_state = if !height.is_zero() {
                    let height_to_timestamp = &indexes.height_to_timestamp_fixed;
                    let height_to_price = price.map(|p| &p.chainindexes_to_price_close.height);

                    let mut height_to_timestamp_iter = height_to_timestamp.into_iter();
                    let mut height_to_price_iter = height_to_price.map(|v| v.into_iter());
                    let mut chain_state_iter = self.chain_state.into_iter();

                    (0..height.to_usize())
                        .map(|h| {
                            let h = Height::from(h);
                            BlockState {
                                supply: chain_state_iter.get_unwrap(h),
                                price: height_to_price_iter.as_mut().map(|v| *v.get_unwrap(h)),
                                timestamp: height_to_timestamp_iter.get_unwrap(h),
                            }
                        })
                        .collect()
                } else {
                    vec![]
                };

                (height, chain_state)
            }
            StartMode::Fresh => {
                // Reset all state
                self.any_address_indexes.reset()?;
                self.addresses_data.reset()?;

                // Reset state heights
                self.utxo_cohorts.reset_separate_state_heights();
                self.address_cohorts.reset_separate_state_heights();

                // Reset price_to_amount for all separate cohorts
                self.utxo_cohorts.reset_separate_price_to_amount()?;
                self.address_cohorts.reset_separate_price_to_amount()?;

                // Reset aggregate cohorts' price_to_amount
                self.utxo_cohorts.reset_aggregate_price_to_amount()?;

                (Height::ZERO, vec![])
            }
        };

        // 3. Get last height from indexer
        let last_height = Height::from(indexer.vecs.height_to_blockhash.len().saturating_sub(1));

        // 4. Process blocks
        if starting_height <= last_height {
            process_blocks(
                self,
                indexer,
                indexes,
                chain,
                price,
                starting_height,
                last_height,
                &mut chain_state,
                exit,
            )?;
        }

        // 5. Compute aggregates (overlapping cohorts from separate cohorts)
        self.utxo_cohorts
            .compute_overlapping_vecs(starting_indexes, exit)?;
        self.address_cohorts
            .compute_overlapping_vecs(starting_indexes, exit)?;

        // 6. Compute rest part1 (dateindex mappings)
        self.utxo_cohorts
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;
        self.address_cohorts
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;

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

        let height_to_realized_cap = self
            .utxo_cohorts
            .all
            .metrics
            .realized
            .as_ref()
            .map(|r| r.height_to_realized_cap.clone());

        let dateindex_to_realized_cap = self
            .utxo_cohorts
            .all
            .metrics
            .realized
            .as_ref()
            .map(|r| r.indexes_to_realized_cap.dateindex.unwrap_last().clone());

        let dateindex_to_supply_ref = dateindex_to_supply.u();
        let height_to_market_cap_ref = height_to_market_cap.as_ref();
        let dateindex_to_market_cap_ref = dateindex_to_market_cap.as_ref();
        let height_to_realized_cap_ref = height_to_realized_cap.as_ref();
        let dateindex_to_realized_cap_ref = dateindex_to_realized_cap.as_ref();

        self.utxo_cohorts.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply_ref,
            height_to_market_cap_ref,
            dateindex_to_market_cap_ref,
            height_to_realized_cap_ref,
            dateindex_to_realized_cap_ref,
            exit,
        )?;

        self.address_cohorts.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply_ref,
            height_to_market_cap_ref,
            dateindex_to_market_cap_ref,
            height_to_realized_cap_ref,
            dateindex_to_realized_cap_ref,
            exit,
        )?;

        self.db.compact()?;
        Ok(())
    }
}

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
    ///
    /// NOTE: This is a placeholder. The full implementation needs to be ported
    /// from stateful/mod.rs once all the supporting methods on UTXOCohorts,
    /// AddressCohorts, and state types are implemented.
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        _indexer: &Indexer,
        _indexes: &indexes::Vecs,
        _chain: &chain::Vecs,
        _price: Option<&price::Vecs>,
        _starting_indexes: &mut Indexes,
        _exit: &Exit,
    ) -> Result<()> {
        // The full compute implementation requires these methods to be implemented:
        //
        // On UTXOCohorts:
        // - tick_tock_next_block(&chain_state, timestamp)
        // - receive(transacted, height, price)
        // - send(height_to_sent, &mut chain_state)
        // - truncate_push_aggregate_percentiles(height)
        // - import_aggregate_price_to_amount(height)
        // - reset_aggregate_price_to_amount()
        //
        // On UTXOCohortState:
        // - reset_block_values()
        // - reset_price_to_amount()
        //
        // On AddressCohortState:
        // - inner.reset_block_values()
        // - inner.reset_price_to_amount()
        //
        // On AddressTypeToHeightToAddressCount:
        // - safe_flush(exit)
        // - truncate_push(height, &count)
        //
        // See stateful/mod.rs:368-1397 for the full implementation.
        //
        // The basic structure is:
        // 1. Validate computed versions against base version
        // 2. Find min stateful height and recover state
        // 3. For each block:
        //    a. Reset per-block values
        //    b. Process outputs in parallel (receive)
        //    c. Process inputs in parallel (send)
        //    d. Push to height-indexed vectors
        //    e. Flush checkpoint every 10,000 blocks
        // 4. Compute aggregate cohorts from separate cohorts
        // 5. Compute rest_part1 (dateindex mappings)
        // 6. Compute rest_part2 (ratios and relative metrics)

        self.db.compact()?;
        Ok(())
    }
}

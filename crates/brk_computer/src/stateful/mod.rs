//! Stateful computation module for Bitcoin UTXO and address cohort analysis.
//!
//! This module contains the main computation loop that processes blocks and computes
//! various metrics for UTXO cohorts (grouped by age, amount, etc.) and address cohorts.
//!
//! ## Architecture
//!
//! The module is organized as follows:
//!
//! - **`Vecs`**: Main struct holding all computed vectors and state
//! - **Cohort Types**:
//!   - **Separate cohorts**: Have full state tracking (e.g., UTXOs 1-2 years old)
//!   - **Aggregate cohorts**: Computed from separate cohorts (e.g., all, sth, lth)
//!
//! ## Checkpoint/Resume
//!
//! The computation supports checkpointing via `flush_states()` which saves:
//! - All separate cohorts' state (via `safe_flush_stateful_vecs`)
//! - Aggregate cohorts' `price_to_amount` (via `HeightFlushable` trait)
//! - Aggregate cohorts' `price_percentiles` (via `Flushable` trait)
//!
//! Resume is handled by:
//! - `import_state()` for separate cohorts
//! - `import_aggregate_price_to_amount()` for aggregate cohorts
//!
//! ## Key Traits
//!
//! - `Flushable`: Simple flush operations (no height tracking)
//! - `HeightFlushable`: Height-indexed state (flush, import, reset)

use std::{cmp::Ordering, collections::BTreeSet, mem, path::Path, thread};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    AnyAddressDataIndexEnum, AnyAddressIndex, DateIndex, Dollars, EmptyAddressData,
    EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex, OutputType, Sats, StoredU64,
    TxInIndex, TxIndex, TxOutIndex, TypeIndex, Version,
};
use log::info;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, CollectableVec, Database, EagerVec, Exit, GenericStoredVec,
    ImportOptions, ImportableVec, IterableCloneableVec, IterableVec, LazyVecFrom1, PAGE_SIZE,
    PcoVec, Stamp, TypedVecIterator, VecIndex,
};

use crate::{
    BlockState, Indexes, SupplyState, Transacted, chain,
    grouped::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes, price,
    utils::OptionExt,
};

mod address_cohort;
mod address_cohorts;
mod address_indexes;
mod addresstype;
mod common;
mod flushable;
mod range_map;
mod readers;
mod r#trait;
mod transaction_processing;
mod utxo_cohort;
mod utxo_cohorts;
mod withaddressdatasource;

pub use crate::states::{Flushable, HeightFlushable};

use address_indexes::{AddressesDataVecs, AnyAddressIndexesVecs};
use addresstype::*;
use range_map::*;
use readers::{
    IndexerReaders, VecsReaders, build_txinindex_to_txindex, build_txoutindex_to_txindex,
};
use r#trait::*;
use withaddressdatasource::*;

type TxIndexVec = SmallVec<[TxIndex; 4]>;

const VERSION: Version = Version::new(21);

const BIP30_DUPLICATE_COINBASE_HEIGHT_1: u32 = 91_842;
const BIP30_DUPLICATE_COINBASE_HEIGHT_2: u32 = 91_880;
const BIP30_ORIGINAL_COINBASE_HEIGHT_1: u32 = 91_812;
const BIP30_ORIGINAL_COINBASE_HEIGHT_2: u32 = 91_722;
const FLUSH_INTERVAL: usize = 10_000;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    // ---
    // States
    // ---
    pub chain_state: BytesVec<Height, SupplyState>,
    pub any_address_indexes: AnyAddressIndexesVecs,
    pub addresses_data: AddressesDataVecs,
    pub utxo_cohorts: utxo_cohorts::Vecs,
    pub address_cohorts: address_cohorts::Vecs,

    pub height_to_unspendable_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_opreturn_supply: EagerVec<PcoVec<Height, Sats>>,
    pub addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount,
    pub addresstype_to_height_to_empty_addr_count: AddressTypeToHeightToAddressCount,

    // ---
    // Computed
    // ---
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
        let db_path = parent.join("stateful");
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let compute_dollars = price.is_some();
        let v0 = version + VERSION + Version::ZERO;
        let v1 = version + VERSION + Version::ONE;
        let v2 = version + VERSION + Version::TWO;

        let utxo_cohorts =
            utxo_cohorts::Vecs::forced_import(&db, version, indexes, price, &states_path)?;

        let loadedaddressindex_to_loadedaddressdata = BytesVec::forced_import_with(
            ImportOptions::new(&db, "loadedaddressdata", v0)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let emptyaddressindex_to_emptyaddressdata = BytesVec::forced_import_with(
            ImportOptions::new(&db, "emptyaddressdata", v0)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
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

        let this = Self {
            chain_state: BytesVec::forced_import_with(
                ImportOptions::new(&db, "chain", v0)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            height_to_unspendable_supply: EagerVec::forced_import(&db, "unspendable_supply", v0)?,
            indexes_to_unspendable_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "unspendable_supply",
                Source::None,
                v0,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            height_to_opreturn_supply: EagerVec::forced_import(&db, "opreturn_supply", v0)?,
            indexes_to_opreturn_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "opreturn_supply",
                Source::None,
                v0,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
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
                    v1,
                    utxo_cohorts
                        .all
                        .inner
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
            addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount::from(
                ByAddressType::new_with_name(|name| {
                    Ok(EagerVec::forced_import(
                        &db,
                        &format!("{name}_addr_count"),
                        v0,
                    )?)
                })?,
            ),
            addresstype_to_height_to_empty_addr_count: AddressTypeToHeightToAddressCount::from(
                ByAddressType::new_with_name(|name| {
                    Ok(EagerVec::forced_import(
                        &db,
                        &format!("{name}_empty_addr_count"),
                        v0,
                    )?)
                })?,
            ),
            addresstype_to_indexes_to_addr_count: AddressTypeToIndexesToAddressCount::from(
                ByAddressType::new_with_name(|name| {
                    ComputedVecsFromHeight::forced_import(
                        &db,
                        &format!("{name}_addr_count"),
                        Source::None,
                        v0,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                })?,
            ),
            addresstype_to_indexes_to_empty_addr_count: AddressTypeToIndexesToAddressCount::from(
                ByAddressType::new_with_name(|name| {
                    ComputedVecsFromHeight::forced_import(
                        &db,
                        &format!("{name}_empty_addr_count"),
                        Source::None,
                        v0,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                })?,
            ),
            utxo_cohorts,
            address_cohorts: address_cohorts::Vecs::forced_import(
                &db,
                version,
                indexes,
                price,
                &states_path,
            )?,

            any_address_indexes: AnyAddressIndexesVecs {
                p2a: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pk33: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pk65: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pkh: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2sh: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2tr: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2wpkh: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2wsh: BytesVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", v0)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
            },
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

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        // Must take ownership as its indexes will be updated for this specific function
        starting_indexes: &mut Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, chain, price, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        // Must take ownership as its indexes will be updated for this specific function
        starting_indexes: &mut Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dateindex_to_first_height = &indexes.dateindex_to_first_height;
        let dateindex_to_height_count = &indexes.dateindex_to_height_count;
        let dateindex_to_price_close = price
            .as_ref()
            .map(|price| price.timeindexes_to_price_close.dateindex.u());
        let height_to_date_fixed = &indexes.height_to_date_fixed;
        let height_to_first_p2aaddressindex = &indexer.vecs.address.height_to_first_p2aaddressindex;
        let height_to_first_p2pk33addressindex = &indexer.vecs.address.height_to_first_p2pk33addressindex;
        let height_to_first_p2pk65addressindex = &indexer.vecs.address.height_to_first_p2pk65addressindex;
        let height_to_first_p2pkhaddressindex = &indexer.vecs.address.height_to_first_p2pkhaddressindex;
        let height_to_first_p2shaddressindex = &indexer.vecs.address.height_to_first_p2shaddressindex;
        let height_to_first_p2traddressindex = &indexer.vecs.address.height_to_first_p2traddressindex;
        let height_to_first_p2wpkhaddressindex = &indexer.vecs.address.height_to_first_p2wpkhaddressindex;
        let height_to_first_p2wshaddressindex = &indexer.vecs.address.height_to_first_p2wshaddressindex;
        let height_to_first_txindex = &indexer.vecs.tx.height_to_first_txindex;
        let height_to_txindex_count = chain.indexes_to_tx_count.height.u();
        let height_to_first_txinindex = &indexer.vecs.txin.height_to_first_txinindex;
        let height_to_first_txoutindex = &indexer.vecs.txout.height_to_first_txoutindex;
        let height_to_input_count = chain.indexes_to_input_count.height.unwrap_sum();
        let height_to_output_count = chain.indexes_to_output_count.height.unwrap_sum();
        let height_to_price_close = price
            .as_ref()
            .map(|price| &price.chainindexes_to_price_close.height);
        let height_to_timestamp_fixed = &indexes.height_to_timestamp_fixed;
        let height_to_tx_count = chain.indexes_to_tx_count.height.u();
        let height_to_unclaimed_rewards = chain
            .indexes_to_unclaimed_rewards
            .sats
            .height
            .as_ref()
            .unwrap();
        let txindex_to_first_txoutindex = &indexer.vecs.tx.txindex_to_first_txoutindex;
        let txindex_to_height = &indexer.vecs.tx.txindex_to_height;
        let txindex_to_input_count = &indexes.txindex_to_input_count;
        let txindex_to_output_count = &indexes.txindex_to_output_count;
        let txinindex_to_outpoint = &indexer.vecs.txin.txinindex_to_outpoint;
        let txoutindex_to_outputtype = &indexer.vecs.txout.txoutindex_to_outputtype;
        let txoutindex_to_txindex = &indexer.vecs.txout.txoutindex_to_txindex;
        let txoutindex_to_typeindex = &indexer.vecs.txout.txoutindex_to_typeindex;
        let txoutindex_to_value = &indexer.vecs.txout.txoutindex_to_value;

        let mut height_to_price_close_iter = height_to_price_close.as_ref().map(|v| v.into_iter());
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();

        let base_version = Version::ZERO
            + dateindex_to_first_height.version()
            + dateindex_to_height_count.version()
            + dateindex_to_price_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + height_to_date_fixed.version()
            + height_to_first_p2aaddressindex.version()
            + height_to_first_p2pk33addressindex.version()
            + height_to_first_p2pk65addressindex.version()
            + height_to_first_p2pkhaddressindex.version()
            + height_to_first_p2shaddressindex.version()
            + height_to_first_p2traddressindex.version()
            + height_to_first_p2wpkhaddressindex.version()
            + height_to_first_p2wshaddressindex.version()
            + height_to_first_txindex.version()
            + height_to_txindex_count.version()
            + height_to_first_txinindex.version()
            + height_to_first_txoutindex.version()
            + height_to_input_count.version()
            + height_to_output_count.version()
            + height_to_price_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + height_to_timestamp_fixed.version()
            + height_to_tx_count.version()
            + height_to_unclaimed_rewards.version()
            + txindex_to_first_txoutindex.version()
            + txindex_to_height.version()
            + txindex_to_input_count.version()
            + txindex_to_output_count.version()
            + txinindex_to_outpoint.version()
            + txoutindex_to_outputtype.version()
            + txoutindex_to_txindex.version()
            + txoutindex_to_typeindex.version()
            + txoutindex_to_value.version();

        let mut separate_utxo_vecs = self.utxo_cohorts.iter_separate_mut().collect::<Vec<_>>();
        let mut separate_address_vecs =
            self.address_cohorts.iter_separate_mut().collect::<Vec<_>>();

        separate_utxo_vecs
            .par_iter_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;
        separate_address_vecs
            .par_iter_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;
        self.height_to_unspendable_supply
            .validate_computed_version_or_reset(
                base_version + self.height_to_unspendable_supply.inner_version(),
            )?;
        self.height_to_opreturn_supply
            .validate_computed_version_or_reset(
                base_version + self.height_to_opreturn_supply.inner_version(),
            )?;

        let mut chain_state_starting_height = Height::from(self.chain_state.len());
        let stateful_starting_height = match separate_utxo_vecs
            .par_iter_mut()
            .map(|v| Height::from(v.min_height_vecs_len()))
            .min()
            .unwrap_or_default()
            .min(
                separate_address_vecs
                    .par_iter_mut()
                    .map(|v| Height::from(v.min_height_vecs_len()))
                    .min()
                    .unwrap_or_default(),
            )
            .min(chain_state_starting_height)
            .min(self.any_address_indexes.min_stamped_height())
            .min(self.addresses_data.min_stamped_height())
            .min(Height::from(self.height_to_unspendable_supply.len()))
            .min(Height::from(self.height_to_opreturn_supply.len()))
            .cmp(&chain_state_starting_height)
        {
            Ordering::Greater => unreachable!(),
            Ordering::Equal => chain_state_starting_height,
            Ordering::Less => Height::ZERO,
        };

        let starting_height = starting_indexes.height.min(stateful_starting_height);
        let last_height = Height::from(indexer.vecs.block.height_to_blockhash.stamp());
        if starting_height <= last_height {
            let stamp = starting_height.into();
            let starting_height = if starting_height.is_not_zero() {
                let mut set = [self.chain_state.rollback_before(stamp)?]
                    .into_iter()
                    .chain(self.any_address_indexes.rollback_before(stamp)?)
                    .chain(self.addresses_data.rollback_before(stamp)?)
                    .map(Height::from)
                    .map(Height::incremented)
                    .collect::<BTreeSet<Height>>();

                if set.len() == 1 {
                    set.pop_first().unwrap()
                } else {
                    Height::ZERO
                }
            } else {
                Height::ZERO
            };

            let starting_height = if starting_height.is_not_zero()
                && separate_utxo_vecs
                    .iter_mut()
                    .map(|v| v.import_state(starting_height).unwrap_or_default())
                    .all(|h| h == starting_height)
            {
                starting_height
            } else {
                Height::ZERO
            };

            let starting_height = if starting_height.is_not_zero()
                && separate_address_vecs
                    .iter_mut()
                    .map(|v| v.import_state(starting_height).unwrap_or_default())
                    .all(|h| h == starting_height)
            {
                starting_height
            } else {
                Height::ZERO
            };

            // Import aggregate cohorts' price_to_amount.
            // Need to temporarily release the separate vecs borrows since iter_aggregate_mut
            // borrows the whole UTXOGroups struct, even though it accesses non-overlapping fields.
            let starting_height = {
                drop(separate_utxo_vecs);
                drop(separate_address_vecs);
                let imported_height = self
                    .utxo_cohorts
                    .import_aggregate_price_to_amount(starting_height)?;
                let result = if starting_height.is_not_zero() && imported_height == starting_height
                {
                    starting_height
                } else {
                    Height::ZERO
                };
                separate_utxo_vecs = self.utxo_cohorts.iter_separate_mut().collect();
                separate_address_vecs = self.address_cohorts.iter_separate_mut().collect();
                result
            };

            let mut chain_state: Vec<BlockState>;
            if starting_height.is_not_zero() {
                chain_state = self
                    .chain_state
                    .collect_range(None, None)
                    .into_iter()
                    .enumerate()
                    .map(|(height, supply)| {
                        let height = Height::from(height);
                        let timestamp = height_to_timestamp_fixed_iter.get_unwrap(height);
                        let price = height_to_price_close_iter
                            .as_mut()
                            .map(|i| *i.get_unwrap(height));
                        BlockState {
                            timestamp,
                            price,
                            supply,
                        }
                    })
                    .collect::<Vec<_>>();
            } else {
                info!("Starting processing utxos from the start");

                chain_state = vec![];

                self.any_address_indexes.reset()?;
                self.addresses_data.reset()?;

                separate_utxo_vecs.par_iter_mut().try_for_each(|v| {
                    v.reset_state_starting_height();
                    v.state.um().reset_price_to_amount_if_needed()
                })?;

                // Reset aggregate cohorts' price_to_amount
                self.utxo_cohorts.reset_aggregate_price_to_amount()?;

                separate_address_vecs.par_iter_mut().try_for_each(|v| {
                    v.reset_state_starting_height();
                    v.state.um().reset_price_to_amount_if_needed()
                })?;
            }

            chain_state_starting_height = starting_height;

            starting_indexes.update_from_height(starting_height, indexes);

            let ir = IndexerReaders::new(indexer);

            let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
            let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();
            let mut dateindex_to_price_close_iter =
                dateindex_to_price_close.as_ref().map(|v| v.into_iter());
            let mut height_to_date_fixed_iter = height_to_date_fixed.into_iter();
            let mut height_to_first_p2aaddressindex_iter =
                height_to_first_p2aaddressindex.into_iter();
            let mut height_to_first_p2pk33addressindex_iter =
                height_to_first_p2pk33addressindex.into_iter();
            let mut height_to_first_p2pk65addressindex_iter =
                height_to_first_p2pk65addressindex.into_iter();
            let mut height_to_first_p2pkhaddressindex_iter =
                height_to_first_p2pkhaddressindex.into_iter();
            let mut height_to_first_p2shaddressindex_iter =
                height_to_first_p2shaddressindex.into_iter();
            let mut height_to_first_p2traddressindex_iter =
                height_to_first_p2traddressindex.into_iter();
            let mut height_to_first_p2wpkhaddressindex_iter =
                height_to_first_p2wpkhaddressindex.into_iter();
            let mut height_to_first_p2wshaddressindex_iter =
                height_to_first_p2wshaddressindex.into_iter();
            let mut height_to_first_txindex_iter = height_to_first_txindex.into_iter();
            let mut height_to_first_txinindex_iter = height_to_first_txinindex.into_iter();
            let mut height_to_first_txoutindex_iter = height_to_first_txoutindex.into_iter();
            let mut height_to_input_count_iter = height_to_input_count.into_iter();
            let mut height_to_output_count_iter = height_to_output_count.into_iter();
            let mut height_to_tx_count_iter = height_to_tx_count.into_iter();
            let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
            let mut txindex_to_input_count_iter = txindex_to_input_count.iter();
            let mut txindex_to_output_count_iter = txindex_to_output_count.iter();

            let height_to_price_close_vec =
                height_to_price_close.map(|height_to_price_close| height_to_price_close.collect());

            let height_to_timestamp_fixed_vec = height_to_timestamp_fixed.collect();
            let txoutindex_range_to_height = RangeMap::from(height_to_first_txoutindex);

            let mut unspendable_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_unspendable_supply
                    .into_iter()
                    .get_unwrap(prev_height)
            } else {
                Sats::ZERO
            };
            let mut opreturn_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_opreturn_supply
                    .into_iter()
                    .get_unwrap(prev_height)
            } else {
                Sats::ZERO
            };
            let mut addresstype_to_addr_count = AddressTypeToAddressCount::from((
                &self.addresstype_to_height_to_addr_count,
                starting_height,
            ));
            let mut addresstype_to_empty_addr_count = AddressTypeToAddressCount::from((
                &self.addresstype_to_height_to_empty_addr_count,
                starting_height,
            ));

            let mut height = starting_height;

            let mut addresstype_to_typeindex_to_loadedaddressdata =
                AddressTypeToTypeIndexMap::<WithAddressDataSource<LoadedAddressData>>::default();
            let mut addresstype_to_typeindex_to_emptyaddressdata =
                AddressTypeToTypeIndexMap::<WithAddressDataSource<EmptyAddressData>>::default();

            let mut vr = VecsReaders::new(self);

            let last_height = Height::from(
                height_to_date_fixed
                    .len()
                    .checked_sub(1)
                    .unwrap_or_default(),
            );

            for _height in (height.to_usize()..height_to_date_fixed.len()).map(Height::from) {
                height = _height;

                info!("Processing chain at {height}...");

                self.utxo_cohorts
                    .iter_separate_mut()
                    .for_each(|v| v.state.um().reset_single_iteration_values());

                self.address_cohorts
                    .iter_separate_mut()
                    .for_each(|v| v.state.um().reset_single_iteration_values());

                let timestamp = height_to_timestamp_fixed_iter.get_unwrap(height);
                let price = height_to_price_close_iter
                    .as_mut()
                    .map(|i| *i.get_unwrap(height));
                let first_txindex = height_to_first_txindex_iter.get_unwrap(height);
                let first_txoutindex = height_to_first_txoutindex_iter
                    .get_unwrap(height)
                    .to_usize();
                let first_txinindex = height_to_first_txinindex_iter.get_unwrap(height).to_usize();
                let tx_count = height_to_tx_count_iter.get_unwrap(height);
                let output_count = height_to_output_count_iter.get_unwrap(height);
                let input_count = height_to_input_count_iter.get_unwrap(height);

                let txoutindex_to_txindex = build_txoutindex_to_txindex(
                    first_txindex,
                    u64::from(tx_count),
                    &mut txindex_to_output_count_iter,
                );

                let txinindex_to_txindex = build_txinindex_to_txindex(
                    first_txindex,
                    u64::from(tx_count),
                    &mut txindex_to_input_count_iter,
                );

                let first_addressindexes: ByAddressType<TypeIndex> = ByAddressType {
                    p2a: height_to_first_p2aaddressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2pk33: height_to_first_p2pk33addressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2pk65: height_to_first_p2pk65addressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2pkh: height_to_first_p2pkhaddressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2sh: height_to_first_p2shaddressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2tr: height_to_first_p2traddressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2wpkh: height_to_first_p2wpkhaddressindex_iter
                        .get_unwrap(height)
                        .into(),
                    p2wsh: height_to_first_p2wshaddressindex_iter
                        .get_unwrap(height)
                        .into(),
                };

                let (
                    mut transacted,
                    addresstype_to_typedindex_to_received_data,
                    mut height_to_sent,
                    addresstype_to_typedindex_to_sent_data,
                    mut stored_or_new_addresstype_to_typeindex_to_addressdatawithsource,
                    mut combined_txindex_vecs,
                ) = thread::scope(|scope| {
                    scope.spawn(|| {
                        self.utxo_cohorts
                            .tick_tock_next_block(&chain_state, timestamp);
                    });

                    let (transacted, addresstype_to_typedindex_to_received_data, receiving_addresstype_to_typeindex_to_addressdatawithsource, output_txindex_vecs) = (first_txoutindex..first_txoutindex + usize::from(output_count))
                        .into_par_iter()
                        .map(|i| {
                            let txoutindex = TxOutIndex::from(i);

                            let local_idx = i - first_txoutindex;
                            let txindex = txoutindex_to_txindex[local_idx];

                            let value = txoutindex_to_value
                                .read_unwrap(txoutindex, &ir.txoutindex_to_value);

                            let output_type = txoutindex_to_outputtype
                                .read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

                            if output_type.is_not_address() {
                                return (txindex, value, output_type, None);
                            }

                            let typeindex = txoutindex_to_typeindex
                                .read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

                            let addressdata_opt = Self::get_addressdatawithsource(
                                output_type,
                                typeindex,
                                &first_addressindexes,
                                &addresstype_to_typeindex_to_loadedaddressdata,
                                &addresstype_to_typeindex_to_emptyaddressdata,
                                &vr,
                                &self.any_address_indexes,
                                &self.addresses_data,
                            );

                            (txindex, value, output_type, Some(( typeindex, addressdata_opt)))
                        }).fold(
                        || {
                            (
                                Transacted::default(),
                                AddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                AddressTypeToTypeIndexMap::default(),
                                AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                            )
                        },
                        |(mut transacted, mut addresstype_to_typedindex_to_data, mut addresstype_to_typeindex_to_addressdatawithsource, mut txindex_vecs),
                            (
                            txindex,
                            value,
                            output_type,
                            typeindex_with_addressdata_opt,
                        )| {
                            transacted.iterate(value, output_type);

                            if let Some((typeindex, addressdata_opt)) = typeindex_with_addressdata_opt {
                                if let Some(addressdata) = addressdata_opt
                                {
                                    addresstype_to_typeindex_to_addressdatawithsource
                                        .insert_for_type(output_type, typeindex, addressdata);
                                }

                                let addr_type = output_type;

                                addresstype_to_typedindex_to_data
                                    .get_mut(addr_type)
                                    .unwrap()
                                    .push((typeindex, value));

                                txindex_vecs
                                    .get_mut(addr_type)
                                    .unwrap()
                                    .entry(typeindex)
                                    .or_insert_with(TxIndexVec::new)
                                    .push(txindex);
                            }

                            (transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource, txindex_vecs)
                        }).reduce(
                            || {
                                (
                                    Transacted::default(),
                                    AddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                    AddressTypeToTypeIndexMap::default(),
                                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                                )
                            },
                            |(transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource, txindex_vecs), (transacted2, addresstype_to_typedindex_to_data2, addresstype_to_typeindex_to_addressdatawithsource2, txindex_vecs2)| {
                                (transacted + transacted2, addresstype_to_typedindex_to_data.merge(addresstype_to_typedindex_to_data2), addresstype_to_typeindex_to_addressdatawithsource.merge(addresstype_to_typeindex_to_addressdatawithsource2), txindex_vecs.merge_vec(txindex_vecs2))
                            },
                        );

                    // Skip coinbase
                    let (
                        height_to_sent,
                        addresstype_to_typedindex_to_sent_data,
                        sending_addresstype_to_typeindex_to_addressdatawithsource,
                        input_txindex_vecs,
                    ) = (first_txinindex + 1..first_txinindex + usize::from(input_count))
                        .into_par_iter()
                        .map(|i| {
                            let txinindex = TxInIndex::from(i);

                            let local_idx = i - first_txinindex;
                            let txindex = txinindex_to_txindex[local_idx];

                            let outpoint = txinindex_to_outpoint
                                .read_unwrap(txinindex, &ir.txinindex_to_outpoint);

                            let txoutindex = txindex_to_first_txoutindex
                                .read_unwrap(outpoint.txindex(), &ir.txindex_to_first_txoutindex)
                                + outpoint.vout();

                            let value = txoutindex_to_value
                                .read_unwrap(txoutindex, &ir.txoutindex_to_value);

                            let input_type = txoutindex_to_outputtype
                                .read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

                            let prev_height = *txoutindex_range_to_height.get(txoutindex).unwrap();

                            if input_type.is_not_address() {
                                return (txindex, prev_height, value, input_type, None);
                            }

                            let typeindex = txoutindex_to_typeindex
                                .read_unwrap(txoutindex, &ir.txoutindex_to_typeindex);

                            let addressdata_opt = Self::get_addressdatawithsource(
                                input_type,
                                typeindex,
                                &first_addressindexes,
                                &addresstype_to_typeindex_to_loadedaddressdata,
                                &addresstype_to_typeindex_to_emptyaddressdata,
                                &vr,
                                &self.any_address_indexes,
                                &self.addresses_data,
                            );

                            (
                                txindex,
                                prev_height,
                                value,
                                input_type,
                                Some((typeindex, addressdata_opt)),
                            )
                        })
                        .fold(
                            || {
                                (
                                    FxHashMap::<Height, Transacted>::default(),
                                    HeightToAddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                    AddressTypeToTypeIndexMap::default(),
                                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                                )
                            },
                            |(
                                mut height_to_transacted,
                                mut height_to_addresstype_to_typedindex_to_data,
                                mut addresstype_to_typeindex_to_addressdatawithsource,
                                mut txindex_vecs,
                            ),
                             (
                                txindex,
                                prev_height,
                                value,
                                output_type,
                                typeindex_with_addressdata_opt,
                            )| {
                                height_to_transacted
                                    .entry(prev_height)
                                    .or_default()
                                    .iterate(value, output_type);

                                if let Some((typeindex, addressdata_opt)) =
                                    typeindex_with_addressdata_opt
                                {
                                    if let Some(addressdata) = addressdata_opt {
                                        addresstype_to_typeindex_to_addressdatawithsource
                                            .insert_for_type(output_type, typeindex, addressdata);
                                    }

                                    let addr_type = output_type;

                                    height_to_addresstype_to_typedindex_to_data
                                        .entry(prev_height)
                                        .or_default()
                                        .get_mut(addr_type)
                                        .unwrap()
                                        .push((typeindex, value));

                                    txindex_vecs
                                        .get_mut(addr_type)
                                        .unwrap()
                                        .entry(typeindex)
                                        .or_insert_with(TxIndexVec::new)
                                        .push(txindex);
                                }

                                (
                                    height_to_transacted,
                                    height_to_addresstype_to_typedindex_to_data,
                                    addresstype_to_typeindex_to_addressdatawithsource,
                                    txindex_vecs,
                                )
                            },
                        )
                        .reduce(
                            || {
                                (
                                    FxHashMap::<Height, Transacted>::default(),
                                    HeightToAddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                    AddressTypeToTypeIndexMap::default(),
                                    AddressTypeToTypeIndexMap::<TxIndexVec>::default(),
                                )
                            },
                            |(
                                height_to_transacted,
                                addresstype_to_typedindex_to_data,
                                addresstype_to_typeindex_to_addressdatawithsource,
                                txindex_vecs,
                            ),
                             (
                                height_to_transacted2,
                                addresstype_to_typedindex_to_data2,
                                addresstype_to_typeindex_to_addressdatawithsource2,
                                txindex_vecs2,
                            )| {
                                let (mut height_to_transacted, height_to_transacted_consumed) =
                                    if height_to_transacted.len() > height_to_transacted2.len() {
                                        (height_to_transacted, height_to_transacted2)
                                    } else {
                                        (height_to_transacted2, height_to_transacted)
                                    };
                                height_to_transacted_consumed
                                    .into_iter()
                                    .for_each(|(k, v)| {
                                        *height_to_transacted.entry(k).or_default() += v;
                                    });

                                let (
                                    mut addresstype_to_typedindex_to_data,
                                    addresstype_to_typedindex_to_data_consumed,
                                ) = if addresstype_to_typedindex_to_data.len()
                                    > addresstype_to_typedindex_to_data2.len()
                                {
                                    (
                                        addresstype_to_typedindex_to_data,
                                        addresstype_to_typedindex_to_data2,
                                    )
                                } else {
                                    (
                                        addresstype_to_typedindex_to_data2,
                                        addresstype_to_typedindex_to_data,
                                    )
                                };
                                addresstype_to_typedindex_to_data_consumed
                                    .0
                                    .into_iter()
                                    .for_each(|(k, v)| {
                                        addresstype_to_typedindex_to_data
                                            .entry(k)
                                            .or_default()
                                            .merge_mut(v);
                                    });

                                (
                                    height_to_transacted,
                                    addresstype_to_typedindex_to_data,
                                    addresstype_to_typeindex_to_addressdatawithsource
                                        .merge(addresstype_to_typeindex_to_addressdatawithsource2),
                                    txindex_vecs.merge_vec(txindex_vecs2),
                                )
                            },
                        );

                    let addresstype_to_typeindex_to_addressdatawithsource =
                        receiving_addresstype_to_typeindex_to_addressdatawithsource
                            .merge(sending_addresstype_to_typeindex_to_addressdatawithsource);

                    let combined_txindex_vecs = output_txindex_vecs.merge_vec(input_txindex_vecs);

                    (
                        transacted,
                        addresstype_to_typedindex_to_received_data,
                        height_to_sent,
                        addresstype_to_typedindex_to_sent_data,
                        addresstype_to_typeindex_to_addressdatawithsource,
                        combined_txindex_vecs,
                    )
                });

                combined_txindex_vecs
                    .par_values_mut()
                    .flat_map(|typeindex_to_txindexes| typeindex_to_txindexes.par_iter_mut())
                    .map(|(_, v)| v)
                    .filter(|txindex_vec| txindex_vec.len() > 1)
                    .for_each(|txindex_vec| {
                        txindex_vec.sort_unstable();
                        txindex_vec.dedup();
                    });

                for (address_type, typeindex, txindex_vec) in combined_txindex_vecs
                    .into_iter()
                    .flat_map(|(t, m)| m.into_iter().map(move |(i, v)| (t, i, v)))
                {
                    let tx_count = txindex_vec.len() as u32;

                    if let Some(addressdata) = addresstype_to_typeindex_to_loadedaddressdata
                        .get_mut_unwrap(address_type)
                        .get_mut(&typeindex)
                    {
                        addressdata.deref_mut().tx_count += tx_count;
                    } else if let Some(addressdata) = addresstype_to_typeindex_to_emptyaddressdata
                        .get_mut_unwrap(address_type)
                        .get_mut(&typeindex)
                    {
                        addressdata.deref_mut().tx_count += tx_count;
                    } else if let Some(addressdata) =
                        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                            .get_mut_unwrap(address_type)
                            .get_mut(&typeindex)
                    {
                        addressdata.deref_mut().tx_count += tx_count;
                    }
                }

                thread::scope(|scope| {
                    scope.spawn(|| {
                        addresstype_to_typedindex_to_received_data.process_received(
                            &mut self.address_cohorts,
                            &mut addresstype_to_typeindex_to_loadedaddressdata,
                            &mut addresstype_to_typeindex_to_emptyaddressdata,
                            price,
                            &mut addresstype_to_addr_count,
                            &mut addresstype_to_empty_addr_count,
                            &mut stored_or_new_addresstype_to_typeindex_to_addressdatawithsource,
                        );

                        addresstype_to_typedindex_to_sent_data
                            .process_sent(
                                &mut self.address_cohorts,
                                &mut addresstype_to_typeindex_to_loadedaddressdata,
                                &mut addresstype_to_typeindex_to_emptyaddressdata,
                                price,
                                &mut addresstype_to_addr_count,
                                &mut addresstype_to_empty_addr_count,
                                height_to_price_close_vec.as_ref(),
                                &height_to_timestamp_fixed_vec,
                                height,
                                timestamp,
                                &mut stored_or_new_addresstype_to_typeindex_to_addressdatawithsource,
                            )
                            .unwrap();
                    });

                    debug_assert!(
                        chain_state_starting_height <= height,
                        "chain_state_starting_height ({chain_state_starting_height}) > height ({height})"
                    );

                    // NOTE: If ByUnspendableType gains more fields, change to .as_vec().into_iter().map(|s| s.value).sum()
                    unspendable_supply += transacted.by_type.unspendable.opreturn.value
                        + height_to_unclaimed_rewards_iter.get_unwrap(height);

                    opreturn_supply += transacted.by_type.unspendable.opreturn.value;

                    if height == Height::ZERO {
                        transacted = Transacted::default();
                        unspendable_supply += Sats::FIFTY_BTC;
                    } else if height == Height::new(BIP30_DUPLICATE_COINBASE_HEIGHT_1)
                        || height == Height::new(BIP30_DUPLICATE_COINBASE_HEIGHT_2)
                    {
                        if height == Height::new(BIP30_DUPLICATE_COINBASE_HEIGHT_1) {
                            height_to_sent
                                .entry(Height::new(BIP30_ORIGINAL_COINBASE_HEIGHT_1))
                                .or_default()
                        } else {
                            height_to_sent
                                .entry(Height::new(BIP30_ORIGINAL_COINBASE_HEIGHT_2))
                                .or_default()
                        }
                        .iterate(Sats::FIFTY_BTC, OutputType::P2PK65);
                    }

                    // Push current block state before processing sends and receives
                    chain_state.push(BlockState {
                        supply: transacted.spendable_supply.clone(),
                        price,
                        timestamp,
                    });

                    self.utxo_cohorts.receive(transacted, height, price);

                    self.utxo_cohorts.send(height_to_sent, &mut chain_state);
                });

                self.height_to_unspendable_supply
                    .truncate_push(height, unspendable_supply)?;

                self.height_to_opreturn_supply
                    .truncate_push(height, opreturn_supply)?;

                self.addresstype_to_height_to_addr_count
                    .truncate_push(height, &addresstype_to_addr_count)?;

                self.addresstype_to_height_to_empty_addr_count
                    .truncate_push(height, &addresstype_to_empty_addr_count)?;

                let date = height_to_date_fixed_iter.get_unwrap(height);
                let dateindex = DateIndex::try_from(date).unwrap();
                let date_first_height = dateindex_to_first_height_iter.get_unwrap(dateindex);
                let date_height_count = dateindex_to_height_count_iter.get_unwrap(dateindex);
                let is_date_last_height = date_first_height
                    + Height::from(date_height_count).decremented().unwrap()
                    == height;
                let date_price = dateindex_to_price_close_iter
                    .as_mut()
                    .map(|v| is_date_last_height.then(|| *v.get_unwrap(dateindex)));

                let dateindex = is_date_last_height.then_some(dateindex);

                self.utxo_cohorts
                    .par_iter_separate_mut()
                    .map(|v| v as &mut dyn DynCohortVecs)
                    .chain(
                        self.address_cohorts
                            .par_iter_separate_mut()
                            .map(|v| v as &mut dyn DynCohortVecs),
                    )
                    .try_for_each(|v| {
                        v.truncate_push(height)?;
                        v.compute_then_truncate_push_unrealized_states(
                            height, price, dateindex, date_price,
                        )
                    })?;

                // Compute and push percentiles for aggregate cohorts (all, sth, lth)
                self.utxo_cohorts
                    .truncate_push_aggregate_percentiles(height)?;

                if height != last_height
                    && height != Height::ZERO
                    && height.to_usize() % FLUSH_INTERVAL == 0
                {
                    let _lock = exit.lock();

                    drop(vr);

                    self.flush_states(
                        height,
                        &chain_state,
                        mem::take(&mut addresstype_to_typeindex_to_loadedaddressdata),
                        mem::take(&mut addresstype_to_typeindex_to_emptyaddressdata),
                        false,
                        exit,
                    )?;

                    vr = VecsReaders::new(self);
                }
            }

            drop(vr);

            let _lock = exit.lock();
            self.flush_states(
                height,
                &chain_state,
                mem::take(&mut addresstype_to_typeindex_to_loadedaddressdata),
                mem::take(&mut addresstype_to_typeindex_to_emptyaddressdata),
                true,
                exit,
            )?;
        }

        info!("Computing overlapping...");

        self.utxo_cohorts
            .compute_overlapping_vecs(starting_indexes, exit)?;

        self.address_cohorts
            .compute_overlapping_vecs(starting_indexes, exit)?;

        info!("Computing rest part 1...");

        self.indexes_to_addr_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_sum_of_others(
                    starting_indexes.height,
                    &self
                        .addresstype_to_height_to_addr_count
                        .iter()
                        .map(|(_, v)| v)
                        .collect::<Vec<_>>(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_empty_addr_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_sum_of_others(
                    starting_indexes.height,
                    &self
                        .addresstype_to_height_to_empty_addr_count
                        .iter()
                        .map(|(_, v)| v)
                        .collect::<Vec<_>>(),
                    exit,
                )?;
                Ok(())
            })?;

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

        self.addresstype_to_indexes_to_addr_count.compute(
            indexes,
            starting_indexes,
            exit,
            &self.addresstype_to_height_to_addr_count,
        )?;

        self.addresstype_to_indexes_to_empty_addr_count.compute(
            indexes,
            starting_indexes,
            exit,
            &self.addresstype_to_height_to_empty_addr_count,
        )?;

        self.utxo_cohorts
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;

        self.address_cohorts
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;

        if let Some(indexes_to_market_cap) = self.indexes_to_market_cap.as_mut() {
            indexes_to_market_cap.compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.utxo_cohorts
                        .all
                        .inner
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

        info!("Computing rest part 2...");

        let height_to_supply = &self
            .utxo_cohorts
            .all
            .inner
            .height_to_supply_value
            .bitcoin
            .clone();
        let dateindex_to_supply = self
            .utxo_cohorts
            .all
            .inner
            .indexes_to_supply
            .bitcoin
            .dateindex
            .clone();
        let height_to_market_cap = self.height_to_market_cap.clone();
        let dateindex_to_market_cap = self
            .indexes_to_market_cap
            .as_ref()
            .map(|v| v.dateindex.u().clone());
        let height_to_realized_cap = self.utxo_cohorts.all.inner.height_to_realized_cap.clone();
        let dateindex_to_realized_cap = self
            .utxo_cohorts
            .all
            .inner
            .indexes_to_realized_cap
            .as_ref()
            .map(|v| v.dateindex.unwrap_last().clone());
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

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn get_addressdatawithsource(
        address_type: OutputType,
        typeindex: TypeIndex,
        first_addressindexes: &ByAddressType<TypeIndex>,
        addresstype_to_typeindex_to_loadedaddressdata: &AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        vr: &VecsReaders,
        any_address_indexes: &AnyAddressIndexesVecs,
        addresses_data: &AddressesDataVecs,
    ) -> Option<WithAddressDataSource<LoadedAddressData>> {
        let first = *first_addressindexes.get(address_type).unwrap();
        if first <= typeindex {
            return Some(WithAddressDataSource::New(LoadedAddressData::default()));
        }

        if addresstype_to_typeindex_to_loadedaddressdata
            .get(address_type)
            .unwrap()
            .contains_key(&typeindex)
            || addresstype_to_typeindex_to_emptyaddressdata
                .get(address_type)
                .unwrap()
                .contains_key(&typeindex)
        {
            return None;
        }

        let reader = vr.get_anyaddressindex_reader(address_type);

        let anyaddressindex =
            any_address_indexes.get_anyaddressindex(address_type, typeindex, reader);

        Some(match anyaddressindex.to_enum() {
            AnyAddressDataIndexEnum::Loaded(loadedaddressindex) => {
                let reader = &vr.anyaddressindex_to_anyaddressdata.loaded;

                let loadedaddressdata = addresses_data
                    .loaded
                    .get_pushed_or_read_unwrap(loadedaddressindex, reader);

                WithAddressDataSource::FromLoadedAddressDataVec((
                    loadedaddressindex,
                    loadedaddressdata,
                ))
            }
            AnyAddressDataIndexEnum::Empty(emtpyaddressindex) => {
                let reader = &vr.anyaddressindex_to_anyaddressdata.empty;

                let emptyaddressdata = addresses_data
                    .empty
                    .get_pushed_or_read_unwrap(emtpyaddressindex, reader);

                WithAddressDataSource::FromEmptyAddressDataVec((
                    emtpyaddressindex,
                    emptyaddressdata.into(),
                ))
            }
        })
    }

    fn flush_states(
        &mut self,
        height: Height,
        chain_state: &[BlockState],
        addresstype_to_typeindex_to_loadedaddressdata: AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        with_changes: bool,
        exit: &Exit,
    ) -> Result<()> {
        info!("Flushing...");

        self.utxo_cohorts.safe_flush_stateful_vecs(height, exit)?;
        self.address_cohorts
            .safe_flush_stateful_vecs(height, exit)?;
        self.height_to_unspendable_supply.safe_write(exit)?;
        self.height_to_opreturn_supply.safe_write(exit)?;
        self.addresstype_to_height_to_addr_count
            .values_mut()
            .try_for_each(|v| v.safe_flush(exit))?;
        self.addresstype_to_height_to_empty_addr_count
            .values_mut()
            .try_for_each(|v| v.safe_flush(exit))?;

        let mut addresstype_to_typeindex_to_new_or_updated_anyaddressindex =
            AddressTypeToTypeIndexMap::default();

        for (address_type, sorted) in
            addresstype_to_typeindex_to_emptyaddressdata.into_sorted_iter()
        {
            for (typeindex, emptyaddressdata_with_source) in sorted.into_iter() {
                match emptyaddressdata_with_source {
                    WithAddressDataSource::New(emptyaddressdata) => {
                        let emptyaddressindex = self
                            .addresses_data
                            .empty
                            .fill_first_hole_or_push(emptyaddressdata)?;

                        let anyaddressindex = AnyAddressIndex::from(emptyaddressindex);

                        addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                            .get_mut(address_type)
                            .unwrap()
                            .insert(typeindex, anyaddressindex);
                    }
                    WithAddressDataSource::FromEmptyAddressDataVec((
                        emptyaddressindex,
                        emptyaddressdata,
                    )) => self
                        .addresses_data
                        .empty
                        .update(emptyaddressindex, emptyaddressdata)?,
                    WithAddressDataSource::FromLoadedAddressDataVec((
                        loadedaddressindex,
                        emptyaddressdata,
                    )) => {
                        self.addresses_data.loaded.delete(loadedaddressindex);

                        let emptyaddressindex = self
                            .addresses_data
                            .empty
                            .fill_first_hole_or_push(emptyaddressdata)?;

                        let anyaddressindex = emptyaddressindex.into();

                        addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                            .get_mut(address_type)
                            .unwrap()
                            .insert(typeindex, anyaddressindex);
                    }
                }
            }
        }

        for (address_type, sorted) in
            addresstype_to_typeindex_to_loadedaddressdata.into_sorted_iter()
        {
            for (typeindex, loadedaddressdata_with_source) in sorted.into_iter() {
                match loadedaddressdata_with_source {
                    WithAddressDataSource::New(loadedaddressdata) => {
                        let loadedaddressindex = self
                            .addresses_data
                            .loaded
                            .fill_first_hole_or_push(loadedaddressdata)?;

                        let anyaddressindex = AnyAddressIndex::from(loadedaddressindex);

                        addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                            .get_mut(address_type)
                            .unwrap()
                            .insert(typeindex, anyaddressindex);
                    }
                    WithAddressDataSource::FromLoadedAddressDataVec((
                        loadedaddressindex,
                        loadedaddressdata,
                    )) => self
                        .addresses_data
                        .loaded
                        .update(loadedaddressindex, loadedaddressdata)?,
                    WithAddressDataSource::FromEmptyAddressDataVec((
                        emptyaddressindex,
                        loadedaddressdata,
                    )) => {
                        self.addresses_data.empty.delete(emptyaddressindex);

                        let loadedaddressindex = self
                            .addresses_data
                            .loaded
                            .fill_first_hole_or_push(loadedaddressdata)?;

                        let anyaddressindex = loadedaddressindex.into();

                        addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                            .get_mut(address_type)
                            .unwrap()
                            .insert(typeindex, anyaddressindex);
                    }
                }
            }
        }

        for (address_type, sorted) in
            addresstype_to_typeindex_to_new_or_updated_anyaddressindex.into_sorted_iter()
        {
            for (typeindex, anyaddressindex) in sorted {
                self.any_address_indexes.update_or_push(
                    address_type,
                    typeindex,
                    anyaddressindex,
                )?;
            }
        }

        let stamp = Stamp::from(height);

        self.any_address_indexes
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.addresses_data
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;

        self.chain_state.truncate_if_needed(Height::ZERO)?;
        chain_state.iter().for_each(|block_state| {
            self.chain_state.push(block_state.supply.clone());
        });
        self.chain_state
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;

        Ok(())
    }
}

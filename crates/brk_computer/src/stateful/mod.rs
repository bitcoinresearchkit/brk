use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    mem,
    path::Path,
    thread,
};

use brk_error::Result;
use brk_grouper::{ByAddressType, ByAnyAddress, Filtered};
use brk_indexer::Indexer;
use brk_structs::{
    AnyAddressDataIndexEnum, AnyAddressIndex, CheckedSub, DateIndex, Dollars, EmptyAddressData,
    EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex, OutputType, P2AAddressIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, Sats, StoredU64, Timestamp, TxInIndex, TxOutIndex,
    TypeIndex, Version,
};
use brk_traversable::Traversable;
use log::info;
use rayon::prelude::*;
use vecdb::{
    AnyCloneableIterableVec, AnyStoredVec, AnyVec, CollectableVec, Database, EagerVec, Exit,
    Format, GenericStoredVec, ImportOptions, LazyVecFrom1, PAGE_SIZE, RawVec, Reader, Stamp,
    StoredIndex, VecIterator,
};

use crate::{
    BlockState, Indexes, SupplyState, Transacted, chain,
    grouped::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes, price,
};

mod address_cohort;
mod address_cohorts;
mod addresstype;
mod common;
mod range_map;
mod r#trait;
mod utxo_cohort;
mod utxo_cohorts;
mod withaddressdatasource;

use addresstype::*;
use range_map::*;
use r#trait::*;
use withaddressdatasource::*;

const VERSION: Version = Version::new(21);

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    // ---
    // States
    // ---
    pub chain_state: RawVec<Height, SupplyState>,
    pub p2pk33addressindex_to_anyaddressindex: RawVec<P2PK33AddressIndex, AnyAddressIndex>,
    pub p2pk65addressindex_to_anyaddressindex: RawVec<P2PK65AddressIndex, AnyAddressIndex>,
    pub p2pkhaddressindex_to_anyaddressindex: RawVec<P2PKHAddressIndex, AnyAddressIndex>,
    pub p2shaddressindex_to_anyaddressindex: RawVec<P2SHAddressIndex, AnyAddressIndex>,
    pub p2traddressindex_to_anyaddressindex: RawVec<P2TRAddressIndex, AnyAddressIndex>,
    pub p2wpkhaddressindex_to_anyaddressindex: RawVec<P2WPKHAddressIndex, AnyAddressIndex>,
    pub p2wshaddressindex_to_anyaddressindex: RawVec<P2WSHAddressIndex, AnyAddressIndex>,
    pub p2aaddressindex_to_anyaddressindex: RawVec<P2AAddressIndex, AnyAddressIndex>,
    pub loadedaddressindex_to_loadedaddressdata: RawVec<LoadedAddressIndex, LoadedAddressData>,
    pub emptyaddressindex_to_emptyaddressdata: RawVec<EmptyAddressIndex, EmptyAddressData>,

    pub utxo_cohorts: utxo_cohorts::Vecs,
    pub address_cohorts: address_cohorts::Vecs,

    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub height_to_opreturn_supply: EagerVec<Height, Sats>,
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
        format: Format,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db_path = parent.join("stateful");
        let states_path = db_path.join("states");

        let db = Database::open(&db_path)?;
        db.set_min_len(PAGE_SIZE * 20_000_000)?;
        db.set_min_regions(50_000)?;

        let compute_dollars = price.is_some();

        let utxo_cohorts =
            utxo_cohorts::Vecs::forced_import(&db, version, format, indexes, price, &states_path)?;

        let loadedaddressindex_to_loadedaddressdata = RawVec::forced_import_with(
            ImportOptions::new(&db, "loadedaddressdata", version + VERSION + Version::ZERO)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let emptyaddressindex_to_emptyaddressdata = RawVec::forced_import_with(
            ImportOptions::new(&db, "emptyaddressdata", version + VERSION + Version::ZERO)
                .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
        )?;
        let loadedaddressindex_to_loadedaddressindex = LazyVecFrom1::init(
            "loadedaddressindex",
            version + VERSION + Version::ZERO,
            loadedaddressindex_to_loadedaddressdata.boxed_clone(),
            |index, _| Some(index),
        );
        let emptyaddressindex_to_emptyaddressindex = LazyVecFrom1::init(
            "emptyaddressindex",
            version + VERSION + Version::ZERO,
            emptyaddressindex_to_emptyaddressdata.boxed_clone(),
            |index, _| Some(index),
        );

        let this = Self {
            chain_state: RawVec::forced_import_with(
                ImportOptions::new(&db, "chain", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            height_to_unspendable_supply: EagerVec::forced_import_compressed(
                &db,
                "unspendable_supply",
                version + VERSION + Version::ZERO,
            )?,
            indexes_to_unspendable_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "unspendable_supply",
                Source::None,
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            height_to_opreturn_supply: EagerVec::forced_import_compressed(
                &db,
                "opreturn_supply",
                version + VERSION + Version::ZERO,
            )?,
            indexes_to_opreturn_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "opreturn_supply",
                Source::None,
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_addr_count: ComputedVecsFromHeight::forced_import(
                &db,
                "addr_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_empty_addr_count: ComputedVecsFromHeight::forced_import(
                &db,
                "empty_addr_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            height_to_market_cap: compute_dollars.then(|| {
                LazyVecFrom1::init(
                    "market_cap",
                    version + VERSION + Version::ONE,
                    utxo_cohorts
                        .all
                        .1
                        .height_to_supply_value
                        .dollars
                        .as_ref()
                        .unwrap()
                        .boxed_clone(),
                    |height: Height, iter| {
                        iter.next_at(height.to_usize()).map(|(_, value)| {
                            let d: Dollars = value.into_owned();
                            d
                        })
                    },
                )
            }),
            indexes_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    "market_cap",
                    Source::Compute,
                    version + VERSION + Version::TWO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            addresstype_to_height_to_addr_count: AddressTypeToHeightToAddressCount::from(
                ByAddressType {
                    p2pk65: EagerVec::forced_import_compressed(
                        &db,
                        "p2pk65_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2pk33: EagerVec::forced_import_compressed(
                        &db,
                        "p2pk33_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2pkh: EagerVec::forced_import_compressed(
                        &db,
                        "p2pkh_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2sh: EagerVec::forced_import_compressed(
                        &db,
                        "p2sh_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2wpkh: EagerVec::forced_import_compressed(
                        &db,
                        "p2wpkh_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2wsh: EagerVec::forced_import_compressed(
                        &db,
                        "p2wsh_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2tr: EagerVec::forced_import_compressed(
                        &db,
                        "p2tr_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2a: EagerVec::forced_import_compressed(
                        &db,
                        "p2a_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                },
            ),
            addresstype_to_height_to_empty_addr_count: AddressTypeToHeightToAddressCount::from(
                ByAddressType {
                    p2pk65: EagerVec::forced_import_compressed(
                        &db,
                        "p2pk65_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2pk33: EagerVec::forced_import_compressed(
                        &db,
                        "p2pk33_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2pkh: EagerVec::forced_import_compressed(
                        &db,
                        "p2pkh_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2sh: EagerVec::forced_import_compressed(
                        &db,
                        "p2sh_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2wpkh: EagerVec::forced_import_compressed(
                        &db,
                        "p2wpkh_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2wsh: EagerVec::forced_import_compressed(
                        &db,
                        "p2wsh_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2tr: EagerVec::forced_import_compressed(
                        &db,
                        "p2tr_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                    p2a: EagerVec::forced_import_compressed(
                        &db,
                        "p2a_empty_addr_count",
                        version + VERSION + Version::ZERO,
                    )?,
                },
            ),
            addresstype_to_indexes_to_addr_count: AddressTypeToIndexesToAddressCount::from(
                ByAddressType {
                    p2pk65: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pk65_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2pk33: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pk33_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2pkh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pkh_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2sh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2sh_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2wpkh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2wpkh_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2wsh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2wsh_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2tr: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2tr_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2a: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2a_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                },
            ),
            addresstype_to_indexes_to_empty_addr_count: AddressTypeToIndexesToAddressCount::from(
                ByAddressType {
                    p2pk65: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pk65_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2pk33: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pk33_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2pkh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2pkh_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2sh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2sh_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2wpkh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2wpkh_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2wsh: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2wsh_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2tr: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2tr_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                    p2a: ComputedVecsFromHeight::forced_import(
                        &db,
                        "p2a_empty_addr_count",
                        Source::None,
                        version + VERSION + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )?,
                },
            ),
            utxo_cohorts,
            address_cohorts: address_cohorts::Vecs::forced_import(
                &db,
                version,
                format,
                indexes,
                price,
                &states_path,
            )?,

            p2aaddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2pk33addressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2pk65addressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2pkhaddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2shaddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2traddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2wpkhaddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            p2wshaddressindex_to_anyaddressindex: RawVec::forced_import_with(
                ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,

            loadedaddressindex_to_loadedaddressdata,
            emptyaddressindex_to_emptyaddressdata,
            loadedaddressindex_to_loadedaddressindex,
            emptyaddressindex_to_emptyaddressindex,

            db,
        };

        this.db.retain_regions(
            this.iter_any_collectable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

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
        self.db.flush_then_punch()?;
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
        let height_to_first_txoutindex = &indexer.vecs.height_to_first_txoutindex;
        let height_to_first_txinindex = &indexer.vecs.height_to_first_txinindex;
        let height_to_first_p2aaddressindex = &indexer.vecs.height_to_first_p2aaddressindex;
        let height_to_first_p2pk33addressindex = &indexer.vecs.height_to_first_p2pk33addressindex;
        let height_to_first_p2pk65addressindex = &indexer.vecs.height_to_first_p2pk65addressindex;
        let height_to_first_p2pkhaddressindex = &indexer.vecs.height_to_first_p2pkhaddressindex;
        let height_to_first_p2shaddressindex = &indexer.vecs.height_to_first_p2shaddressindex;
        let height_to_first_p2traddressindex = &indexer.vecs.height_to_first_p2traddressindex;
        let height_to_first_p2wpkhaddressindex = &indexer.vecs.height_to_first_p2wpkhaddressindex;
        let height_to_first_p2wshaddressindex = &indexer.vecs.height_to_first_p2wshaddressindex;
        let height_to_output_count = chain.indexes_to_output_count.height.unwrap_sum();
        let height_to_input_count = chain.indexes_to_input_count.height.unwrap_sum();
        let txinindex_to_txoutindex = &indexer.vecs.txinindex_to_txoutindex;
        let txoutindex_to_value = &indexer.vecs.txoutindex_to_value;
        let txindex_to_height = &indexes.txindex_to_height;
        let height_to_timestamp_fixed = &indexes.height_to_timestamp_fixed;
        let txoutindex_to_txindex = &indexes.txoutindex_to_txindex;
        let txoutindex_to_outputtype = &indexer.vecs.txoutindex_to_outputtype;
        let txoutindex_to_typeindex = &indexer.vecs.txoutindex_to_typeindex;
        let height_to_unclaimed_rewards = chain
            .indexes_to_unclaimed_rewards
            .sats
            .height
            .as_ref()
            .unwrap();
        let height_to_price_close = price
            .as_ref()
            .map(|price| &price.chainindexes_to_price_close.height);
        let dateindex_to_price_close = price
            .as_ref()
            .map(|price| price.timeindexes_to_price_close.dateindex.as_ref().unwrap());
        let height_to_date_fixed = &indexes.height_to_date_fixed;
        let dateindex_to_first_height = &indexes.dateindex_to_first_height;
        let dateindex_to_height_count = &indexes.dateindex_to_height_count;

        let mut height_to_price_close_iter = height_to_price_close.as_ref().map(|v| v.into_iter());
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();

        let base_version = Version::ZERO
            + height_to_first_txoutindex.version()
            + height_to_first_txinindex.version()
            + height_to_first_p2aaddressindex.version()
            + height_to_first_p2pk33addressindex.version()
            + height_to_first_p2pk65addressindex.version()
            + height_to_first_p2pkhaddressindex.version()
            + height_to_first_p2shaddressindex.version()
            + height_to_first_p2traddressindex.version()
            + height_to_first_p2wpkhaddressindex.version()
            + height_to_first_p2wshaddressindex.version()
            + height_to_timestamp_fixed.version()
            + height_to_output_count.version()
            + height_to_input_count.version()
            + txinindex_to_txoutindex.version()
            + txoutindex_to_value.version()
            + txindex_to_height.version()
            + txoutindex_to_txindex.version()
            + txoutindex_to_outputtype.version()
            + txoutindex_to_typeindex.version()
            + height_to_unclaimed_rewards.version()
            + height_to_price_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + dateindex_to_price_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + height_to_date_fixed.version()
            + dateindex_to_first_height.version()
            + dateindex_to_height_count.version();

        let mut separate_utxo_vecs = self.utxo_cohorts.iter_separate_mut().collect::<Vec<_>>();
        let mut separate_address_vecs =
            self.address_cohorts.iter_separate_mut().collect::<Vec<_>>();

        separate_utxo_vecs
            .par_iter_mut()
            .try_for_each(|Filtered(_, v)| v.validate_computed_versions(base_version))?;
        separate_address_vecs
            .par_iter_mut()
            .try_for_each(|Filtered(_, v)| v.validate_computed_versions(base_version))?;
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
            .map(|Filtered(_, v)| Height::from(v.min_height_vecs_len()))
            .min()
            .unwrap_or_default()
            .min(
                separate_address_vecs
                    .par_iter_mut()
                    .map(|Filtered(_, v)| Height::from(v.min_height_vecs_len()))
                    .min()
                    .unwrap_or_default(),
            )
            .min(chain_state_starting_height)
            .min(Height::from(self.p2pk33addressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2pk65addressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2pkhaddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2shaddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2traddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2wpkhaddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2wshaddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.p2aaddressindex_to_anyaddressindex.stamp()).incremented())
            .min(Height::from(self.loadedaddressindex_to_loadedaddressdata.stamp()).incremented())
            .min(Height::from(self.emptyaddressindex_to_emptyaddressdata.stamp()).incremented())
            .min(Height::from(self.height_to_unspendable_supply.len()))
            .min(Height::from(self.height_to_opreturn_supply.len()))
            .cmp(&chain_state_starting_height)
        {
            Ordering::Greater => unreachable!(),
            Ordering::Equal => chain_state_starting_height,
            Ordering::Less => Height::ZERO,
        };

        // info!("stateful_starting_height = {stateful_starting_height}");
        // let stateful_starting_height = stateful_starting_height
        //     .checked_sub(Height::new(1))
        //     .unwrap_or_default();
        // info!("stateful_starting_height = {stateful_starting_height}");

        let starting_height = starting_indexes.height.min(stateful_starting_height);
        // info!("starting_height = {starting_height}");
        let last_height = Height::from(indexer.vecs.height_to_blockhash.stamp());
        // info!("last_height = {last_height}");
        if starting_height <= last_height {
            // info!("starting_height = {starting_height}");

            let starting_height = if starting_height.is_not_zero() {
                let mut set = [
                    self.chain_state.rollback_before(starting_height.into())?,
                    self.p2pk33addressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2pk65addressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2pkhaddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2shaddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2traddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2wpkhaddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2wshaddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.p2aaddressindex_to_anyaddressindex
                        .rollback_before(starting_height.into())?,
                    self.loadedaddressindex_to_loadedaddressdata
                        .rollback_before(starting_height.into())?,
                    self.emptyaddressindex_to_emptyaddressdata
                        .rollback_before(starting_height.into())?,
                ]
                .into_iter()
                // .enumerate()
                // .map(|(i, s)| {
                //     let h = Height::from(s).incremented();
                //     // dbg!((i, s, h));
                //     h
                // })
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
                    .map(|Filtered(_, v)| v.import_state(starting_height).unwrap_or_default())
                    .all(|h| h == starting_height)
            {
                starting_height
            } else {
                Height::ZERO
            };
            // info!("starting_height = {starting_height}");

            let starting_height = if starting_height.is_not_zero()
                && separate_address_vecs
                    .iter_mut()
                    .map(|Filtered(_, v)| v.import_state(starting_height).unwrap_or_default())
                    .all(|h| h == starting_height)
            {
                starting_height
            } else {
                Height::ZERO
            };

            // info!("starting_height = {starting_height}");

            let mut chain_state: Vec<BlockState>;
            if starting_height.is_not_zero() {
                chain_state = self
                    .chain_state
                    .collect_range(None, None)
                    .into_iter()
                    .enumerate()
                    .map(|(height, supply)| {
                        let height = Height::from(height);
                        let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                        let price = height_to_price_close_iter
                            .as_mut()
                            .map(|i| *i.unwrap_get_inner(height));
                        BlockState {
                            timestamp,
                            price,
                            supply,
                        }
                    })
                    .collect::<Vec<_>>();
            } else {
                info!("Starting processing utxos from the start");

                // std::process::exit(0);

                chain_state = vec![];

                self.p2pk33addressindex_to_anyaddressindex.reset()?;
                self.p2pk65addressindex_to_anyaddressindex.reset()?;
                self.p2pkhaddressindex_to_anyaddressindex.reset()?;
                self.p2shaddressindex_to_anyaddressindex.reset()?;
                self.p2traddressindex_to_anyaddressindex.reset()?;
                self.p2wpkhaddressindex_to_anyaddressindex.reset()?;
                self.p2wshaddressindex_to_anyaddressindex.reset()?;
                self.p2aaddressindex_to_anyaddressindex.reset()?;
                self.loadedaddressindex_to_loadedaddressdata.reset()?;
                self.emptyaddressindex_to_emptyaddressdata.reset()?;

                separate_utxo_vecs
                    .par_iter_mut()
                    .try_for_each(|Filtered(_, v)| {
                        v.reset_state_starting_height();
                        v.state.as_mut().unwrap().reset_price_to_amount_if_needed()
                    })?;

                separate_address_vecs
                    .par_iter_mut()
                    .try_for_each(|Filtered(_, v)| {
                        v.reset_state_starting_height();
                        v.state.as_mut().unwrap().reset_price_to_amount_if_needed()
                    })?;
            }

            chain_state_starting_height = starting_height;

            starting_indexes.update_from_height(starting_height, indexes);

            let txinindex_to_txoutindex_reader = txinindex_to_txoutindex.create_reader();
            let txoutindex_to_value_reader = txoutindex_to_value.create_reader();
            let txoutindex_to_outputtype_reader = txoutindex_to_outputtype.create_reader();
            let txoutindex_to_typeindex_reader = txoutindex_to_typeindex.create_reader();

            let mut height_to_first_txoutindex_iter = height_to_first_txoutindex.into_iter();
            let mut height_to_first_txinindex_iter = height_to_first_txinindex.into_iter();
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
            let mut height_to_output_count_iter = height_to_output_count.into_iter();
            let mut height_to_input_count_iter = height_to_input_count.into_iter();
            let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
            let mut dateindex_to_price_close_iter =
                dateindex_to_price_close.as_ref().map(|v| v.into_iter());
            let mut height_to_date_fixed_iter = height_to_date_fixed.into_iter();
            let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
            let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();

            let height_to_price_close_vec =
                height_to_price_close.map(|height_to_price_close| height_to_price_close.collect());

            let height_to_timestamp_fixed_vec = height_to_timestamp_fixed.collect();
            let txoutindex_range_to_height = RangeMap::from(height_to_first_txoutindex);

            let mut unspendable_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_unspendable_supply
                    .into_iter()
                    .unwrap_get_inner(prev_height)
            } else {
                Sats::ZERO
            };
            let mut opreturn_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_opreturn_supply
                    .into_iter()
                    .unwrap_get_inner(prev_height)
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
                AddressTypeToTypeIndexTree::<WithAddressDataSource<LoadedAddressData>>::default();
            let mut addresstype_to_typeindex_to_emptyaddressdata =
                AddressTypeToTypeIndexTree::<WithAddressDataSource<EmptyAddressData>>::default();
            let mut addresstypeindex_to_anyaddressindex_reader_opt =
                ByAddressType::<Option<Reader>>::default();
            let mut anyaddressindex_to_anyaddressdata_reader_opt =
                ByAnyAddress::<Option<Reader>>::default();

            self.reset_readers_options(
                &mut addresstypeindex_to_anyaddressindex_reader_opt,
                &mut anyaddressindex_to_anyaddressdata_reader_opt,
            );

            let last_height = Height::from(
                height_to_date_fixed
                    .len()
                    .checked_sub(1)
                    .unwrap_or_default(),
            );

            (height.to_usize()..height_to_date_fixed.len())
                .map(Height::from)
                .try_for_each(|_height| -> Result<()> {
                    height = _height;

                    info!("Processing chain at {height}...");

                    self.utxo_cohorts
                        .iter_separate_mut()
                        .for_each(|Filtered(_, v)| v.state.as_mut().unwrap().reset_single_iteration_values());

                    self.address_cohorts
                        .iter_separate_mut()
                        .for_each(|Filtered(_, v)| v.state.as_mut().unwrap().reset_single_iteration_values());

                    let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                    let price = height_to_price_close_iter
                        .as_mut()
                        .map(|i| *i.unwrap_get_inner(height));
                    let first_txoutindex = height_to_first_txoutindex_iter
                        .unwrap_get_inner(height)
                        .to_usize();
                    let first_txinindex = height_to_first_txinindex_iter
                        .unwrap_get_inner(height)
                        .to_usize();
                    let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                    let input_count = height_to_input_count_iter.unwrap_get_inner(height);

                    let first_addressindexes: ByAddressType<TypeIndex> = ByAddressType {
                        p2a: height_to_first_p2aaddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2pk33: height_to_first_p2pk33addressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2pk65: height_to_first_p2pk65addressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2pkh: height_to_first_p2pkhaddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2sh: height_to_first_p2shaddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2tr: height_to_first_p2traddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2wpkh: height_to_first_p2wpkhaddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                        p2wsh: height_to_first_p2wshaddressindex_iter
                            .unwrap_get_inner(height)
                            .into(),
                    };

                    let (
                        mut transacted,
                        addresstype_to_typedindex_to_received_data,
                        mut height_to_sent,
                        addresstype_to_typedindex_to_sent_data,
                        mut stored_or_new_addresstype_to_typeindex_to_addressdatawithsource,
                    ) = thread::scope(|scope| {
                        scope.spawn(|| {
                            self.utxo_cohorts
                                .tick_tock_next_block(&chain_state, timestamp);
                        });

                        let (transacted, addresstype_to_typedindex_to_received_data, receiving_addresstype_to_typeindex_to_addressdatawithsource) = (first_txoutindex..first_txoutindex + usize::from(output_count))
                            .into_par_iter()
                            .map(TxOutIndex::from)
                            .map(|txoutindex| {
                                let value = txoutindex_to_value
                                    .unwrap_read(txoutindex, &txoutindex_to_value_reader);

                                let output_type = txoutindex_to_outputtype
                                    .unwrap_read(txoutindex, &txoutindex_to_outputtype_reader);

                                if output_type.is_not_address() {
                                    return (value, output_type, None);
                                }

                                let typeindex = txoutindex_to_typeindex
                                    .unwrap_read(txoutindex, &txoutindex_to_typeindex_reader);

                                let addressdata_opt = Self::get_addressdatawithsource(
                                    output_type,
                                    typeindex,
                                    &first_addressindexes,
                                    &addresstype_to_typeindex_to_loadedaddressdata,
                                    &addresstype_to_typeindex_to_emptyaddressdata,
                                    &addresstypeindex_to_anyaddressindex_reader_opt,
                                    &anyaddressindex_to_anyaddressdata_reader_opt,
                                    &self.p2pk33addressindex_to_anyaddressindex,
                                    &self.p2pk65addressindex_to_anyaddressindex,
                                    &self.p2pkhaddressindex_to_anyaddressindex,
                                    &self.p2shaddressindex_to_anyaddressindex,
                                    &self.p2traddressindex_to_anyaddressindex,
                                    &self.p2wpkhaddressindex_to_anyaddressindex,
                                    &self.p2wshaddressindex_to_anyaddressindex,
                                    &self.p2aaddressindex_to_anyaddressindex,
                                    &self.loadedaddressindex_to_loadedaddressdata,
                                    &self.emptyaddressindex_to_emptyaddressdata,
                                );

                                (value, output_type, Some((typeindex, addressdata_opt)))
                            }).fold(
                            || {
                                (
                                    Transacted::default(),
                                    AddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                    AddressTypeToTypeIndexTree::default()
                                )
                            },
                            |(mut transacted, mut addresstype_to_typedindex_to_data, mut addresstype_to_typeindex_to_addressdatawithsource),
                                (
                                value,
                                output_type,
                                typeindex_with_addressdata_opt,
                            )| {
                                transacted.iterate(value, output_type);

                                if let Some((typeindex, addressdata_opt)) = typeindex_with_addressdata_opt {
                                    if let Some(addressdata) = addressdata_opt
                                    {
                                        addresstype_to_typeindex_to_addressdatawithsource
                                            .get_mut(output_type)
                                            .unwrap()
                                            .insert(typeindex, addressdata);
                                    }

                                    addresstype_to_typedindex_to_data
                                        .get_mut(output_type)
                                        .unwrap()
                                        .push((typeindex, value));
                                }

                                (transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource)
                            }).reduce(
                                || {
                                    (
                                        Transacted::default(),
                                        AddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                        AddressTypeToTypeIndexTree::default()
                                    )
                                },
                                |(transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource), (transacted2, addresstype_to_typedindex_to_data2, addresstype_to_typeindex_to_addressdatawithsource2)| {
                                    (transacted + transacted2, addresstype_to_typedindex_to_data.merge(addresstype_to_typedindex_to_data2), addresstype_to_typeindex_to_addressdatawithsource.merge(addresstype_to_typeindex_to_addressdatawithsource2))
                                },
                            );

                        // Skip coinbase
                        let (height_to_sent, addresstype_to_typedindex_to_sent_data, sending_addresstype_to_typeindex_to_addressdatawithsource) = (first_txinindex + 1..first_txinindex + usize::from(input_count))
                            .into_par_iter()
                            .map(TxInIndex::from)
                            .map(|txinindex| {
                                let txoutindex =
                                    txinindex_to_txoutindex.unwrap_read(txinindex, &txinindex_to_txoutindex_reader);

                                let value = txoutindex_to_value
                                    .unwrap_read(txoutindex, &txoutindex_to_value_reader);

                                let input_type = txoutindex_to_outputtype
                                    .unwrap_read(txoutindex, &txoutindex_to_outputtype_reader);

                                let prev_height =
                                    *txoutindex_range_to_height.get(txoutindex).unwrap();

                                if input_type.is_not_address() {
                                    return (prev_height, value, input_type, None);
                                }

                                let typeindex = txoutindex_to_typeindex
                                    .unwrap_read(txoutindex, &txoutindex_to_typeindex_reader);

                                let addressdata_opt = Self::get_addressdatawithsource(
                                    input_type,
                                    typeindex,
                                    &first_addressindexes,
                                    &addresstype_to_typeindex_to_loadedaddressdata,
                                    &addresstype_to_typeindex_to_emptyaddressdata,
                                    &addresstypeindex_to_anyaddressindex_reader_opt,
                                    &anyaddressindex_to_anyaddressdata_reader_opt,
                                    &self.p2pk33addressindex_to_anyaddressindex,
                                    &self.p2pk65addressindex_to_anyaddressindex,
                                    &self.p2pkhaddressindex_to_anyaddressindex,
                                    &self.p2shaddressindex_to_anyaddressindex,
                                    &self.p2traddressindex_to_anyaddressindex,
                                    &self.p2wpkhaddressindex_to_anyaddressindex,
                                    &self.p2wshaddressindex_to_anyaddressindex,
                                    &self.p2aaddressindex_to_anyaddressindex,
                                    &self.loadedaddressindex_to_loadedaddressdata,
                                    &self.emptyaddressindex_to_emptyaddressdata,
                                );

                                (prev_height, value, input_type, Some((typeindex, addressdata_opt)))
                            }).fold(
                            || {
                                (
                                    BTreeMap::<Height, Transacted>::default(),
                                    HeightToAddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                    AddressTypeToTypeIndexTree::default()
                                )
                            },
                            |(mut height_to_transacted, mut height_to_addresstype_to_typedindex_to_data, mut addresstype_to_typeindex_to_addressdatawithsource),
                                (
                                prev_height,
                                value,
                                output_type,
                                typeindex_with_addressdata_opt,
                            )| {
                                height_to_transacted
                                    .entry(prev_height)
                                    .or_default()
                                    .iterate(value, output_type);

                                if let Some((typeindex, addressdata_opt)) = typeindex_with_addressdata_opt {
                                    if let Some(addressdata) = addressdata_opt
                                    {
                                        addresstype_to_typeindex_to_addressdatawithsource
                                            .get_mut(output_type)
                                            .unwrap()
                                            .insert(typeindex, addressdata);
                                    }

                                    height_to_addresstype_to_typedindex_to_data
                                        .entry(prev_height)
                                        .or_default()
                                        .get_mut(output_type)
                                        .unwrap()
                                        .push((typeindex, value));
                                }

                                (height_to_transacted, height_to_addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource)
                            }).reduce(
                                || {
                                    (
                                        BTreeMap::<Height, Transacted>::default(),
                                        HeightToAddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                        AddressTypeToTypeIndexTree::default()
                                    )
                                },
                                |(height_to_transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource), (height_to_transacted2, addresstype_to_typedindex_to_data2, addresstype_to_typeindex_to_addressdatawithsource2)| {
                                    let (mut height_to_transacted, height_to_transacted_consumed) = if height_to_transacted.len() > height_to_transacted2.len() {
                                        (height_to_transacted, height_to_transacted2)
                                    } else {
                                        (height_to_transacted2, height_to_transacted)
                                    };
                                    height_to_transacted_consumed.into_iter().for_each(|(k, v)| {
                                        *height_to_transacted.entry(k).or_default() += v;
                                    });

                                    let (mut addresstype_to_typedindex_to_data, addresstype_to_typedindex_to_data_consumed) = if addresstype_to_typedindex_to_data.len() > addresstype_to_typedindex_to_data2.len() {
                                        (addresstype_to_typedindex_to_data, addresstype_to_typedindex_to_data2)
                                    } else {
                                        (addresstype_to_typedindex_to_data2, addresstype_to_typedindex_to_data)
                                    };
                                    addresstype_to_typedindex_to_data_consumed.0.into_iter().for_each(|(k, v)| {
                                        addresstype_to_typedindex_to_data.entry(k).or_default().merge_mut(v);
                                    });

                                    (height_to_transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource.merge(addresstype_to_typeindex_to_addressdatawithsource2))
                                },
                            );

                        let addresstype_to_typeindex_to_addressdatawithsource =
                            receiving_addresstype_to_typeindex_to_addressdatawithsource
                                .merge(sending_addresstype_to_typeindex_to_addressdatawithsource);

                        (
                            transacted,
                            addresstype_to_typedindex_to_received_data,
                            height_to_sent,
                            addresstype_to_typedindex_to_sent_data,
                            addresstype_to_typeindex_to_addressdatawithsource,
                        )
                    });

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

                        if chain_state_starting_height > height {
                            dbg!(chain_state_starting_height, height);
                            panic!("temp, just making sure")
                        }

                        unspendable_supply += transacted
                            .by_type
                            .unspendable
                            .as_vec()
                            .into_iter()
                            .map(|state| state.value)
                            .sum::<Sats>()
                            + height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                        opreturn_supply += transacted.by_type.unspendable.opreturn.value;

                        if height == Height::new(0) {
                            transacted = Transacted::default();
                            unspendable_supply += Sats::FIFTY_BTC;
                        } else if height == Height::new(91_842) || height == Height::new(91_880) {
                            // Need to destroy invalid coinbases due to duplicate txids
                            if height == Height::new(91_842) {
                                height_to_sent.entry(Height::new(91_812)).or_default()
                            } else {
                                height_to_sent.entry(Height::new(91_722)).or_default()
                            }
                            .iterate(Sats::FIFTY_BTC, OutputType::P2PK65);
                        };

                        // Push current block state before processing sends and receives
                        chain_state.push(BlockState {
                            supply: transacted.spendable_supply.clone(),
                            price,
                            timestamp,
                        });

                        self.utxo_cohorts.receive(transacted, height, price);

                        self.utxo_cohorts.send(height_to_sent, &mut chain_state);
                    });

                    self.height_to_unspendable_supply.forced_push_at(
                        height,
                        unspendable_supply,
                        exit,
                    )?;

                    self.height_to_opreturn_supply
                        .forced_push_at(height, opreturn_supply, exit)?;

                    self.addresstype_to_height_to_addr_count.forced_push_at(
                        height,
                        &addresstype_to_addr_count,
                        exit,
                    )?;

                    self.addresstype_to_height_to_empty_addr_count
                        .forced_push_at(height, &addresstype_to_empty_addr_count, exit)?;

                    let date = height_to_date_fixed_iter.unwrap_get_inner(height);
                    let dateindex = DateIndex::try_from(date).unwrap();
                    let date_first_height =
                        dateindex_to_first_height_iter.unwrap_get_inner(dateindex);
                    let date_height_count =
                        dateindex_to_height_count_iter.unwrap_get_inner(dateindex);
                    let is_date_last_height = date_first_height
                        + Height::from(date_height_count).decremented().unwrap()
                        == height;
                    let date_price = dateindex_to_price_close_iter
                        .as_mut()
                        .map(|v| is_date_last_height.then(|| *v.unwrap_get_inner(dateindex)));

                    let dateindex = is_date_last_height.then_some(dateindex);

                    self.utxo_cohorts.iter_separate_mut().par_bridge()
                        .map(|Filtered(_, v)| v as &mut dyn DynCohortVecs)
                        .chain(
                            self.address_cohorts.iter_separate_mut()
                                .par_bridge()
                                .map(|Filtered(_, v)| v as &mut dyn DynCohortVecs),
                        )
                        .try_for_each(|v| {
                            v.forced_pushed_at(height, exit)?;
                            v.compute_then_force_push_unrealized_states(
                                height, price, dateindex, date_price, exit,
                            )
                        })?;

                    if height != last_height && height != Height::ZERO && height.to_usize() % 10_000 == 0 {
                        let _lock = exit.lock();

                        addresstypeindex_to_anyaddressindex_reader_opt.take();
                        anyaddressindex_to_anyaddressdata_reader_opt.take();

                        self.flush_states(height, &chain_state, mem::take(&mut addresstype_to_typeindex_to_loadedaddressdata), mem::take(&mut addresstype_to_typeindex_to_emptyaddressdata), false, exit)?;

                        self.reset_readers_options(
                            &mut addresstypeindex_to_anyaddressindex_reader_opt,
                            &mut anyaddressindex_to_anyaddressdata_reader_opt,
                        );
                    }

                    Ok(())
                })?;

            let _lock = exit.lock();

            addresstypeindex_to_anyaddressindex_reader_opt.take();
            anyaddressindex_to_anyaddressdata_reader_opt.take();

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
                        .iter_typed()
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
                        .iter_typed()
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
                        .1
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
            .1
            .height_to_supply_value
            .bitcoin
            .clone();
        let dateindex_to_supply = self
            .utxo_cohorts
            .all
            .1
            .indexes_to_supply
            .bitcoin
            .dateindex
            .clone();
        let height_to_market_cap = self.height_to_market_cap.clone();
        let dateindex_to_market_cap = self
            .indexes_to_market_cap
            .as_ref()
            .map(|v| v.dateindex.as_ref().unwrap().clone());
        let height_to_realized_cap = self.utxo_cohorts.all.1.height_to_realized_cap.clone();
        let dateindex_to_realized_cap = self
            .utxo_cohorts
            .all
            .1
            .indexes_to_realized_cap
            .as_ref()
            .map(|v| v.dateindex.unwrap_last().clone());
        let dateindex_to_supply_ref = dateindex_to_supply.as_ref().unwrap();
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
        addresstype_to_typeindex_to_loadedaddressdata: &AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        addresstypeindex_to_anyaddressindex_reader_opt: &ByAddressType<Option<Reader>>,
        anyaddressindex_to_anyaddressdata_reader_opt: &ByAnyAddress<Option<Reader>>,
        p2pk33addressindex_to_anyaddressindex: &RawVec<P2PK33AddressIndex, AnyAddressIndex>,
        p2pk65addressindex_to_anyaddressindex: &RawVec<P2PK65AddressIndex, AnyAddressIndex>,
        p2pkhaddressindex_to_anyaddressindex: &RawVec<P2PKHAddressIndex, AnyAddressIndex>,
        p2shaddressindex_to_anyaddressindex: &RawVec<P2SHAddressIndex, AnyAddressIndex>,
        p2traddressindex_to_anyaddressindex: &RawVec<P2TRAddressIndex, AnyAddressIndex>,
        p2wpkhaddressindex_to_anyaddressindex: &RawVec<P2WPKHAddressIndex, AnyAddressIndex>,
        p2wshaddressindex_to_anyaddressindex: &RawVec<P2WSHAddressIndex, AnyAddressIndex>,
        p2aaddressindex_to_anyaddressindex: &RawVec<P2AAddressIndex, AnyAddressIndex>,
        loadedaddressindex_to_loadedaddressdata: &RawVec<LoadedAddressIndex, LoadedAddressData>,
        emptyaddressindex_to_emptyaddressdata: &RawVec<EmptyAddressIndex, EmptyAddressData>,
    ) -> Option<WithAddressDataSource<LoadedAddressData>> {
        if *first_addressindexes.get(address_type).unwrap() <= typeindex {
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

        let reader = addresstypeindex_to_anyaddressindex_reader_opt
            .get_unwrap(address_type)
            .as_ref()
            .unwrap();

        let anyaddressindex = match address_type {
            OutputType::P2PK33 => {
                p2pk33addressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2PK65 => {
                p2pk65addressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2PKH => {
                p2pkhaddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2SH => {
                p2shaddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2TR => {
                p2traddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2WPKH => {
                p2wpkhaddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2WSH => {
                p2wshaddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            OutputType::P2A => {
                p2aaddressindex_to_anyaddressindex.get_or_read(typeindex.into(), reader)
            }
            _ => unreachable!(),
        }
        .unwrap()
        .unwrap()
        .into_owned();

        Some(match anyaddressindex.to_enum() {
            AnyAddressDataIndexEnum::Loaded(loadedaddressindex) => {
                let mmap = anyaddressindex_to_anyaddressdata_reader_opt
                    .loaded
                    .as_ref()
                    .unwrap();

                let loadedaddressdata = loadedaddressindex_to_loadedaddressdata
                    .get_or_read(loadedaddressindex, mmap)
                    .unwrap()
                    .unwrap()
                    .into_owned();

                WithAddressDataSource::FromLoadedAddressDataVec((
                    loadedaddressindex,
                    loadedaddressdata,
                ))
            }
            AnyAddressDataIndexEnum::Empty(emtpyaddressindex) => {
                let mmap = anyaddressindex_to_anyaddressdata_reader_opt
                    .empty
                    .as_ref()
                    .unwrap();

                let emptyaddressdata = emptyaddressindex_to_emptyaddressdata
                    .get_or_read(emtpyaddressindex, mmap)
                    .unwrap()
                    .unwrap()
                    .into_owned();

                WithAddressDataSource::FromEmptyAddressDataVec((
                    emtpyaddressindex,
                    emptyaddressdata.into(),
                ))
            }
        })
    }

    fn reset_readers_options(
        &self,
        addresstypeindex_to_anyaddressindex_reader_opt: &mut ByAddressType<Option<Reader>>,
        anyaddressindex_to_anyaddressdata_reader_opt: &mut ByAnyAddress<Option<Reader>>,
    ) {
        addresstypeindex_to_anyaddressindex_reader_opt
            .p2pk65
            .replace(
                self.p2pk65addressindex_to_anyaddressindex
                    .create_static_reader(),
            );
        addresstypeindex_to_anyaddressindex_reader_opt
            .p2pk33
            .replace(
                self.p2pk33addressindex_to_anyaddressindex
                    .create_static_reader(),
            );
        addresstypeindex_to_anyaddressindex_reader_opt
            .p2pkh
            .replace(
                self.p2pkhaddressindex_to_anyaddressindex
                    .create_static_reader(),
            );
        addresstypeindex_to_anyaddressindex_reader_opt.p2sh.replace(
            self.p2shaddressindex_to_anyaddressindex
                .create_static_reader(),
        );
        addresstypeindex_to_anyaddressindex_reader_opt
            .p2wpkh
            .replace(
                self.p2wpkhaddressindex_to_anyaddressindex
                    .create_static_reader(),
            );
        addresstypeindex_to_anyaddressindex_reader_opt
            .p2wsh
            .replace(
                self.p2wshaddressindex_to_anyaddressindex
                    .create_static_reader(),
            );
        addresstypeindex_to_anyaddressindex_reader_opt.p2tr.replace(
            self.p2traddressindex_to_anyaddressindex
                .create_static_reader(),
        );
        addresstypeindex_to_anyaddressindex_reader_opt.p2a.replace(
            self.p2aaddressindex_to_anyaddressindex
                .create_static_reader(),
        );
        anyaddressindex_to_anyaddressdata_reader_opt.loaded.replace(
            self.loadedaddressindex_to_loadedaddressdata
                .create_static_reader(),
        );
        anyaddressindex_to_anyaddressdata_reader_opt.empty.replace(
            self.emptyaddressindex_to_emptyaddressdata
                .create_static_reader(),
        );
    }

    fn flush_states(
        &mut self,
        height: Height,
        chain_state: &[BlockState],
        addresstype_to_typeindex_to_loadedaddressdata: AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        with_changes: bool,
        exit: &Exit,
    ) -> Result<()> {
        info!("Flushing...");

        self.utxo_cohorts
            .iter_separate_mut()
            .par_bridge()
            .try_for_each(|Filtered(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.address_cohorts
            .iter_separate_mut()
            .par_bridge()
            .try_for_each(|Filtered(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.height_to_unspendable_supply.safe_flush(exit)?;
        self.height_to_opreturn_supply.safe_flush(exit)?;
        self.addresstype_to_height_to_addr_count
            .iter_mut()
            .try_for_each(|v| v.safe_flush(exit))?;
        self.addresstype_to_height_to_empty_addr_count
            .iter_mut()
            .try_for_each(|v| v.safe_flush(exit))?;

        let mut addresstype_to_typeindex_to_new_or_updated_anyaddressindex =
            AddressTypeToTypeIndexTree::default();

        addresstype_to_typeindex_to_emptyaddressdata
            .unwrap()
            .into_iter_typed()
            .try_for_each(|(_type, tree)| -> Result<()> {
                tree.into_iter().try_for_each(
                    |(typeindex, emptyaddressdata_with_source)| -> Result<()> {
                        match emptyaddressdata_with_source {
                            WithAddressDataSource::New(emptyaddressdata) => {
                                let emptyaddressindex = self
                                    .emptyaddressindex_to_emptyaddressdata
                                    .fill_first_hole_or_push(emptyaddressdata)?;

                                let anyaddressindex = AnyAddressIndex::from(emptyaddressindex);

                                addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                                    .get_mut(_type)
                                    .unwrap()
                                    .insert(typeindex, anyaddressindex);

                                Ok(())
                            }
                            WithAddressDataSource::FromEmptyAddressDataVec((
                                emptyaddressindex,
                                emptyaddressdata,
                            )) => self
                                .emptyaddressindex_to_emptyaddressdata
                                .update(emptyaddressindex, emptyaddressdata)
                                .map_err(|e| e.into()),
                            WithAddressDataSource::FromLoadedAddressDataVec((
                                loadedaddressindex,
                                emptyaddressdata,
                            )) => {
                                self.loadedaddressindex_to_loadedaddressdata
                                    .delete(loadedaddressindex);

                                let emptyaddressindex = self
                                    .emptyaddressindex_to_emptyaddressdata
                                    .fill_first_hole_or_push(emptyaddressdata)?;

                                let anyaddressindex = emptyaddressindex.into();

                                addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                                    .get_mut(_type)
                                    .unwrap()
                                    .insert(typeindex, anyaddressindex);

                                Ok(())
                            }
                        }
                    },
                )
            })?;

        addresstype_to_typeindex_to_loadedaddressdata
            .unwrap()
            .into_iter_typed()
            .try_for_each(|(_type, tree)| -> Result<()> {
                tree.into_iter().try_for_each(
                    |(typeindex, loadedaddressdata_with_source)| -> Result<()> {
                        match loadedaddressdata_with_source {
                            WithAddressDataSource::New(loadedaddressdata) => {
                                let loadedaddressindex = self
                                    .loadedaddressindex_to_loadedaddressdata
                                    .fill_first_hole_or_push(loadedaddressdata)?;

                                let anyaddressindex = AnyAddressIndex::from(loadedaddressindex);

                                addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                                    .get_mut(_type)
                                    .unwrap()
                                    .insert(typeindex, anyaddressindex);

                                Ok(())
                            }
                            WithAddressDataSource::FromLoadedAddressDataVec((
                                loadedaddressindex,
                                loadedaddressdata,
                            )) => self
                                .loadedaddressindex_to_loadedaddressdata
                                .update(loadedaddressindex, loadedaddressdata)
                                .map_err(|e| e.into()),
                            WithAddressDataSource::FromEmptyAddressDataVec((
                                emptyaddressindex,
                                loadedaddressdata,
                            )) => {
                                self.emptyaddressindex_to_emptyaddressdata
                                    .delete(emptyaddressindex);

                                let loadedaddressindex = self
                                    .loadedaddressindex_to_loadedaddressdata
                                    .fill_first_hole_or_push(loadedaddressdata)?;

                                let anyaddressindex = loadedaddressindex.into();

                                addresstype_to_typeindex_to_new_or_updated_anyaddressindex
                                    .get_mut(_type)
                                    .unwrap()
                                    .insert(typeindex, anyaddressindex);

                                Ok(())
                            }
                        }
                    },
                )
            })?;

        addresstype_to_typeindex_to_new_or_updated_anyaddressindex
            .unwrap()
            .into_iter_typed()
            .try_for_each(|(_type, tree)| -> Result<()> {
                tree.into_iter()
                    .try_for_each(|(typeindex, anyaddressindex)| -> Result<()> {
                        match _type {
                            OutputType::P2PK33 => self
                                .p2pk33addressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2PK65 => self
                                .p2pk65addressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2PKH => self
                                .p2pkhaddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2SH => self
                                .p2shaddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2TR => self
                                .p2traddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2WPKH => self
                                .p2wpkhaddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2WSH => self
                                .p2wshaddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            OutputType::P2A => self
                                .p2aaddressindex_to_anyaddressindex
                                .update_or_push(typeindex.into(), anyaddressindex),
                            _ => unreachable!(),
                        }?;
                        Ok(())
                    })
            })?;

        let stamp = Stamp::from(height);

        self.p2pk33addressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pk65addressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pkhaddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2shaddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2traddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wpkhaddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wshaddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2aaddressindex_to_anyaddressindex
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.loadedaddressindex_to_loadedaddressdata
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.emptyaddressindex_to_emptyaddressdata
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

impl AddressTypeToVec<(TypeIndex, Sats)> {
    #[allow(clippy::too_many_arguments)]
    fn process_received(
        self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
    ) {
        self.unwrap().into_iter_typed().for_each(|(_type, vec)| {
            vec.into_iter().for_each(|(type_index, value)| {
                let mut is_new = false;
                let mut from_any_empty = false;

                let addressdata_withsource = addresstype_to_typeindex_to_loadedaddressdata
                    .get_mut(_type)
                    .unwrap()
                    .entry(type_index)
                    .or_insert_with(|| {
                        addresstype_to_typeindex_to_emptyaddressdata
                            .get_mut(_type)
                            .unwrap()
                            .remove(&type_index)
                            .map(|ad| {
                                from_any_empty = true;
                                ad.into()
                            })
                            .unwrap_or_else(|| {
                                let addressdata =
                                    stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                                        .get_mut(_type)
                                        .unwrap()
                                        .remove(&type_index)
                                        .unwrap();
                                is_new = addressdata.is_new();
                                from_any_empty = addressdata.is_from_emptyaddressdata();
                                addressdata
                            })
                    });

                if is_new || from_any_empty {
                    (*addresstype_to_addr_count.get_mut(_type).unwrap()) += 1;
                    if from_any_empty {
                        (*addresstype_to_empty_addr_count.get_mut(_type).unwrap()) -= 1;
                    }
                }

                let addressdata = addressdata_withsource.deref_mut();

                let prev_amount = addressdata.balance();

                let amount = prev_amount + value;

                if is_new
                    || from_any_empty
                    || vecs.amount_range.get_mut(amount).0.clone()
                        != vecs.amount_range.get_mut(prev_amount).0.clone()
                {
                    if !is_new && !from_any_empty {
                        vecs.amount_range
                            .get_mut(prev_amount)
                            .1
                            .state
                            .as_mut()
                            .unwrap()
                            .subtract(addressdata);
                    }

                    addressdata.receive(value, price);

                    vecs.amount_range
                        .get_mut(amount)
                        .1
                        .state
                        .as_mut()
                        .unwrap()
                        .add(addressdata);
                } else {
                    vecs.amount_range
                        .get_mut(amount)
                        .1
                        .state
                        .as_mut()
                        .unwrap()
                        .receive(addressdata, value, price);
                }
            });
        });
    }
}

impl HeightToAddressTypeToVec<(TypeIndex, Sats)> {
    #[allow(clippy::too_many_arguments)]
    fn process_sent(
        self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        height_to_price_close_vec: Option<&Vec<brk_structs::Close<Dollars>>>,
        height_to_timestamp_fixed_vec: &[Timestamp],
        height: Height,
        timestamp: Timestamp,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<LoadedAddressData>,
        >,
    ) -> Result<()> {
        self.0.into_iter().try_for_each(|(prev_height, v)| {
            let prev_price = height_to_price_close_vec
                .as_ref()
                .map(|v| **v.get(prev_height.to_usize()).unwrap());

            let prev_timestamp = *height_to_timestamp_fixed_vec
                .get(prev_height.to_usize())
                .unwrap();

            let blocks_old = height.to_usize() - prev_height.to_usize();

            let days_old = timestamp.difference_in_days_between_float(prev_timestamp);

            let older_than_hour = timestamp
                .checked_sub(prev_timestamp)
                .unwrap()
                .is_more_than_hour();

            v.unwrap().into_iter_typed().try_for_each(|(_type, vec)| {
                vec.into_iter().try_for_each(|(type_index, value)| {
                    let typeindex_to_loadedaddressdata =
                        addresstype_to_typeindex_to_loadedaddressdata
                            .get_mut(_type)
                            .unwrap();

                    let addressdata_withsource = typeindex_to_loadedaddressdata
                        .entry(type_index)
                        .or_insert_with(|| {
                            stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                                .get_mut(_type)
                                .unwrap()
                                .remove(&type_index)
                                .unwrap()
                        });

                    let addressdata = addressdata_withsource.deref_mut();

                    let prev_amount = addressdata.balance();

                    let amount = prev_amount.checked_sub(value).unwrap();

                    let will_be_empty = addressdata.utxo_count - 1 == 0;

                    if will_be_empty
                        || vecs.amount_range.get_mut(amount).0.clone()
                            != vecs.amount_range.get_mut(prev_amount).0.clone()
                    {
                        vecs.amount_range
                            .get_mut(prev_amount)
                            .1
                            .state
                            .as_mut()
                            .unwrap()
                            .subtract(addressdata);

                        addressdata.send(value, prev_price)?;

                        if will_be_empty {
                            if amount.is_not_zero() {
                                unreachable!()
                            }

                            (*addresstype_to_addr_count.get_mut(_type).unwrap()) -= 1;
                            (*addresstype_to_empty_addr_count.get_mut(_type).unwrap()) += 1;

                            let addressdata =
                                typeindex_to_loadedaddressdata.remove(&type_index).unwrap();

                            addresstype_to_typeindex_to_emptyaddressdata
                                .get_mut(_type)
                                .unwrap()
                                .insert(type_index, addressdata.into());
                        } else {
                            vecs.amount_range
                                .get_mut(amount)
                                .1
                                .state
                                .as_mut()
                                .unwrap()
                                .add(addressdata);
                        }
                    } else {
                        vecs.amount_range
                            .get_mut(amount)
                            .1
                            .state
                            .as_mut()
                            .unwrap()
                            .send(
                                addressdata,
                                value,
                                price,
                                prev_price,
                                blocks_old,
                                days_old,
                                older_than_hour,
                            )?;
                    }

                    Ok(())
                })
            })
        })
    }
}

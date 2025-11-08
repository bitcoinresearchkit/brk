use std::{cmp::Ordering, collections::BTreeSet, mem, path::Path, thread};

use brk_error::Result;
use brk_grouper::{ByAddressType, ByAnyAddress, Filtered};
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    AnyAddressDataIndexEnum, AnyAddressIndex, CheckedSub, DateIndex, Dollars, EmptyAddressData,
    EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex, OutputType, P2AAddressIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, Sats, StoredU64, Timestamp, TxInIndex, TxIndex,
    TxOutIndex, TypeIndex, Version,
};
use log::info;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use vecdb::{
    AnyCloneableIterableVec, AnyIterableVec, AnyStoredVec, AnyVec, BoxedVecIterator,
    CollectableVec, Database, EagerVec, Exit, Format, GenericStoredVec, ImportOptions,
    LazyVecFrom1, PAGE_SIZE, RawVec, Reader, Stamp, StoredIndex, VecIteratorExtended,
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

type TxIndexVec = SmallVec<[TxIndex; 4]>;

const VERSION: Version = Version::new(21);

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    // ---
    // States
    // ---
    pub chain_state: RawVec<Height, SupplyState>,
    pub any_address_indexes: AnyAddressIndexes,
    pub addresses_data: AddressesData,
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
                    |height: Height, iter| iter.get(height),
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

            any_address_indexes: AnyAddressIndexes {
                p2a: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pk33: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pk65: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2pkh: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2sh: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2tr: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2wpkh: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
                p2wsh: RawVec::forced_import_with(
                    ImportOptions::new(&db, "anyaddressindex", version + VERSION + Version::ZERO)
                        .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                )?,
            },
            addresses_data: AddressesData {
                loaded: loadedaddressindex_to_loadedaddressdata,
                empty: emptyaddressindex_to_emptyaddressdata,
            },
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
            .map(|price| price.timeindexes_to_price_close.dateindex.as_ref().unwrap());
        let height_to_date_fixed = &indexes.height_to_date_fixed;
        let height_to_first_p2aaddressindex = &indexer.vecs.height_to_first_p2aaddressindex;
        let height_to_first_p2pk33addressindex = &indexer.vecs.height_to_first_p2pk33addressindex;
        let height_to_first_p2pk65addressindex = &indexer.vecs.height_to_first_p2pk65addressindex;
        let height_to_first_p2pkhaddressindex = &indexer.vecs.height_to_first_p2pkhaddressindex;
        let height_to_first_p2shaddressindex = &indexer.vecs.height_to_first_p2shaddressindex;
        let height_to_first_p2traddressindex = &indexer.vecs.height_to_first_p2traddressindex;
        let height_to_first_p2wpkhaddressindex = &indexer.vecs.height_to_first_p2wpkhaddressindex;
        let height_to_first_p2wshaddressindex = &indexer.vecs.height_to_first_p2wshaddressindex;
        let height_to_first_txindex = &indexer.vecs.height_to_first_txindex;
        let height_to_first_txinindex = &indexer.vecs.height_to_first_txinindex;
        let height_to_first_txoutindex = &indexer.vecs.height_to_first_txoutindex;
        let height_to_input_count = chain.indexes_to_input_count.height.unwrap_sum();
        let height_to_output_count = chain.indexes_to_output_count.height.unwrap_sum();
        let height_to_price_close = price
            .as_ref()
            .map(|price| &price.chainindexes_to_price_close.height);
        let height_to_timestamp_fixed = &indexes.height_to_timestamp_fixed;
        let height_to_tx_count = chain.indexes_to_tx_count.height.as_ref().unwrap();
        let height_to_unclaimed_rewards = chain
            .indexes_to_unclaimed_rewards
            .sats
            .height
            .as_ref()
            .unwrap();
        let txindex_to_first_txoutindex = &indexer.vecs.txindex_to_first_txoutindex;
        let txindex_to_height = &indexer.vecs.txindex_to_height;
        let txindex_to_input_count = &indexes.txindex_to_input_count;
        let txindex_to_output_count = &indexes.txindex_to_output_count;
        let txinindex_to_outpoint = &indexer.vecs.txinindex_to_outpoint;
        let txoutindex_to_outputtype = &indexer.vecs.txoutindex_to_outputtype;
        let txoutindex_to_txindex = &indexer.vecs.txoutindex_to_txindex;
        let txoutindex_to_typeindex = &indexer.vecs.txoutindex_to_typeindex;
        let txoutindex_to_value = &indexer.vecs.txoutindex_to_value;

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

            let stamp = starting_height.into();
            let starting_height = if starting_height.is_not_zero() {
                let mut set = [self.chain_state.rollback_before(stamp)?]
                    .into_iter()
                    .chain(self.any_address_indexes.rollback_before(stamp)?)
                    .chain(self.addresses_data.rollback_before(stamp)?)
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

                // std::process::exit(0);

                chain_state = vec![];

                self.any_address_indexes.reset()?;
                self.addresses_data.reset()?;

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
            let mut txindex_to_input_count_iter = txindex_to_input_count.boxed_iter();
            let mut txindex_to_output_count_iter = txindex_to_output_count.boxed_iter();

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
                    .for_each(|Filtered(_, v)| {
                        v.state.as_mut().unwrap().reset_single_iteration_values()
                    });

                self.address_cohorts
                    .iter_separate_mut()
                    .for_each(|Filtered(_, v)| {
                        v.state.as_mut().unwrap().reset_single_iteration_values()
                    });

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
                ) = thread::scope(|scope| {
                    scope.spawn(|| {
                        self.utxo_cohorts
                            .tick_tock_next_block(&chain_state, timestamp);
                    });

                    let (transacted, addresstype_to_typedindex_to_received_data, receiving_addresstype_to_typeindex_to_addressdatawithsource) = (first_txoutindex..first_txoutindex + usize::from(output_count))
                        .into_par_iter()
                        .map(|i| {
                            let txoutindex = TxOutIndex::from(i);

                            let value = txoutindex_to_value
                                .read_unwrap(txoutindex, &ir.txoutindex_to_value);

                            let output_type = txoutindex_to_outputtype
                                .read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

                            if output_type.is_not_address() {
                                return (value, output_type, None);
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

                            (value, output_type, Some((typeindex, addressdata_opt)))
                        }).fold(
                        || {
                            (
                                Transacted::default(),
                                AddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                AddressTypeToTypeIndexMap::default()
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
                                        .insert_for_type(output_type, typeindex, addressdata);
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
                                    AddressTypeToTypeIndexMap::default()
                                )
                            },
                            |(transacted, addresstype_to_typedindex_to_data, addresstype_to_typeindex_to_addressdatawithsource), (transacted2, addresstype_to_typedindex_to_data2, addresstype_to_typeindex_to_addressdatawithsource2)| {
                                (transacted + transacted2, addresstype_to_typedindex_to_data.merge(addresstype_to_typedindex_to_data2), addresstype_to_typeindex_to_addressdatawithsource.merge(addresstype_to_typeindex_to_addressdatawithsource2))
                            },
                        );

                    // Skip coinbase
                    let (
                        height_to_sent,
                        addresstype_to_typedindex_to_sent_data,
                        sending_addresstype_to_typeindex_to_addressdatawithsource,
                    ) =
                        (first_txinindex + 1..first_txinindex + usize::from(input_count))
                            .into_par_iter()
                            .map(|i| {
                                let txinindex = TxInIndex::from(i);

                                let outpoint = txinindex_to_outpoint
                                    .read_unwrap(txinindex, &ir.txinindex_to_outpoint);

                                let txoutindex = txindex_to_first_txoutindex.read_unwrap(
                                    outpoint.txindex(),
                                    &ir.txindex_to_first_txoutindex,
                                ) + outpoint.vout();

                                let value = txoutindex_to_value
                                    .read_unwrap(txoutindex, &ir.txoutindex_to_value);

                                let input_type = txoutindex_to_outputtype
                                    .read_unwrap(txoutindex, &ir.txoutindex_to_outputtype);

                                let prev_height =
                                    *txoutindex_range_to_height.get(txoutindex).unwrap();

                                if input_type.is_not_address() {
                                    return (prev_height, value, input_type, None);
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
                                    )
                                },
                                |(
                                    mut height_to_transacted,
                                    mut height_to_addresstype_to_typedindex_to_data,
                                    mut addresstype_to_typeindex_to_addressdatawithsource,
                                ),
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

                                    if let Some((typeindex, addressdata_opt)) =
                                        typeindex_with_addressdata_opt
                                    {
                                        if let Some(addressdata) = addressdata_opt {
                                            addresstype_to_typeindex_to_addressdatawithsource
                                                .insert_for_type(
                                                    output_type,
                                                    typeindex,
                                                    addressdata,
                                                );
                                        }

                                        height_to_addresstype_to_typedindex_to_data
                                            .entry(prev_height)
                                            .or_default()
                                            .get_mut(output_type)
                                            .unwrap()
                                            .push((typeindex, value));
                                    }

                                    (
                                        height_to_transacted,
                                        height_to_addresstype_to_typedindex_to_data,
                                        addresstype_to_typeindex_to_addressdatawithsource,
                                    )
                                },
                            )
                            .reduce(
                                || {
                                    (
                                        FxHashMap::<Height, Transacted>::default(),
                                        HeightToAddressTypeToVec::<(TypeIndex, Sats)>::default(),
                                        AddressTypeToTypeIndexMap::default(),
                                    )
                                },
                                |(
                                    height_to_transacted,
                                    addresstype_to_typedindex_to_data,
                                    addresstype_to_typeindex_to_addressdatawithsource,
                                ),
                                 (
                                    height_to_transacted2,
                                    addresstype_to_typedindex_to_data2,
                                    addresstype_to_typeindex_to_addressdatawithsource2,
                                )| {
                                    let (mut height_to_transacted, height_to_transacted_consumed) =
                                        if height_to_transacted.len() > height_to_transacted2.len()
                                        {
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
                                        addresstype_to_typeindex_to_addressdatawithsource.merge(
                                            addresstype_to_typeindex_to_addressdatawithsource2,
                                        ),
                                    )
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
                        + height_to_unclaimed_rewards_iter.get_unwrap(height);

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

                self.height_to_unspendable_supply
                    .forced_push(height, unspendable_supply, exit)?;

                self.height_to_opreturn_supply
                    .forced_push(height, opreturn_supply, exit)?;

                self.addresstype_to_height_to_addr_count.forced_push(
                    height,
                    &addresstype_to_addr_count,
                    exit,
                )?;

                self.addresstype_to_height_to_empty_addr_count.forced_push(
                    height,
                    &addresstype_to_empty_addr_count,
                    exit,
                )?;

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
                    .map(|Filtered(_, v)| v as &mut dyn DynCohortVecs)
                    .chain(
                        self.address_cohorts
                            .par_iter_separate_mut()
                            .map(|Filtered(_, v)| v as &mut dyn DynCohortVecs),
                    )
                    .try_for_each(|v| {
                        v.forced_pushed_at(height, exit)?;
                        v.compute_then_force_push_unrealized_states(
                            height, price, dateindex, date_price, exit,
                        )
                    })?;

                if height != last_height
                    && height != Height::ZERO
                    && height.to_usize() % 10_000 == 0
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
        addresstype_to_typeindex_to_loadedaddressdata: &AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        vr: &VecsReaders,
        any_address_indexes: &AnyAddressIndexes,
        addresses_data: &AddressesData,
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

        let reader = vr.get_anyaddressindex_reader(address_type);

        let anyaddressindex =
            any_address_indexes.get_anyaddressindex(address_type, typeindex, reader);

        Some(match anyaddressindex.to_enum() {
            AnyAddressDataIndexEnum::Loaded(loadedaddressindex) => {
                let reader = &vr.anyaddressindex_to_anyaddressdata.loaded;

                let loadedaddressdata = addresses_data
                    .loaded
                    .get_or_read(loadedaddressindex, reader)
                    .unwrap()
                    .unwrap();

                WithAddressDataSource::FromLoadedAddressDataVec((
                    loadedaddressindex,
                    loadedaddressdata,
                ))
            }
            AnyAddressDataIndexEnum::Empty(emtpyaddressindex) => {
                let reader = &vr.anyaddressindex_to_anyaddressdata.empty;

                let emptyaddressdata = addresses_data
                    .empty
                    .get_or_read(emtpyaddressindex, reader)
                    .unwrap()
                    .unwrap();

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

        self.utxo_cohorts
            .par_iter_separate_mut()
            .try_for_each(|Filtered(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.address_cohorts
            .par_iter_separate_mut()
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

impl AddressTypeToVec<(TypeIndex, Sats)> {
    #[allow(clippy::too_many_arguments)]
    fn process_received(
        self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexMap<
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
                                        .remove_for_type(_type, &type_index);
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
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        height_to_price_close_vec: Option<&Vec<brk_types::Close<Dollars>>>,
        height_to_timestamp_fixed_vec: &[Timestamp],
        height: Height,
        timestamp: Timestamp,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexMap<
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
                        addresstype_to_typeindex_to_loadedaddressdata.get_mut_unwrap(_type);

                    let addressdata_withsource = typeindex_to_loadedaddressdata
                        .entry(type_index)
                        .or_insert_with(|| {
                            stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                                .remove_for_type(_type, &type_index)
                        });

                    let addressdata = addressdata_withsource.deref_mut();

                    let prev_amount = addressdata.balance();

                    let amount = prev_amount.checked_sub(value).unwrap();

                    let will_be_empty = addressdata.has_1_utxos();

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

#[derive(Clone, Traversable)]
pub struct AnyAddressIndexes {
    pub p2pk33: RawVec<P2PK33AddressIndex, AnyAddressIndex>,
    pub p2pk65: RawVec<P2PK65AddressIndex, AnyAddressIndex>,
    pub p2pkh: RawVec<P2PKHAddressIndex, AnyAddressIndex>,
    pub p2sh: RawVec<P2SHAddressIndex, AnyAddressIndex>,
    pub p2tr: RawVec<P2TRAddressIndex, AnyAddressIndex>,
    pub p2wpkh: RawVec<P2WPKHAddressIndex, AnyAddressIndex>,
    pub p2wsh: RawVec<P2WSHAddressIndex, AnyAddressIndex>,
    pub p2a: RawVec<P2AAddressIndex, AnyAddressIndex>,
}

impl AnyAddressIndexes {
    fn min_stamped_height(&self) -> Height {
        Height::from(self.p2pk33.stamp())
            .incremented()
            .min(Height::from(self.p2pk65.stamp()).incremented())
            .min(Height::from(self.p2pkh.stamp()).incremented())
            .min(Height::from(self.p2sh.stamp()).incremented())
            .min(Height::from(self.p2tr.stamp()).incremented())
            .min(Height::from(self.p2wpkh.stamp()).incremented())
            .min(Height::from(self.p2wsh.stamp()).incremented())
            .min(Height::from(self.p2a.stamp()).incremented())
    }

    fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 8]> {
        Ok([
            self.p2pk33.rollback_before(stamp)?,
            self.p2pk65.rollback_before(stamp)?,
            self.p2pkh.rollback_before(stamp)?,
            self.p2sh.rollback_before(stamp)?,
            self.p2tr.rollback_before(stamp)?,
            self.p2wpkh.rollback_before(stamp)?,
            self.p2wsh.rollback_before(stamp)?,
            self.p2a.rollback_before(stamp)?,
        ])
    }

    fn reset(&mut self) -> Result<()> {
        self.p2pk33.reset()?;
        self.p2pk65.reset()?;
        self.p2pkh.reset()?;
        self.p2sh.reset()?;
        self.p2tr.reset()?;
        self.p2wpkh.reset()?;
        self.p2wsh.reset()?;
        self.p2a.reset()?;
        Ok(())
    }

    fn get_anyaddressindex(
        &self,
        address_type: OutputType,
        typeindex: TypeIndex,
        reader: &Reader<'static>,
    ) -> AnyAddressIndex {
        let result = match address_type {
            OutputType::P2PK33 => self.p2pk33.get_or_read(typeindex.into(), reader),
            OutputType::P2PK65 => self.p2pk65.get_or_read(typeindex.into(), reader),
            OutputType::P2PKH => self.p2pkh.get_or_read(typeindex.into(), reader),
            OutputType::P2SH => self.p2sh.get_or_read(typeindex.into(), reader),
            OutputType::P2TR => self.p2tr.get_or_read(typeindex.into(), reader),
            OutputType::P2WPKH => self.p2wpkh.get_or_read(typeindex.into(), reader),
            OutputType::P2WSH => self.p2wsh.get_or_read(typeindex.into(), reader),
            OutputType::P2A => self.p2a.get_or_read(typeindex.into(), reader),
            _ => unreachable!(),
        };
        result.unwrap().unwrap()
    }

    fn update_or_push(
        &mut self,
        address_type: OutputType,
        typeindex: TypeIndex,
        anyaddressindex: AnyAddressIndex,
    ) -> Result<()> {
        (match address_type {
            OutputType::P2PK33 => self
                .p2pk33
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2PK65 => self
                .p2pk65
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2PKH => self.p2pkh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2SH => self.p2sh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2TR => self.p2tr.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2WPKH => self
                .p2wpkh
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2WSH => self.p2wsh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2A => self.p2a.update_or_push(typeindex.into(), anyaddressindex),
            _ => unreachable!(),
        })?;
        Ok(())
    }

    fn stamped_flush_maybe_with_changes(&mut self, stamp: Stamp, with_changes: bool) -> Result<()> {
        self.p2pk33
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pk65
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pkh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2sh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2tr
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wpkh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wsh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2a
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }
}

#[derive(Clone, Traversable)]
pub struct AddressesData {
    pub loaded: RawVec<LoadedAddressIndex, LoadedAddressData>,
    pub empty: RawVec<EmptyAddressIndex, EmptyAddressData>,
}

impl AddressesData {
    fn min_stamped_height(&self) -> Height {
        Height::from(self.loaded.stamp())
            .incremented()
            .min(Height::from(self.empty.stamp()).incremented())
    }

    fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 2]> {
        Ok([
            self.loaded.rollback_before(stamp)?,
            self.empty.rollback_before(stamp)?,
        ])
    }

    fn reset(&mut self) -> Result<()> {
        self.loaded.reset()?;
        self.empty.reset()?;
        Ok(())
    }

    fn stamped_flush_maybe_with_changes(&mut self, stamp: Stamp, with_changes: bool) -> Result<()> {
        self.loaded
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.empty
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }
}

struct IndexerReaders<'a> {
    txinindex_to_outpoint: Reader<'a>,
    txindex_to_first_txoutindex: Reader<'a>,
    txoutindex_to_value: Reader<'a>,
    txoutindex_to_outputtype: Reader<'a>,
    txoutindex_to_typeindex: Reader<'a>,
}

impl<'a> IndexerReaders<'a> {
    fn new(indexer: &'a Indexer) -> Self {
        Self {
            txinindex_to_outpoint: indexer.vecs.txinindex_to_outpoint.create_reader(),
            txindex_to_first_txoutindex: indexer.vecs.txindex_to_first_txoutindex.create_reader(),
            txoutindex_to_value: indexer.vecs.txoutindex_to_value.create_reader(),
            txoutindex_to_outputtype: indexer.vecs.txoutindex_to_outputtype.create_reader(),
            txoutindex_to_typeindex: indexer.vecs.txoutindex_to_typeindex.create_reader(),
        }
    }
}

struct VecsReaders {
    addresstypeindex_to_anyaddressindex: ByAddressType<Reader<'static>>,
    anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader<'static>>,
}

impl VecsReaders {
    fn new(vecs: &Vecs) -> Self {
        Self {
            addresstypeindex_to_anyaddressindex: ByAddressType {
                p2pk33: vecs.any_address_indexes.p2pk33.create_static_reader(),
                p2pk65: vecs.any_address_indexes.p2pk65.create_static_reader(),
                p2pkh: vecs.any_address_indexes.p2pkh.create_static_reader(),
                p2sh: vecs.any_address_indexes.p2sh.create_static_reader(),
                p2tr: vecs.any_address_indexes.p2tr.create_static_reader(),
                p2wpkh: vecs.any_address_indexes.p2wpkh.create_static_reader(),
                p2wsh: vecs.any_address_indexes.p2wsh.create_static_reader(),
                p2a: vecs.any_address_indexes.p2a.create_static_reader(),
            },
            anyaddressindex_to_anyaddressdata: ByAnyAddress {
                loaded: vecs.addresses_data.loaded.create_static_reader(),
                empty: vecs.addresses_data.empty.create_static_reader(),
            },
        }
    }

    fn get_anyaddressindex_reader(&self, address_type: OutputType) -> &Reader<'static> {
        self.addresstypeindex_to_anyaddressindex
            .get_unwrap(address_type)
    }
}

fn build_txoutindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_output_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let mut vec = Vec::new();

    let block_first_txindex = block_first_txindex.to_usize();
    for tx_offset in 0..block_tx_count as usize {
        let txindex = TxIndex::from(block_first_txindex + tx_offset);
        let output_count = u64::from(txindex_to_output_count.get_unwrap(txindex));

        for _ in 0..output_count {
            vec.push(txindex);
        }
    }

    vec
}

fn build_txinindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_input_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let mut vec = Vec::new();

    let block_first_txindex = block_first_txindex.to_usize();
    for tx_offset in 0..block_tx_count as usize {
        let txindex = TxIndex::from(block_first_txindex + tx_offset);
        let input_count = u64::from(txindex_to_input_count.get_unwrap(txindex));

        for _ in 0..input_count {
            vec.push(txindex);
        }
    }

    vec
}

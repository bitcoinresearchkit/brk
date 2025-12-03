use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, CheckedSub, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, FeeRate, HalvingEpoch,
    Height, MonthIndex, ONE_DAY_IN_SEC_F64, QuarterIndex, Sats, SemesterIndex, StoredBool,
    StoredF32, StoredF64, StoredU32, StoredU64, Timestamp, TxInIndex, TxIndex, TxOutIndex,
    TxVersion, Version, WeekIndex, Weight, YearIndex,
};
use vecdb::{
    Database, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec, IterableVec,
    LazyVecFrom1, LazyVecFrom2, PAGE_SIZE, PcoVec, TypedVecIterator, VecIndex, unlikely,
};

use crate::{
    grouped::{
        ComputedValueVecsFromHeight, ComputedValueVecsFromTxindex, ComputedVecsFromDateIndex,
        ComputedVecsFromHeight, ComputedVecsFromTxindex, Source, VecBuilderOptions,
    },
    utils::OptionExt,
};

use super::{Indexes, indexes, price};

const TARGET_BLOCKS_PER_DAY_F64: f64 = 144.0;
const TARGET_BLOCKS_PER_DAY_F32: f32 = 144.0;
const TARGET_BLOCKS_PER_DAY: u64 = 144;
const TARGET_BLOCKS_PER_WEEK: u64 = 7 * TARGET_BLOCKS_PER_DAY;
const TARGET_BLOCKS_PER_MONTH: u64 = 30 * TARGET_BLOCKS_PER_DAY;
const TARGET_BLOCKS_PER_QUARTER: u64 = 3 * TARGET_BLOCKS_PER_MONTH;
const TARGET_BLOCKS_PER_SEMESTER: u64 = 2 * TARGET_BLOCKS_PER_QUARTER;
const TARGET_BLOCKS_PER_YEAR: u64 = 2 * TARGET_BLOCKS_PER_SEMESTER;
const TARGET_BLOCKS_PER_DECADE: u64 = 10 * TARGET_BLOCKS_PER_YEAR;
const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub dateindex_to_block_count_target: LazyVecFrom1<DateIndex, StoredU64, DateIndex, DateIndex>,
    pub weekindex_to_block_count_target: LazyVecFrom1<WeekIndex, StoredU64, WeekIndex, WeekIndex>,
    pub monthindex_to_block_count_target:
        LazyVecFrom1<MonthIndex, StoredU64, MonthIndex, MonthIndex>,
    pub quarterindex_to_block_count_target:
        LazyVecFrom1<QuarterIndex, StoredU64, QuarterIndex, QuarterIndex>,
    pub semesterindex_to_block_count_target:
        LazyVecFrom1<SemesterIndex, StoredU64, SemesterIndex, SemesterIndex>,
    pub yearindex_to_block_count_target: LazyVecFrom1<YearIndex, StoredU64, YearIndex, YearIndex>,
    pub decadeindex_to_block_count_target:
        LazyVecFrom1<DecadeIndex, StoredU64, DecadeIndex, DecadeIndex>,
    pub height_to_interval: EagerVec<PcoVec<Height, Timestamp>>,
    pub height_to_24h_block_count: EagerVec<PcoVec<Height, StoredU32>>,
    pub height_to_24h_coinbase_sum: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_24h_coinbase_usd_sum: EagerVec<PcoVec<Height, Dollars>>,
    pub height_to_vbytes: EagerVec<PcoVec<Height, StoredU64>>,
    pub difficultyepoch_to_timestamp: EagerVec<PcoVec<DifficultyEpoch, Timestamp>>,
    pub halvingepoch_to_timestamp: EagerVec<PcoVec<HalvingEpoch, Timestamp>>,
    pub timeindexes_to_timestamp: ComputedVecsFromDateIndex<Timestamp>,
    pub indexes_to_block_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_1w_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1m_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1y_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_weight: ComputedVecsFromHeight<Weight>,
    pub indexes_to_difficulty: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_difficultyepoch: ComputedVecsFromDateIndex<DifficultyEpoch>,
    pub indexes_to_halvingepoch: ComputedVecsFromDateIndex<HalvingEpoch>,
    pub indexes_to_coinbase: ComputedValueVecsFromHeight,
    pub indexes_to_emptyoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_fee_rate: ComputedVecsFromTxindex<FeeRate>,
    /// Value == 0 when Coinbase
    pub txindex_to_input_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub indexes_to_sent: ComputedValueVecsFromHeight,
    // pub indexes_to_input_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredU64>,
    pub txindex_to_output_value: EagerVec<PcoVec<TxIndex, Sats>>,
    // pub indexes_to_output_value: ComputedVecsFromTxindex<Sats>,
    pub indexes_to_p2a_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2ms_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk33_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk65_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2sh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2tr_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wpkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wsh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight,
    pub indexes_to_tx_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<StoredU64>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub txinindex_to_value: EagerVec<PcoVec<TxInIndex, Sats>>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_vsize: LazyVecFrom1<TxIndex, StoredU64, TxIndex, Weight>,
    pub txindex_to_weight: LazyVecFrom2<TxIndex, Weight, TxIndex, StoredU32, TxIndex, StoredU32>,
    pub txindex_to_fee: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee_rate: EagerVec<PcoVec<TxIndex, FeeRate>>,
    pub indexes_to_exact_utxo_count: ComputedVecsFromHeight<StoredU64>,
    pub dateindex_to_fee_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_subsidy_dominance: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_subsidy_usd_1y_sma: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_puell_multiple: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_hash_rate: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_hash_rate_1w_sma: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_hash_rate_1m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_rate_2m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_rate_1y_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_price_ths: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_ths_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_phs: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_phs_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_rebound: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_ths: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_ths_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_phs: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_phs_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_rebound: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_difficulty_as_hash: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_difficulty_adjustment: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_blocks_before_next_difficulty_adjustment: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_days_before_next_difficulty_adjustment: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_blocks_before_next_halving: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_days_before_next_halving: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_inflation_rate: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_annualized_volume: ComputedVecsFromDateIndex<Sats>,
    pub indexes_to_annualized_volume_btc: ComputedVecsFromDateIndex<Bitcoin>,
    pub indexes_to_annualized_volume_usd: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_tx_btc_velocity: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_tx_usd_velocity: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_tx_per_sec: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_outputs_per_sec: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_inputs_per_sec: ComputedVecsFromDateIndex<StoredF32>,
}

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("chain"))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version + Version::ZERO;

        let compute_dollars = price.is_some();
        let v0 = Version::ZERO;
        let v2 = Version::TWO;
        let v4 = Version::new(4);
        let v5 = Version::new(5);

        // Helper macros for common patterns
        macro_rules! eager {
            ($name:expr) => {
                EagerVec::forced_import(&db, $name, version + v0)?
            };
            ($name:expr, $v:expr) => {
                EagerVec::forced_import(&db, $name, version + $v)?
            };
        }
        macro_rules! computed_h {
            ($name:expr, $source:expr, $opts:expr) => {
                ComputedVecsFromHeight::forced_import(&db, $name, $source, version + v0, indexes, $opts)?
            };
            ($name:expr, $source:expr, $v:expr, $opts:expr) => {
                ComputedVecsFromHeight::forced_import(&db, $name, $source, version + $v, indexes, $opts)?
            };
        }
        macro_rules! computed_di {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(&db, $name, Source::Compute, version + v0, indexes, $opts)?
            };
            ($name:expr, $v:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(&db, $name, Source::Compute, version + $v, indexes, $opts)?
            };
        }
        macro_rules! computed_tx {
            ($name:expr, $source:expr, $opts:expr) => {
                ComputedVecsFromTxindex::forced_import(&db, $name, $source, version + v0, indexes, $opts)?
            };
        }
        let last = || VecBuilderOptions::default().add_last();
        let sum = || VecBuilderOptions::default().add_sum();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();
        let stats = || VecBuilderOptions::default().add_average().add_minmax().add_percentiles();
        let full_stats = || VecBuilderOptions::default().add_average().add_minmax().add_percentiles().add_sum().add_cumulative();

        let txinindex_to_value: EagerVec<PcoVec<TxInIndex, Sats>> = eager!("value");

        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version + Version::ZERO,
            indexer.vecs.txindex_to_base_size.boxed_clone(),
            indexer.vecs.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.to_usize();
                txindex_to_base_size_iter.get_at(index).map(|base_size| {
                    let total_size = txindex_to_total_size_iter.get_at_unwrap(index);

                    // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                    let wu = usize::from(base_size) * 3 + usize::from(total_size);

                    Weight::from(bitcoin::Weight::from_wu_usize(wu))
                })
            },
        );

        let txindex_to_vsize = LazyVecFrom1::init(
            "vsize",
            version + Version::ZERO,
            txindex_to_weight.boxed_clone(),
            |index: TxIndex, iter| {
                iter.get(index).map(|weight| {
                    StoredU64::from(bitcoin::Weight::from(weight).to_vbytes_ceil() as usize)
                })
            },
        );

        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version + Version::ZERO,
            indexer.vecs.txindex_to_height.boxed_clone(),
            indexer.vecs.height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter.get(index).map(|height| {
                    let txindex = height_to_first_txindex_iter.get_unwrap(height);
                    StoredBool::from(index == txindex)
                })
            },
        );

        let txindex_to_input_value = eager!("input_value");
        let txindex_to_output_value = eager!("output_value");
        let txindex_to_fee = eager!("fee");
        let txindex_to_fee_rate = eager!("fee_rate");

        let dateindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.dateindex_to_dateindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DAY)),
        );
        let weekindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.weekindex_to_weekindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_WEEK)),
        );
        let monthindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.monthindex_to_monthindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_MONTH)),
        );
        let quarterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.quarterindex_to_quarterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_QUARTER)),
        );
        let semesterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.semesterindex_to_semesterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)),
        );
        let yearindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.yearindex_to_yearindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_YEAR)),
        );
        let decadeindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.decadeindex_to_decadeindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DECADE)),
        );

        let this = Self {
            dateindex_to_block_count_target,
            weekindex_to_block_count_target,
            monthindex_to_block_count_target,
            quarterindex_to_block_count_target,
            semesterindex_to_block_count_target,
            yearindex_to_block_count_target,
            decadeindex_to_block_count_target,
            height_to_interval: eager!("interval"),
            timeindexes_to_timestamp: computed_di!("timestamp", VecBuilderOptions::default().add_first()),
            indexes_to_block_interval: computed_h!("block_interval", Source::None, stats()),
            indexes_to_block_count: computed_h!("block_count", Source::Compute, sum_cum()),
            indexes_to_1w_block_count: computed_di!("1w_block_count", last()),
            indexes_to_1m_block_count: computed_di!("1m_block_count", last()),
            indexes_to_1y_block_count: computed_di!("1y_block_count", last()),
            indexes_to_block_weight: computed_h!("block_weight", Source::None, full_stats()),
            indexes_to_block_size: computed_h!("block_size", Source::None, full_stats()),
            height_to_vbytes: eager!("vbytes"),
            height_to_24h_block_count: eager!("24h_block_count"),
            height_to_24h_coinbase_sum: eager!("24h_coinbase_sum"),
            height_to_24h_coinbase_usd_sum: eager!("24h_coinbase_usd_sum"),
            indexes_to_block_vbytes: computed_h!("block_vbytes", Source::None, full_stats()),
            difficultyepoch_to_timestamp: eager!("timestamp"),
            halvingepoch_to_timestamp: eager!("timestamp"),

            dateindex_to_fee_dominance: eager!("fee_dominance"),
            dateindex_to_subsidy_dominance: eager!("subsidy_dominance"),
            indexes_to_difficulty: computed_h!("difficulty", Source::None, last()),
            indexes_to_difficultyepoch: computed_di!("difficultyepoch", last()),
            indexes_to_halvingepoch: computed_di!("halvingepoch", last()),
            indexes_to_tx_count: computed_h!("tx_count", Source::Compute, full_stats()),
            indexes_to_input_count: computed_tx!("input_count", Source::None, full_stats()),
            indexes_to_output_count: computed_tx!("output_count", Source::None, full_stats()),
            indexes_to_tx_v1: computed_h!("tx_v1", Source::Compute, sum_cum()),
            indexes_to_tx_v2: computed_h!("tx_v2", Source::Compute, sum_cum()),
            indexes_to_tx_v3: computed_h!("tx_v3", Source::Compute, sum_cum()),
            indexes_to_sent: ComputedValueVecsFromHeight::forced_import(
                &db,
                "sent",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                &db,
                "fee",
                indexer,
                indexes,
                Source::Vec(txindex_to_fee.boxed_clone()),
                version + Version::ZERO,
                price,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_fee_rate: computed_tx!("fee_rate", Source::None, stats()),
            indexes_to_tx_vsize: computed_tx!("tx_vsize", Source::None, stats()),
            indexes_to_tx_weight: computed_tx!("tx_weight", Source::None, stats()),
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                &db,
                "subsidy",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                &db,
                "coinbase",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight::forced_import(
                &db,
                "unclaimed_rewards",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_p2a_count: computed_h!("p2a_count", Source::Compute, full_stats()),
            indexes_to_p2ms_count: computed_h!("p2ms_count", Source::Compute, full_stats()),
            indexes_to_p2pk33_count: computed_h!("p2pk33_count", Source::Compute, full_stats()),
            indexes_to_p2pk65_count: computed_h!("p2pk65_count", Source::Compute, full_stats()),
            indexes_to_p2pkh_count: computed_h!("p2pkh_count", Source::Compute, full_stats()),
            indexes_to_p2sh_count: computed_h!("p2sh_count", Source::Compute, full_stats()),
            indexes_to_p2tr_count: computed_h!("p2tr_count", Source::Compute, full_stats()),
            indexes_to_p2wpkh_count: computed_h!("p2wpkh_count", Source::Compute, full_stats()),
            indexes_to_p2wsh_count: computed_h!("p2wsh_count", Source::Compute, full_stats()),
            indexes_to_opreturn_count: computed_h!("opreturn_count", Source::Compute, full_stats()),
            indexes_to_unknownoutput_count: computed_h!("unknownoutput_count", Source::Compute, full_stats()),
            indexes_to_emptyoutput_count: computed_h!("emptyoutput_count", Source::Compute, full_stats()),
            indexes_to_exact_utxo_count: computed_h!("exact_utxo_count", Source::Compute, last()),
            indexes_to_subsidy_usd_1y_sma: compute_dollars
                .then(|| ComputedVecsFromDateIndex::forced_import(&db, "subsidy_usd_1y_sma", Source::Compute, version + v0, indexes, last()))
                .transpose()?,
            indexes_to_puell_multiple: compute_dollars
                .then(|| ComputedVecsFromDateIndex::forced_import(&db, "puell_multiple", Source::Compute, version + v0, indexes, last()))
                .transpose()?,
            indexes_to_hash_rate: computed_h!("hash_rate", Source::Compute, v5, last()),
            indexes_to_hash_rate_1w_sma: computed_di!("hash_rate_1w_sma", last()),
            indexes_to_hash_rate_1m_sma: computed_di!("hash_rate_1m_sma", last()),
            indexes_to_hash_rate_2m_sma: computed_di!("hash_rate_2m_sma", last()),
            indexes_to_hash_rate_1y_sma: computed_di!("hash_rate_1y_sma", last()),
            indexes_to_difficulty_as_hash: computed_h!("difficulty_as_hash", Source::Compute, last()),
            indexes_to_difficulty_adjustment: computed_h!("difficulty_adjustment", Source::Compute, sum()),
            indexes_to_blocks_before_next_difficulty_adjustment: computed_h!("blocks_before_next_difficulty_adjustment", Source::Compute, v2, last()),
            indexes_to_days_before_next_difficulty_adjustment: computed_h!("days_before_next_difficulty_adjustment", Source::Compute, v2, last()),
            indexes_to_blocks_before_next_halving: computed_h!("blocks_before_next_halving", Source::Compute, v2, last()),
            indexes_to_days_before_next_halving: computed_h!("days_before_next_halving", Source::Compute, v2, last()),
            indexes_to_hash_price_ths: computed_h!("hash_price_ths", Source::Compute, v4, last()),
            indexes_to_hash_price_phs: computed_h!("hash_price_phs", Source::Compute, v4, last()),
            indexes_to_hash_value_ths: computed_h!("hash_value_ths", Source::Compute, v4, last()),
            indexes_to_hash_value_phs: computed_h!("hash_value_phs", Source::Compute, v4, last()),
            indexes_to_hash_price_ths_min: computed_h!("hash_price_ths_min", Source::Compute, v4, last()),
            indexes_to_hash_price_phs_min: computed_h!("hash_price_phs_min", Source::Compute, v4, last()),
            indexes_to_hash_price_rebound: computed_h!("hash_price_rebound", Source::Compute, v4, last()),
            indexes_to_hash_value_ths_min: computed_h!("hash_value_ths_min", Source::Compute, v4, last()),
            indexes_to_hash_value_phs_min: computed_h!("hash_value_phs_min", Source::Compute, v4, last()),
            indexes_to_hash_value_rebound: computed_h!("hash_value_rebound", Source::Compute, v4, last()),
            indexes_to_inflation_rate: computed_di!("inflation_rate", last()),
            indexes_to_annualized_volume: computed_di!("annualized_volume", last()),
            indexes_to_annualized_volume_btc: computed_di!("annualized_volume_btc", last()),
            indexes_to_annualized_volume_usd: computed_di!("annualized_volume_usd", last()),
            indexes_to_tx_btc_velocity: computed_di!("tx_btc_velocity", last()),
            indexes_to_tx_usd_velocity: computed_di!("tx_usd_velocity", last()),
            indexes_to_tx_per_sec: computed_di!("tx_per_sec", v2, last()),
            indexes_to_outputs_per_sec: computed_di!("outputs_per_sec", v2, last()),
            indexes_to_inputs_per_sec: computed_di!("inputs_per_sec", v2, last()),

            txindex_to_is_coinbase,
            txinindex_to_value,
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee,
            txindex_to_fee_rate,
            txindex_to_vsize,
            txindex_to_weight,

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

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, price, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.timeindexes_to_timestamp
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_date,
                    |(di, d, ..)| (di, Timestamp::from(d)),
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_fixed_iter = indexes.height_to_timestamp_fixed.into_iter();
        let mut prev = Height::ZERO;
        self.height_to_24h_block_count.compute_transform(
            starting_indexes.height,
            &indexes.height_to_timestamp_fixed,
            |(h, t, ..)| {
                while t.difference_in_days_between(height_to_timestamp_fixed_iter.get_unwrap(prev))
                    > 0
                {
                    prev.increment();
                    if prev > h {
                        unreachable!()
                    }
                }
                (h, StoredU32::from(*h + 1 - *prev))
            },
            exit,
        )?;

        self.indexes_to_block_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_range(
                    starting_indexes.height,
                    &indexer.vecs.height_to_weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1w_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_iter = indexer.vecs.height_to_timestamp.iter()?;
        self.height_to_interval.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_timestamp,
            |(height, timestamp, ..)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = height_to_timestamp_iter.get_unwrap(prev_h);
                    timestamp
                        .checked_sub(prev_timestamp)
                        .unwrap_or(Timestamp::ZERO)
                });
                (height, interval)
            },
            exit,
        )?;

        self.indexes_to_block_interval.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_interval),
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_weight),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_total_size),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            |(h, w, ..)| {
                (
                    h,
                    StoredU64::from(bitcoin::Weight::from(w).to_vbytes_floor()),
                )
            },
            exit,
        )?;

        self.indexes_to_block_vbytes.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_vbytes),
        )?;

        let mut height_to_timestamp_iter = indexer.vecs.height_to_timestamp.iter()?;

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            &indexes.difficultyepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        self.halvingepoch_to_timestamp.compute_transform(
            starting_indexes.halvingepoch,
            &indexes.halvingepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        let mut height_to_difficultyepoch_iter = indexes.height_to_difficultyepoch.into_iter();
        self.indexes_to_difficultyepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_difficultyepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_halvingepoch_iter = indexes.height_to_halvingepoch.into_iter();
        self.indexes_to_halvingepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_halvingepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_difficulty.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_difficulty),
        )?;

        self.indexes_to_tx_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_txindex,
                    &indexer.vecs.txindex_to_txid,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_input_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_input_count),
        )?;

        self.indexes_to_output_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_output_count),
        )?;

        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredU64>, txversion| {
                let mut txindex_to_txversion_iter = indexer.vecs.txindex_to_txversion.iter()?;
                indexes_to_tx_vany.compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_filtered_count_from_indexes(
                        starting_indexes.height,
                        &indexer.vecs.height_to_first_txindex,
                        &indexer.vecs.txindex_to_txid,
                        |txindex| {
                            let v = txindex_to_txversion_iter.get_unwrap(txindex);
                            v == txversion
                        },
                        exit,
                    )?;
                    Ok(())
                })
            };
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v1, TxVersion::ONE)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v2, TxVersion::TWO)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v3, TxVersion::THREE)?;

        // Because random reads are needed, reading directly from the mmap is faster than using buffered iterators
        let txoutindex_to_value = &indexer.vecs.txoutindex_to_value;
        let txoutindex_to_value_reader = indexer.vecs.txoutindex_to_value.create_reader();
        self.txinindex_to_value.compute_transform(
            starting_indexes.txinindex,
            &indexes.txinindex_to_txoutindex,
            |(txinindex, txoutindex, ..)| {
                let value = if txoutindex == TxOutIndex::COINBASE {
                    Sats::MAX
                } else {
                    txoutindex_to_value
                        .unchecked_read(txoutindex, &txoutindex_to_value_reader)
                        .unwrap()
                };
                (txinindex, value)
            },
            exit,
        )?;

        self.txindex_to_input_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.txindex_to_first_txinindex,
            &indexes.txindex_to_input_count,
            &self.txinindex_to_value,
            exit,
        )?;

        self.txindex_to_output_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.txindex_to_first_txoutindex,
            &indexes.txindex_to_output_count,
            &indexer.vecs.txoutindex_to_value,
            exit,
        )?;

        self.txindex_to_fee.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_input_value,
            &self.txindex_to_output_value,
            |(i, input, output, ..)| {
                let fee = if unlikely(input.is_max()) {
                    Sats::ZERO
                } else {
                    input - output
                };
                (i, fee)
            },
            exit,
        )?;

        self.txindex_to_fee_rate.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_fee,
            &self.txindex_to_vsize,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        self.indexes_to_sent
            .compute_all(indexes, price, starting_indexes, exit, |v| {
                v.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_txindex,
                    &indexes.height_to_txindex_count,
                    &self.txindex_to_input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_fee.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee),
            price,
        )?;

        self.indexes_to_fee_rate.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee_rate),
        )?;

        self.indexes_to_tx_weight.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_weight),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_vsize),
        )?;

        self.indexes_to_coinbase
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                let mut txindex_to_first_txoutindex_iter =
                    indexer.vecs.txindex_to_first_txoutindex.iter()?;
                let mut txindex_to_output_count_iter = indexes.txindex_to_output_count.iter();
                let mut txoutindex_to_value_iter = indexer.vecs.txoutindex_to_value.iter()?;
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_txindex,
                    |(height, txindex, ..)| {
                        let first_txoutindex = txindex_to_first_txoutindex_iter
                            .get_unwrap(txindex)
                            .to_usize();
                        let output_count = txindex_to_output_count_iter.get_unwrap(txindex);
                        let mut sats = Sats::ZERO;
                        (first_txoutindex..first_txoutindex + usize::from(output_count)).for_each(
                            |txoutindex| {
                                sats += txoutindex_to_value_iter
                                    .get_unwrap(TxOutIndex::from(txoutindex));
                            },
                        );
                        (height, sats)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_coinbase_iter = self
            .indexes_to_coinbase
            .sats
            .height
            .as_ref()
            .unwrap()
            .into_iter();
        self.height_to_24h_coinbase_sum.compute_transform(
            starting_indexes.height,
            &self.height_to_24h_block_count,
            |(h, count, ..)| {
                let range = *h - (*count - 1)..=*h;
                let sum = range
                    .map(Height::from)
                    .map(|h| height_to_coinbase_iter.get_unwrap(h))
                    .sum::<Sats>();
                (h, sum)
            },
            exit,
        )?;
        drop(height_to_coinbase_iter);

        if let Some(mut height_to_coinbase_iter) = self
            .indexes_to_coinbase
            .dollars
            .as_ref()
            .map(|c| c.height.u().into_iter())
        {
            self.height_to_24h_coinbase_usd_sum.compute_transform(
                starting_indexes.height,
                &self.height_to_24h_block_count,
                |(h, count, ..)| {
                    let range = *h - (*count - 1)..=*h;
                    let sum = range
                        .map(Height::from)
                        .map(|h| height_to_coinbase_iter.get_unwrap(h))
                        .sum::<Dollars>();
                    (h, sum)
                },
                exit,
            )?;
        }

        self.indexes_to_subsidy
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    self.indexes_to_coinbase.sats.height.u(),
                    self.indexes_to_fee.sats.height.unwrap_sum(),
                    |(height, coinbase, fees, ..)| {
                        (
                            height,
                            coinbase.checked_sub(fees).unwrap_or_else(|| {
                                dbg!(height, coinbase, fees);
                                panic!()
                            }),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_unclaimed_rewards.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_subsidy.sats.height.u(),
                    |(height, subsidy, ..)| {
                        let halving = HalvingEpoch::from(height);
                        let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_inflation_rate
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_subsidy.sats.dateindex.unwrap_sum(),
                    self.indexes_to_subsidy.sats.dateindex.unwrap_cumulative(),
                    |(i, subsidy_1d_sum, subsidy_cumulative, ..)| {
                        (
                            i,
                            (365.0 * *subsidy_1d_sum as f64 / *subsidy_cumulative as f64 * 100.0)
                                .into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2a_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2aaddressindex,
                    &indexer.vecs.p2aaddressindex_to_p2abytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2ms_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2msoutputindex,
                    &indexer.vecs.p2msoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk33_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pk33addressindex,
                    &indexer.vecs.p2pk33addressindex_to_p2pk33bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk65_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pk65addressindex,
                    &indexer.vecs.p2pk65addressindex_to_p2pk65bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2pkhaddressindex,
                    &indexer.vecs.p2pkhaddressindex_to_p2pkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2sh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2shaddressindex,
                    &indexer.vecs.p2shaddressindex_to_p2shbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2tr_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2traddressindex,
                    &indexer.vecs.p2traddressindex_to_p2trbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wpkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2wpkhaddressindex,
                    &indexer.vecs.p2wpkhaddressindex_to_p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wsh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_p2wshaddressindex,
                    &indexer.vecs.p2wshaddressindex_to_p2wshbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_opreturn_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_opreturnindex,
                    &indexer.vecs.opreturnindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_unknownoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_unknownoutputindex,
                    &indexer.vecs.unknownoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_emptyoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.height_to_first_emptyoutputindex,
                    &indexer.vecs.emptyoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_exact_utxo_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                let mut input_count_iter = self
                    .indexes_to_input_count
                    .height
                    .unwrap_cumulative()
                    .into_iter();
                let mut opreturn_count_iter = self
                    .indexes_to_opreturn_count
                    .height_extra
                    .unwrap_cumulative()
                    .into_iter();
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_output_count.height.unwrap_cumulative(),
                    |(h, output_count, ..)| {
                        let input_count = input_count_iter.get_unwrap(h);
                        let opreturn_count = opreturn_count_iter.get_unwrap(h);
                        let block_count = u64::from(h + 1_usize);
                        // -1 > genesis output is unspendable
                        let mut utxo_count =
                            *output_count - (*input_count - block_count) - *opreturn_count - 1;

                        // txid dup: e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468
                        // Block 91_722 https://mempool.space/block/00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e
                        // Block 91_880 https://mempool.space/block/00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721
                        //
                        // txid dup: d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599
                        // Block 91_812 https://mempool.space/block/00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f
                        // Block 91_842 https://mempool.space/block/00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec
                        //
                        // Warning: Dups invalidate the previous coinbase according to
                        // https://chainquery.com/bitcoin-cli/gettxoutsetinfo

                        if h >= Height::new(91_842) {
                            utxo_count -= 1;
                        }
                        if h >= Height::new(91_880) {
                            utxo_count -= 1;
                        }

                        (h, StoredU64::from(utxo_count))
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.dateindex_to_fee_dominance.compute_transform2(
            starting_indexes.dateindex,
            self.indexes_to_fee.sats.dateindex.unwrap_sum(),
            self.indexes_to_coinbase.sats.dateindex.unwrap_sum(),
            |(i, fee, coinbase, ..)| {
                (
                    i,
                    StoredF32::from(u64::from(fee) as f64 / u64::from(coinbase) as f64 * 100.0),
                )
            },
            exit,
        )?;
        self.dateindex_to_subsidy_dominance.compute_transform2(
            starting_indexes.dateindex,
            self.indexes_to_subsidy.sats.dateindex.unwrap_sum(),
            self.indexes_to_coinbase.sats.dateindex.unwrap_sum(),
            |(i, subsidy, coinbase, ..)| {
                (
                    i,
                    StoredF32::from(u64::from(subsidy) as f64 / u64::from(coinbase) as f64 * 100.0),
                )
            },
            exit,
        )?;

        self.indexes_to_difficulty_as_hash
            .compute_all(indexes, starting_indexes, exit, |v| {
                let multiplier = 2.0_f64.powi(32) / 600.0;
                v.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.height_to_difficulty,
                    |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_block_count,
                    self.indexes_to_difficulty_as_hash.height.u(),
                    |(i, block_count_sum, difficulty_as_hash, ..)| {
                        (
                            i,
                            StoredF64::from(
                                (f64::from(block_count_sum) / TARGET_BLOCKS_PER_DAY_F64)
                                    * f64::from(difficulty_as_hash),
                            ),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1w_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_2m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    2 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1y_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        if self.indexes_to_subsidy_usd_1y_sma.is_some() {
            let date_to_coinbase_usd_sum = self
                .indexes_to_coinbase
                .dollars
                .as_ref()
                .unwrap()
                .dateindex
                .unwrap_sum();

            self.indexes_to_subsidy_usd_1y_sma
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_sma(
                        starting_indexes.dateindex,
                        date_to_coinbase_usd_sum,
                        365,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_puell_multiple
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_divide(
                        starting_indexes.dateindex,
                        date_to_coinbase_usd_sum,
                        self.indexes_to_subsidy_usd_1y_sma
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        self.indexes_to_difficulty_adjustment.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_percentage_change(
                    starting_indexes.height,
                    &indexer.vecs.height_to_difficulty,
                    1,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_blocks_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_blocks_before_next_difficulty_adjustment
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_blocks_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_days_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_blocks_before_next_halving
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_hash_price_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_coinbase_usd_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (i, (*coinbase_sum / (*hashrate / ONE_TERA_HASH)).into())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    |(i, price, ..)| (i, (*price * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_coinbase_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (
                            i,
                            (*coinbase_sum as f64 / (*hashrate / ONE_TERA_HASH)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    |(i, value, ..)| (i, (*value * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    self.indexes_to_hash_price_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    self.indexes_to_hash_value_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent.sats.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume_btc
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent.bitcoin.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_annualized_volume_btc
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    self.indexes_to_subsidy
                        .bitcoin
                        .dateindex
                        .unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(indexes_to_sent) = self.indexes_to_sent.dollars.as_ref() {
            self.indexes_to_annualized_volume_usd
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_sum(
                        starting_indexes.dateindex,
                        indexes_to_sent.dateindex.unwrap_sum(),
                        365,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_tx_usd_velocity
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_divide(
                        starting_indexes.dateindex,
                        self.indexes_to_annualized_volume_usd
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        self.indexes_to_subsidy
                            .dollars
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_cumulative(),
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        self.indexes_to_tx_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_tx_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_inputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_input_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_outputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_output_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}

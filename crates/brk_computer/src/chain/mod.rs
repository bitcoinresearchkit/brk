mod compute;
mod import;

use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, FeeRate, HalvingEpoch, Height,
    MonthIndex, QuarterIndex, Sats, SemesterIndex, StoredBool, StoredF32, StoredF64, StoredU32,
    StoredU64, Timestamp, TxInIndex, TxIndex, VSize, WeekIndex, Weight, YearIndex,
};
use vecdb::{Database, EagerVec, LazyVecFrom1, LazyVecFrom2, PcoVec};

use crate::grouped::{
    ComputedValueVecsFromHeight, ComputedValueVecsFromTxindex, ComputedVecsFromDateIndex,
    ComputedVecsFromHeight, ComputedVecsFromTxindex,
};

pub(crate) const TARGET_BLOCKS_PER_DAY_F64: f64 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY_F32: f32 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY: u64 = 144;
pub(crate) const TARGET_BLOCKS_PER_WEEK: u64 = 7 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_MONTH: u64 = 30 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_QUARTER: u64 = 3 * TARGET_BLOCKS_PER_MONTH;
pub(crate) const TARGET_BLOCKS_PER_SEMESTER: u64 = 2 * TARGET_BLOCKS_PER_QUARTER;
pub(crate) const TARGET_BLOCKS_PER_YEAR: u64 = 2 * TARGET_BLOCKS_PER_SEMESTER;
pub(crate) const TARGET_BLOCKS_PER_DECADE: u64 = 10 * TARGET_BLOCKS_PER_YEAR;
pub(crate) const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub(crate) db: Database,

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
    pub indexes_to_sent_sum: ComputedValueVecsFromHeight,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredU64>,
    pub txindex_to_output_value: EagerVec<PcoVec<TxIndex, Sats>>,
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
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<VSize>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub txinindex_to_value: EagerVec<PcoVec<TxInIndex, Sats>>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_vsize: LazyVecFrom1<TxIndex, VSize, TxIndex, Weight>,
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

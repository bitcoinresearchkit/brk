use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, Height, MonthIndex, QuarterIndex, SemesterIndex, StoredU32, StoredU64,
    Timestamp, WeekIndex, Weight, YearIndex,
};
use vecdb::{EagerVec, LazyVecFrom1, PcoVec};

use crate::grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight};

/// Block-related metrics: count, interval, size, weight, vbytes
#[derive(Clone, Traversable)]
pub struct Vecs {
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
    pub height_to_vbytes: EagerVec<PcoVec<Height, StoredU64>>,
    pub indexes_to_block_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_1w_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1m_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1y_block_count: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_weight: ComputedVecsFromHeight<Weight>,
}

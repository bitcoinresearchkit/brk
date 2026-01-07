use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, StoredU32, StoredU64,
    WeekIndex, YearIndex,
};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedBlockSumCum, ComputedDateLast};

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
    pub height_to_24h_block_count: vecdb::EagerVec<vecdb::PcoVec<brk_types::Height, StoredU32>>,
    pub indexes_to_block_count: ComputedBlockSumCum<StoredU32>,
    pub indexes_to_1w_block_count: ComputedDateLast<StoredU32>,
    pub indexes_to_1m_block_count: ComputedDateLast<StoredU32>,
    pub indexes_to_1y_block_count: ComputedDateLast<StoredU32>,
}

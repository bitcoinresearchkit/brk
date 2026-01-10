use brk_traversable::Traversable;
use brk_types::{Height, StoredU32, StoredU64};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedFromHeightLast, ComputedFromHeightSumCum, LazyFromDate};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub block_count_target: LazyFromDate<StoredU64>,
    pub block_count: ComputedFromHeightSumCum<StoredU32>,
    // Rolling window starts (height-indexed only, no date aggregation needed)
    pub _24h_start: EagerVec<PcoVec<Height, Height>>,
    pub _1w_start: EagerVec<PcoVec<Height, Height>>,
    pub _1m_start: EagerVec<PcoVec<Height, Height>>,
    pub _1y_start: EagerVec<PcoVec<Height, Height>>,
    // Rolling window block counts
    pub _24h_block_count: ComputedFromHeightLast<StoredU32>,
    pub _1w_block_count: ComputedFromHeightLast<StoredU32>,
    pub _1m_block_count: ComputedFromHeightLast<StoredU32>,
    pub _1y_block_count: ComputedFromHeightLast<StoredU32>,
}

use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{Database, LazyVecFrom2};

use super::{burned, velocity};
use crate::internal::{
    ComputedFromDateAverage, ComputedFromDateLast, LazyFromHeightLast, LazyValueFromHeightLast,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub circulating: LazyValueFromHeightLast,
    pub burned: burned::Vecs,
    pub inflation: ComputedFromDateAverage<StoredF32>,
    pub velocity: velocity::Vecs,
    pub market_cap: Option<LazyFromHeightLast<Dollars>>,
    pub market_cap_growth_rate: ComputedFromDateLast<StoredF32>,
    pub realized_cap_growth_rate: ComputedFromDateLast<StoredF32>,
    pub cap_growth_rate_diff:
        LazyVecFrom2<DateIndex, StoredF32, DateIndex, StoredF32, DateIndex, StoredF32>,
}

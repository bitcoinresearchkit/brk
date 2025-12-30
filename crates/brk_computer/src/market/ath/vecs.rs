use brk_traversable::Traversable;
use brk_types::{Close, Dollars, Height, StoredF32, StoredU16};
use vecdb::{EagerVec, PcoVec};

use crate::grouped::{
    ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex, LazyVecsFromDateIndex,
};

/// All-time high related metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_price_ath: EagerVec<PcoVec<Height, Dollars>>,
    pub height_to_price_drawdown: EagerVec<PcoVec<Height, StoredF32>>,
    pub indexes_to_price_ath: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_drawdown: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub indexes_to_days_since_price_ath: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_days_between_price_aths: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_years_between_price_aths: LazyVecsFromDateIndex<StoredF32, StoredU16>,
}

use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32, StoredU16};

use crate::internal::{
    ComputedDateLast, ComputedHeightDateLast, LazyBinaryHeightDateLast, LazyDateLast,
};

/// All-time high related metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_ath: ComputedHeightDateLast<Dollars>,
    pub price_drawdown: LazyBinaryHeightDateLast<StoredF32, Close<Dollars>, Dollars>,
    pub days_since_price_ath: ComputedDateLast<StoredU16>,
    pub years_since_price_ath: LazyDateLast<StoredF32, StoredU16>,
    pub max_days_between_price_aths: ComputedDateLast<StoredU16>,
    pub max_years_between_price_aths: LazyDateLast<StoredF32, StoredU16>,
}

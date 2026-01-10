use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32, StoredU16};

use crate::internal::{
    ComputedFromDateLast, ComputedFromHeightAndDateLast, LazyBinaryFromHeightAndDateLast, LazyFromDateLast,
};

/// All-time high related metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_ath: ComputedFromHeightAndDateLast<Dollars>,
    pub price_drawdown: LazyBinaryFromHeightAndDateLast<StoredF32, Close<Dollars>, Dollars>,
    pub days_since_price_ath: ComputedFromDateLast<StoredU16>,
    pub years_since_price_ath: LazyFromDateLast<StoredF32, StoredU16>,
    pub max_days_between_price_aths: ComputedFromDateLast<StoredU16>,
    pub max_years_between_price_aths: LazyFromDateLast<StoredF32, StoredU16>,
}

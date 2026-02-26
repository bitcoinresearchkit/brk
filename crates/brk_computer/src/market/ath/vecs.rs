use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32, StoredU16};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, LazyHeightDerivedLast, Price};

/// All-time high related metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_ath: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_drawdown: ComputedFromHeightLast<StoredF32, M>,
    pub days_since_price_ath: ComputedFromHeightLast<StoredU16, M>,
    pub years_since_price_ath: LazyHeightDerivedLast<StoredF32, StoredU16>,
    pub max_days_between_price_aths: ComputedFromHeightLast<StoredU16, M>,
    pub max_years_between_price_aths: LazyHeightDerivedLast<StoredF32, StoredU16>,
}

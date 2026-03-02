use brk_traversable::Traversable;
use brk_types::{Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, LazyHeightDerived, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_ath: Price<ComputedFromHeight<Cents, M>>,
    pub price_drawdown: ComputedFromHeight<StoredF32, M>,
    pub days_since_price_ath: ComputedFromHeight<StoredF32, M>,
    pub years_since_price_ath: LazyHeightDerived<StoredF32, StoredF32>,
    pub max_days_between_price_aths: ComputedFromHeight<StoredF32, M>,
    pub max_years_between_price_aths: LazyHeightDerived<StoredF32, StoredF32>,
}

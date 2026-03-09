use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, DerivedResolutions, PercentPerBlock, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_ath: Price<ComputedPerBlock<Cents, M>>,
    pub price_drawdown: PercentPerBlock<BasisPointsSigned16, M>,
    pub days_since_price_ath: ComputedPerBlock<StoredF32, M>,
    pub years_since_price_ath: DerivedResolutions<StoredF32, StoredF32>,
    pub max_days_between_price_ath: ComputedPerBlock<StoredF32, M>,
    pub max_years_between_price_ath: DerivedResolutions<StoredF32, StoredF32>,
}

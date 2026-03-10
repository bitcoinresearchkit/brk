use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, DerivedResolutions, PercentPerBlock, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price: Price<ComputedPerBlock<Cents, M>>,
    pub drawdown: PercentPerBlock<BasisPointsSigned16, M>,
    pub days_since: ComputedPerBlock<StoredF32, M>,
    pub years_since: DerivedResolutions<StoredF32, StoredF32>,
    pub max_days_between: ComputedPerBlock<StoredF32, M>,
    pub max_years_between: DerivedResolutions<StoredF32, StoredF32>,
}

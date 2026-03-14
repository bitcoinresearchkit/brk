use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock, PercentPerBlock, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub high: Price<PerBlock<Cents, M>>,
    pub drawdown: PercentPerBlock<BasisPointsSigned16, M>,
    pub days_since: PerBlock<StoredF32, M>,
    pub years_since: LazyPerBlock<StoredF32>,
    pub max_days_between: PerBlock<StoredF32, M>,
    pub max_years_between: LazyPerBlock<StoredF32>,
}

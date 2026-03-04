use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, PercentFromHeight, Price};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_min_1w: Price<ComputedFromHeight<Cents, M>>,
    pub price_max_1w: Price<ComputedFromHeight<Cents, M>>,
    pub price_min_2w: Price<ComputedFromHeight<Cents, M>>,
    pub price_max_2w: Price<ComputedFromHeight<Cents, M>>,
    pub price_min_1m: Price<ComputedFromHeight<Cents, M>>,
    pub price_max_1m: Price<ComputedFromHeight<Cents, M>>,
    pub price_min_1y: Price<ComputedFromHeight<Cents, M>>,
    pub price_max_1y: Price<ComputedFromHeight<Cents, M>>,
    pub price_true_range: ComputedFromHeight<StoredF32, M>,
    pub price_true_range_sum_2w: ComputedFromHeight<StoredF32, M>,
    pub price_choppiness_index_2w: PercentFromHeight<BasisPoints16, M>,
}

use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PercentPerBlock, Price};

#[derive(Traversable)]
pub struct PriceMinMaxVecs<M: StorageMode = Rw> {
    pub _1w: Price<ComputedPerBlock<Cents, M>>,
    pub _2w: Price<ComputedPerBlock<Cents, M>>,
    pub _1m: Price<ComputedPerBlock<Cents, M>>,
    pub _1y: Price<ComputedPerBlock<Cents, M>>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub min: PriceMinMaxVecs<M>,
    pub max: PriceMinMaxVecs<M>,
    pub true_range: ComputedPerBlock<StoredF32, M>,
    pub true_range_sum_2w: ComputedPerBlock<StoredF32, M>,
    pub choppiness_index_2w: PercentPerBlock<BasisPoints16, M>,
}

use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::PriceWithRatioExtendedPerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted: PriceWithRatioExtendedPerBlock<M>,
    pub active: PriceWithRatioExtendedPerBlock<M>,
    pub true_market_mean: PriceWithRatioExtendedPerBlock<M>,
    pub cointime: PriceWithRatioExtendedPerBlock<M>,
}

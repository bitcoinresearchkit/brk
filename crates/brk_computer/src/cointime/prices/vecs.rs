use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PriceWithRatioExtendedPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted: PriceWithRatioExtendedPerBlock<M>,
    pub active: PriceWithRatioExtendedPerBlock<M>,
    pub true_market_mean: PriceWithRatioExtendedPerBlock<M>,
    pub cointime: PriceWithRatioExtendedPerBlock<M>,
    pub transfer: PriceWithRatioExtendedPerBlock<M>,
    pub balanced: PriceWithRatioExtendedPerBlock<M>,
    pub terminal: PriceWithRatioExtendedPerBlock<M>,
    pub delta: PriceWithRatioExtendedPerBlock<M>,

    pub cumulative_market_cap: ComputedPerBlock<Dollars, M>,
}

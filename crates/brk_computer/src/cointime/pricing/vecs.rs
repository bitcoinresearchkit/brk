use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PriceWithRatioExtendedPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_price: PriceWithRatioExtendedPerBlock<M>,
    pub active_price: PriceWithRatioExtendedPerBlock<M>,
    pub true_market_mean: PriceWithRatioExtendedPerBlock<M>,
    pub cointime_price: PriceWithRatioExtendedPerBlock<M>,
    pub transfer_price: PriceWithRatioExtendedPerBlock<M>,
    pub balanced_price: PriceWithRatioExtendedPerBlock<M>,
    pub terminal_price: PriceWithRatioExtendedPerBlock<M>,
    pub delta_price: PriceWithRatioExtendedPerBlock<M>,

    pub cumulative_market_cap: ComputedPerBlock<Dollars, M>,
}

use brk_traversable::Traversable;
use brk_types::{Cents, Dollars};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, RatioPerBlockExtended, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_price: Price<ComputedPerBlock<Cents, M>>,
    pub vaulted_price_ratio: RatioPerBlockExtended<M>,
    pub active_price: Price<ComputedPerBlock<Cents, M>>,
    pub active_price_ratio: RatioPerBlockExtended<M>,
    pub true_market_mean: Price<ComputedPerBlock<Cents, M>>,
    pub true_market_mean_ratio: RatioPerBlockExtended<M>,
    pub cointime_price: Price<ComputedPerBlock<Cents, M>>,
    pub cointime_price_ratio: RatioPerBlockExtended<M>,
    pub transfer_price: Price<ComputedPerBlock<Cents, M>>,
    pub transfer_price_ratio: RatioPerBlockExtended<M>,
    pub balanced_price: Price<ComputedPerBlock<Cents, M>>,
    pub balanced_price_ratio: RatioPerBlockExtended<M>,
    pub terminal_price: Price<ComputedPerBlock<Cents, M>>,
    pub terminal_price_ratio: RatioPerBlockExtended<M>,
    pub delta_price: Price<ComputedPerBlock<Cents, M>>,
    pub delta_price_ratio: RatioPerBlockExtended<M>,

    pub cumulative_market_cap: ComputedPerBlock<Dollars, M>,
}

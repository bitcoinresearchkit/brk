use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightPriceWithRatioExtended, LazyFromHeight, Price};

/// Simple and exponential moving average metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_sma_1w: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_8d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_13d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_21d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_1m: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_34d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_55d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_89d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_111d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_144d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_200d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_350d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_1y: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_2y: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_200w: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_sma_4y: ComputedFromHeightPriceWithRatioExtended<M>,

    pub price_ema_1w: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_8d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_12d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_13d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_21d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_26d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_1m: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_34d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_55d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_89d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_144d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_200d: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_1y: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_2y: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_200w: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_ema_4y: ComputedFromHeightPriceWithRatioExtended<M>,

    pub price_sma_200d_x2_4: Price<LazyFromHeight<Cents, Cents>>,
    pub price_sma_200d_x0_8: Price<LazyFromHeight<Cents, Cents>>,
    pub price_sma_350d_x2: Price<LazyFromHeight<Cents, Cents>>,
}

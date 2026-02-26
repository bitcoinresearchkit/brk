use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightPriceWithRatioExtended, LazyFromHeightLast, Price};

/// Simple and exponential moving average metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_8d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_13d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_21d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_1m_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_34d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_55d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_89d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_111d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_144d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_200d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_350d_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_1y_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_2y_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_200w_sma: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_4y_sma: ComputedFromHeightPriceWithRatioExtended<M>,

    pub price_1w_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_8d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_12d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_13d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_21d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_26d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_1m_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_34d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_55d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_89d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_144d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_200d_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_1y_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_2y_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_200w_ema: ComputedFromHeightPriceWithRatioExtended<M>,
    pub price_4y_ema: ComputedFromHeightPriceWithRatioExtended<M>,

    pub price_200d_sma_x2_4: Price<LazyFromHeightLast<Dollars, Dollars>>,
    pub price_200d_sma_x0_8: Price<LazyFromHeightLast<Dollars, Dollars>>,
    pub price_350d_sma_x2: Price<LazyFromHeightLast<Dollars, Dollars>>,
}

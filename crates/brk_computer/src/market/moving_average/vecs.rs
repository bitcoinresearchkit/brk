use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightRatio, LazyPriceFromHeight};

/// Simple and exponential moving average metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_sma: ComputedFromHeightRatio<M>,
    pub price_8d_sma: ComputedFromHeightRatio<M>,
    pub price_13d_sma: ComputedFromHeightRatio<M>,
    pub price_21d_sma: ComputedFromHeightRatio<M>,
    pub price_1m_sma: ComputedFromHeightRatio<M>,
    pub price_34d_sma: ComputedFromHeightRatio<M>,
    pub price_55d_sma: ComputedFromHeightRatio<M>,
    pub price_89d_sma: ComputedFromHeightRatio<M>,
    pub price_111d_sma: ComputedFromHeightRatio<M>,
    pub price_144d_sma: ComputedFromHeightRatio<M>,
    pub price_200d_sma: ComputedFromHeightRatio<M>,
    pub price_350d_sma: ComputedFromHeightRatio<M>,
    pub price_1y_sma: ComputedFromHeightRatio<M>,
    pub price_2y_sma: ComputedFromHeightRatio<M>,
    pub price_200w_sma: ComputedFromHeightRatio<M>,
    pub price_4y_sma: ComputedFromHeightRatio<M>,

    pub price_1w_ema: ComputedFromHeightRatio<M>,
    pub price_8d_ema: ComputedFromHeightRatio<M>,
    pub price_12d_ema: ComputedFromHeightRatio<M>,
    pub price_13d_ema: ComputedFromHeightRatio<M>,
    pub price_21d_ema: ComputedFromHeightRatio<M>,
    pub price_26d_ema: ComputedFromHeightRatio<M>,
    pub price_1m_ema: ComputedFromHeightRatio<M>,
    pub price_34d_ema: ComputedFromHeightRatio<M>,
    pub price_55d_ema: ComputedFromHeightRatio<M>,
    pub price_89d_ema: ComputedFromHeightRatio<M>,
    pub price_144d_ema: ComputedFromHeightRatio<M>,
    pub price_200d_ema: ComputedFromHeightRatio<M>,
    pub price_1y_ema: ComputedFromHeightRatio<M>,
    pub price_2y_ema: ComputedFromHeightRatio<M>,
    pub price_200w_ema: ComputedFromHeightRatio<M>,
    pub price_4y_ema: ComputedFromHeightRatio<M>,

    pub price_200d_sma_x2_4: LazyPriceFromHeight<Dollars>,
    pub price_200d_sma_x0_8: LazyPriceFromHeight<Dollars>,
    pub price_350d_sma_x2: LazyPriceFromHeight<Dollars>,
}

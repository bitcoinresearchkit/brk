use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedFromDateRatio, LazyFromDateLast};

/// Simple and exponential moving average metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_sma: ComputedFromDateRatio,
    pub price_8d_sma: ComputedFromDateRatio,
    pub price_13d_sma: ComputedFromDateRatio,
    pub price_21d_sma: ComputedFromDateRatio,
    pub price_1m_sma: ComputedFromDateRatio,
    pub price_34d_sma: ComputedFromDateRatio,
    pub price_55d_sma: ComputedFromDateRatio,
    pub price_89d_sma: ComputedFromDateRatio,
    pub price_111d_sma: ComputedFromDateRatio,
    pub price_144d_sma: ComputedFromDateRatio,
    pub price_200d_sma: ComputedFromDateRatio,
    pub price_350d_sma: ComputedFromDateRatio,
    pub price_1y_sma: ComputedFromDateRatio,
    pub price_2y_sma: ComputedFromDateRatio,
    pub price_200w_sma: ComputedFromDateRatio,
    pub price_4y_sma: ComputedFromDateRatio,

    pub price_1w_ema: ComputedFromDateRatio,
    pub price_8d_ema: ComputedFromDateRatio,
    pub price_12d_ema: ComputedFromDateRatio,
    pub price_13d_ema: ComputedFromDateRatio,
    pub price_21d_ema: ComputedFromDateRatio,
    pub price_26d_ema: ComputedFromDateRatio,
    pub price_1m_ema: ComputedFromDateRatio,
    pub price_34d_ema: ComputedFromDateRatio,
    pub price_55d_ema: ComputedFromDateRatio,
    pub price_89d_ema: ComputedFromDateRatio,
    pub price_144d_ema: ComputedFromDateRatio,
    pub price_200d_ema: ComputedFromDateRatio,
    pub price_1y_ema: ComputedFromDateRatio,
    pub price_2y_ema: ComputedFromDateRatio,
    pub price_200w_ema: ComputedFromDateRatio,
    pub price_4y_ema: ComputedFromDateRatio,

    pub price_200d_sma_x2_4: LazyFromDateLast<Dollars>,
    pub price_200d_sma_x0_8: LazyFromDateLast<Dollars>,
    pub price_350d_sma_x2: LazyFromDateLast<Dollars>,
}

use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedRatioVecsDate, LazyDateLast};

/// Simple and exponential moving average metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_sma: ComputedRatioVecsDate,
    pub price_8d_sma: ComputedRatioVecsDate,
    pub price_13d_sma: ComputedRatioVecsDate,
    pub price_21d_sma: ComputedRatioVecsDate,
    pub price_1m_sma: ComputedRatioVecsDate,
    pub price_34d_sma: ComputedRatioVecsDate,
    pub price_55d_sma: ComputedRatioVecsDate,
    pub price_89d_sma: ComputedRatioVecsDate,
    pub price_111d_sma: ComputedRatioVecsDate,
    pub price_144d_sma: ComputedRatioVecsDate,
    pub price_200d_sma: ComputedRatioVecsDate,
    pub price_350d_sma: ComputedRatioVecsDate,
    pub price_1y_sma: ComputedRatioVecsDate,
    pub price_2y_sma: ComputedRatioVecsDate,
    pub price_200w_sma: ComputedRatioVecsDate,
    pub price_4y_sma: ComputedRatioVecsDate,

    pub price_1w_ema: ComputedRatioVecsDate,
    pub price_8d_ema: ComputedRatioVecsDate,
    pub price_12d_ema: ComputedRatioVecsDate,
    pub price_13d_ema: ComputedRatioVecsDate,
    pub price_21d_ema: ComputedRatioVecsDate,
    pub price_26d_ema: ComputedRatioVecsDate,
    pub price_1m_ema: ComputedRatioVecsDate,
    pub price_34d_ema: ComputedRatioVecsDate,
    pub price_55d_ema: ComputedRatioVecsDate,
    pub price_89d_ema: ComputedRatioVecsDate,
    pub price_144d_ema: ComputedRatioVecsDate,
    pub price_200d_ema: ComputedRatioVecsDate,
    pub price_1y_ema: ComputedRatioVecsDate,
    pub price_2y_ema: ComputedRatioVecsDate,
    pub price_200w_ema: ComputedRatioVecsDate,
    pub price_4y_ema: ComputedRatioVecsDate,

    pub price_200d_sma_x2_4: LazyDateLast<Dollars>,
    pub price_200d_sma_x0_8: LazyDateLast<Dollars>,
    pub price_350d_sma_x2: LazyDateLast<Dollars>,
}

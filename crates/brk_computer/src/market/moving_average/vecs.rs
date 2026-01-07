use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedRatioVecsDate, LazyDateLast};

/// Simple and exponential moving average metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_price_1w_sma: ComputedRatioVecsDate,
    pub indexes_to_price_8d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_13d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_21d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_1m_sma: ComputedRatioVecsDate,
    pub indexes_to_price_34d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_55d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_89d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_111d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_144d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_200d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_350d_sma: ComputedRatioVecsDate,
    pub indexes_to_price_1y_sma: ComputedRatioVecsDate,
    pub indexes_to_price_2y_sma: ComputedRatioVecsDate,
    pub indexes_to_price_200w_sma: ComputedRatioVecsDate,
    pub indexes_to_price_4y_sma: ComputedRatioVecsDate,

    pub indexes_to_price_1w_ema: ComputedRatioVecsDate,
    pub indexes_to_price_8d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_12d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_13d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_21d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_26d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_1m_ema: ComputedRatioVecsDate,
    pub indexes_to_price_34d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_55d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_89d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_144d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_200d_ema: ComputedRatioVecsDate,
    pub indexes_to_price_1y_ema: ComputedRatioVecsDate,
    pub indexes_to_price_2y_ema: ComputedRatioVecsDate,
    pub indexes_to_price_200w_ema: ComputedRatioVecsDate,
    pub indexes_to_price_4y_ema: ComputedRatioVecsDate,

    pub indexes_to_price_200d_sma_x2_4: LazyDateLast<Dollars>,
    pub indexes_to_price_200d_sma_x0_8: LazyDateLast<Dollars>,
    pub indexes_to_price_350d_sma_x2: LazyDateLast<Dollars>,
}

use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::grouped::{ComputedRatioVecsFromDateIndex, LazyVecsFromDateIndex};

/// Simple and exponential moving average metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_price_1w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_8d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_13d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_21d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1m_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_34d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_55d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_89d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_144d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_2y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_4y_sma: ComputedRatioVecsFromDateIndex,

    pub indexes_to_price_1w_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_8d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_13d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_21d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1m_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_34d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_55d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_89d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_144d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1y_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_2y_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200w_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_4y_ema: ComputedRatioVecsFromDateIndex,

    pub indexes_to_price_200d_sma_x2_4: LazyVecsFromDateIndex<Dollars>,
    pub indexes_to_price_200d_sma_x0_8: LazyVecsFromDateIndex<Dollars>,
}

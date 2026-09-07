use brk_types::{BoundedRatio, Cents};

use super::{Percentiles, PriceBands};

pub struct ModeResult {
    pub loss_threshold: Percentiles<BoundedRatio>,
    pub prices: PriceBands<Cents>,
}

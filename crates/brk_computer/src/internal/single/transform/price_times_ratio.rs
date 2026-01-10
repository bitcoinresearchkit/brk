use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// Dollars * StoredF32 -> Dollars (price × ratio)
pub struct PriceTimesRatio;

impl BinaryTransform<Dollars, StoredF32, Dollars> for PriceTimesRatio {
    #[inline(always)]
    fn apply(price: Dollars, ratio: StoredF32) -> Dollars {
        price * ratio
    }
}

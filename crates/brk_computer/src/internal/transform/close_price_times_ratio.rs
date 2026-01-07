use brk_types::{Close, Dollars, StoredF32};
use vecdb::BinaryTransform;

/// Close<Dollars> * StoredF32 -> Dollars (price × ratio)
/// Same as PriceTimesRatio but accepts Close<Dollars> price source.
pub struct ClosePriceTimesRatio;

impl BinaryTransform<Close<Dollars>, StoredF32, Dollars> for ClosePriceTimesRatio {
    #[inline(always)]
    fn apply(price: Close<Dollars>, ratio: StoredF32) -> Dollars {
        *price * ratio
    }
}

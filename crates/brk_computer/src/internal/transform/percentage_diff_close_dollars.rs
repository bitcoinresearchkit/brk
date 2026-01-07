use brk_types::{Close, Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Close<Dollars>, Dollars) -> StoredF32 percentage difference ((a/b - 1) × 100)
/// Used for DCA returns: (price / dca_average_price - 1) × 100
/// Also used for drawdown: (close / ath - 1) × 100 (note: drawdown is typically negative)
pub struct PercentageDiffCloseDollars;

impl BinaryTransform<Close<Dollars>, Dollars, StoredF32> for PercentageDiffCloseDollars {
    #[inline(always)]
    fn apply(close: Close<Dollars>, base: Dollars) -> StoredF32 {
        if base == Dollars::ZERO {
            StoredF32::default()
        } else {
            StoredF32::from((**close / *base - 1.0) * 100.0)
        }
    }
}

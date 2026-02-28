use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> StoredF32 percentage difference ((a/b - 1) × 100)
pub struct PercentageDiffDollars;

impl BinaryTransform<Dollars, Dollars, StoredF32> for PercentageDiffDollars {
    #[inline(always)]
    fn apply(close: Dollars, base: Dollars) -> StoredF32 {
        if base == Dollars::ZERO {
            StoredF32::default()
        } else {
            StoredF32::from((*close / *base - 1.0) * 100.0)
        }
    }
}

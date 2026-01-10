use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> StoredF32 ratio
/// Used for computing percentage ratios like profit/total, loss/total, etc.
pub struct Ratio32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for Ratio32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        StoredF32::from(numerator / denominator)
    }
}

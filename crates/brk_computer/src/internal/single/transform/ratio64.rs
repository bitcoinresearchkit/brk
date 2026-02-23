use brk_types::{Dollars, StoredF64};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> StoredF64 ratio
/// Used for computing ratios like SOPR where f64 precision is needed.
pub struct Ratio64;

impl BinaryTransform<Dollars, Dollars, StoredF64> for Ratio64 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF64 {
        numerator / denominator
    }
}

use brk_types::{Cents, StoredF64};
use vecdb::BinaryTransform;

/// (Cents, Cents) -> StoredF64 ratio
/// Used for computing ratios like SOPR where f64 precision is needed.
pub struct RatioCents64;

impl BinaryTransform<Cents, Cents, StoredF64> for RatioCents64 {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> StoredF64 {
        if denominator == Cents::ZERO {
            StoredF64::from(1.0)
        } else {
            StoredF64::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}

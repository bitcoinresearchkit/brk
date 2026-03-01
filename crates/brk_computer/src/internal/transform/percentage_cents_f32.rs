use brk_types::{Cents, StoredF32};
use vecdb::BinaryTransform;

/// (Cents, Cents) -> StoredF32 percentage (a/b × 100)
pub struct PercentageCentsF32;

impl BinaryTransform<Cents, Cents, StoredF32> for PercentageCentsF32 {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> StoredF32 {
        if denominator == Cents::ZERO {
            StoredF32::default()
        } else {
            StoredF32::from(numerator.inner() as f64 / denominator.inner() as f64 * 100.0)
        }
    }
}

use brk_types::{Cents, CentsSigned, StoredF32};
use vecdb::BinaryTransform;

/// (CentsSigned, Cents) -> StoredF32 percentage (a/b × 100)
/// For cross-type percentage when numerator is signed.
pub struct PercentageCentsSignedCentsF32;

impl BinaryTransform<CentsSigned, Cents, StoredF32> for PercentageCentsSignedCentsF32 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Cents) -> StoredF32 {
        if denominator == Cents::ZERO {
            StoredF32::default()
        } else {
            StoredF32::from(numerator.inner() as f64 / denominator.inner() as f64 * 100.0)
        }
    }
}

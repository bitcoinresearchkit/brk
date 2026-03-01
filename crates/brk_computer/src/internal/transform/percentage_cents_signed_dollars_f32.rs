use brk_types::{CentsSigned, Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (CentsSigned, Dollars) -> StoredF32 percentage (a/b × 100)
/// For cross-type percentage when numerator is CentsSigned and denominator is Dollars.
pub struct PercentageCentsSignedDollarsF32;

impl BinaryTransform<CentsSigned, Dollars, StoredF32> for PercentageCentsSignedDollarsF32 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Dollars) -> StoredF32 {
        if denominator == Dollars::ZERO {
            StoredF32::default()
        } else {
            StoredF32::from(numerator.inner() as f64 / *denominator * 100.0)
        }
    }
}

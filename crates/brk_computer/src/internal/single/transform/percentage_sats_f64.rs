use brk_types::{Sats, StoredF64};
use vecdb::BinaryTransform;

/// (Sats, Sats) -> StoredF64 percentage (a/b × 100)
/// Used for supply ratio calculations (equivalent to Bitcoin/Bitcoin since 1e8 cancels)
pub struct PercentageSatsF64;

impl BinaryTransform<Sats, Sats, StoredF64> for PercentageSatsF64 {
    #[inline(always)]
    fn apply(numerator: Sats, denominator: Sats) -> StoredF64 {
        StoredF64::from((*numerator as f64 / *denominator as f64) * 100.0)
    }
}

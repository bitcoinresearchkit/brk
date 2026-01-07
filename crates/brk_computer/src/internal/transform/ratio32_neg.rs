use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> -StoredF32 (negated ratio)
/// Computes -(a/b) directly to avoid lazy-from-lazy chains.
pub struct NegRatio32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for NegRatio32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        -StoredF32::from(numerator / denominator)
    }
}

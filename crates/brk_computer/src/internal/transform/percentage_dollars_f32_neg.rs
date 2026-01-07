use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> StoredF32 negated percentage (-(a/b × 100))
/// Used for negated loss ratio calculations, avoiding lazy-from-lazy chains.
pub struct NegPercentageDollarsF32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for NegPercentageDollarsF32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        // Dollars / Dollars returns StoredF64, so dereference and multiply
        StoredF32::from(-(*(numerator / denominator) * 100.0))
    }
}

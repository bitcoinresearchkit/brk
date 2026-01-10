use brk_types::{Dollars, StoredF32};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> StoredF32 percentage (a/b × 100)
/// Used for unrealized/realized ratio calculations
pub struct PercentageDollarsF32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for PercentageDollarsF32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        // Dollars / Dollars returns StoredF64, so dereference and multiply
        StoredF32::from(*(numerator / denominator) * 100.0)
    }
}

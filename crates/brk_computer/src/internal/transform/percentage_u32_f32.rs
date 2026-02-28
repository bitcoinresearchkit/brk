use brk_types::{StoredF32, StoredU32};
use vecdb::BinaryTransform;

/// (StoredU32, StoredU32) -> StoredF32 percentage (a/b × 100)
/// Used for pool dominance calculations (pool_blocks / total_blocks × 100)
pub struct PercentageU32F32;

impl BinaryTransform<StoredU32, StoredU32, StoredF32> for PercentageU32F32 {
    #[inline(always)]
    fn apply(numerator: StoredU32, denominator: StoredU32) -> StoredF32 {
        StoredF32::from((*numerator as f64 / *denominator as f64) * 100.0)
    }
}

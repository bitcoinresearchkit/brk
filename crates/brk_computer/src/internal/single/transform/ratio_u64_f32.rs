use brk_types::{StoredF32, StoredU64};
use vecdb::BinaryTransform;

/// (StoredU64, StoredU64) -> StoredF32 ratio (a/b)
/// Used for adoption ratio calculations (script_count / total_outputs)
pub struct RatioU64F32;

impl BinaryTransform<StoredU64, StoredU64, StoredF32> for RatioU64F32 {
    #[inline(always)]
    fn apply(numerator: StoredU64, denominator: StoredU64) -> StoredF32 {
        if *denominator > 0 {
            StoredF32::from(*numerator as f64 / *denominator as f64)
        } else {
            StoredF32::from(0.0)
        }
    }
}

use brk_types::{StoredF32, StoredU64};
use vecdb::BinaryTransform;

/// (StoredU64, StoredU64) -> StoredF32 percentage (a/b × 100)
/// Used for adoption ratio calculations (type_count / total_count × 100)
pub struct PercentageU64F32;

impl BinaryTransform<StoredU64, StoredU64, StoredF32> for PercentageU64F32 {
    #[inline(always)]
    fn apply(numerator: StoredU64, denominator: StoredU64) -> StoredF32 {
        if *denominator == 0 {
            StoredF32::default()
        } else {
            StoredF32::from((*numerator as f64 / *denominator as f64) * 100.0)
        }
    }
}

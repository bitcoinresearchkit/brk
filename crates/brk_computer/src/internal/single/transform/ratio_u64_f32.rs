//! (StoredU64, StoredU64) -> StoredF32 ratio

use brk_types::{StoredF32, StoredU64};
use vecdb::BinaryTransform;

/// (StoredU64, StoredU64) -> StoredF32 ratio (a/b)
pub struct RatioU64F32;

impl BinaryTransform<StoredU64, StoredU64, StoredF32> for RatioU64F32 {
    #[inline(always)]
    fn apply(numerator: StoredU64, denominator: StoredU64) -> StoredF32 {
        let num: f64 = (*numerator) as f64;
        let den: f64 = (*denominator) as f64;
        if den == 0.0 {
            StoredF32::from(0.0)
        } else {
            StoredF32::from(num / den)
        }
    }
}

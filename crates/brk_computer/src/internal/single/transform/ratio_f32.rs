use brk_types::StoredF32;
use vecdb::BinaryTransform;

/// (StoredF32, StoredF32) -> StoredF32 ratio (a / b)
pub struct RatioF32;

impl BinaryTransform<StoredF32, StoredF32, StoredF32> for RatioF32 {
    #[inline(always)]
    fn apply(a: StoredF32, b: StoredF32) -> StoredF32 {
        if *b == 0.0 {
            StoredF32::from(0.0)
        } else {
            StoredF32::from(*a / *b)
        }
    }
}

use brk_types::{StoredF32, StoredU16};
use vecdb::UnaryTransform;

/// StoredU16 / 365.0 -> StoredF32 (days to years conversion)
pub struct StoredU16ToYears;

impl UnaryTransform<StoredU16, StoredF32> for StoredU16ToYears {
    #[inline(always)]
    fn apply(v: StoredU16) -> StoredF32 {
        StoredF32::from(*v as f64 / 365.0)
    }
}

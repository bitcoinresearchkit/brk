use brk_types::StoredF32;
use vecdb::UnaryTransform;

pub struct DaysToYears;

impl UnaryTransform<StoredF32, StoredF32> for DaysToYears {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        StoredF32::from(*v / 365.0)
    }
}

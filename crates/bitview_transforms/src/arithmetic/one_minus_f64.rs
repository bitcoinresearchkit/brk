use brk_types::StoredF64;
use vecdb::UnaryTransform;

pub struct OneMinusF64;

impl UnaryTransform<StoredF64, StoredF64> for OneMinusF64 {
    #[inline(always)]
    fn apply(v: StoredF64) -> StoredF64 {
        StoredF64::from(1.0 - *v)
    }
}

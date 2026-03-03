use brk_types::{BasisPoints32, StoredF32};
use vecdb::UnaryTransform;

pub struct Bp32ToPercent;

impl UnaryTransform<BasisPoints32, StoredF32> for Bp32ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPoints32) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

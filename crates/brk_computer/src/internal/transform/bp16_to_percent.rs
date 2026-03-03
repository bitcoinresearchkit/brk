use brk_types::{BasisPoints16, StoredF32};
use vecdb::UnaryTransform;

pub struct Bp16ToPercent;

impl UnaryTransform<BasisPoints16, StoredF32> for Bp16ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPoints16) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

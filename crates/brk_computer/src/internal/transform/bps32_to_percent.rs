use brk_types::{BasisPointsSigned32, StoredF32};
use vecdb::UnaryTransform;

pub struct Bps32ToPercent;

impl UnaryTransform<BasisPointsSigned32, StoredF32> for Bps32ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned32) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

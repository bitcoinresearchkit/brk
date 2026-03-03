use brk_types::{BasisPointsSigned16, StoredF32};
use vecdb::UnaryTransform;

pub struct Bps16ToPercent;

impl UnaryTransform<BasisPointsSigned16, StoredF32> for Bps16ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned16) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

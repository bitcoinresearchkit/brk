use brk_types::{BasisPoints16, BasisPointsSigned16, BasisPointsSigned32, StoredF32};
use vecdb::UnaryTransform;

pub struct Bp16ToPercent;

impl UnaryTransform<BasisPoints16, StoredF32> for Bp16ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPoints16) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

pub struct Bps16ToPercent;

impl UnaryTransform<BasisPointsSigned16, StoredF32> for Bps16ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned16) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

pub struct Bps32ToPercent;

impl UnaryTransform<BasisPointsSigned32, StoredF32> for Bps32ToPercent {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned32) -> StoredF32 {
        StoredF32::from(bp.inner() as f32 / 100.0)
    }
}

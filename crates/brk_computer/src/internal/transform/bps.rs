use brk_types::{
    BasisPoints16, BasisPoints32, BasisPointsSigned16, BasisPointsSigned32, StoredF32,
};
use vecdb::UnaryTransform;

pub struct Bp16ToFloat;

impl UnaryTransform<BasisPoints16, StoredF32> for Bp16ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPoints16) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

pub struct Bp32ToFloat;

impl UnaryTransform<BasisPoints32, StoredF32> for Bp32ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPoints32) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

pub struct Bps16ToFloat;

impl UnaryTransform<BasisPointsSigned16, StoredF32> for Bps16ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned16) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

pub struct Bps32ToFloat;

impl UnaryTransform<BasisPointsSigned32, StoredF32> for Bps32ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned32) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

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

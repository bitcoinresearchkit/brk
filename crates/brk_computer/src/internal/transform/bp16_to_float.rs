use brk_types::{BasisPoints16, StoredF32};
use vecdb::UnaryTransform;

pub struct Bp16ToFloat;

impl UnaryTransform<BasisPoints16, StoredF32> for Bp16ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPoints16) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

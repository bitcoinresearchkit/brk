use brk_types::{BasisPointsSigned32, StoredF32};
use vecdb::UnaryTransform;

pub struct Bps32ToFloat;

impl UnaryTransform<BasisPointsSigned32, StoredF32> for Bps32ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned32) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

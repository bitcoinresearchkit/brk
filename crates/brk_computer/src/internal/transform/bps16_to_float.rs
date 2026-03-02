use brk_types::{BasisPointsSigned16, StoredF32};
use vecdb::UnaryTransform;

pub struct Bps16ToFloat;

impl UnaryTransform<BasisPointsSigned16, StoredF32> for Bps16ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPointsSigned16) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

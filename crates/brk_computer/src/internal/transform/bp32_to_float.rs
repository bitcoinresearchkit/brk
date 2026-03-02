use brk_types::{BasisPoints32, StoredF32};
use vecdb::UnaryTransform;

pub struct Bp32ToFloat;

impl UnaryTransform<BasisPoints32, StoredF32> for Bp32ToFloat {
    #[inline(always)]
    fn apply(bp: BasisPoints32) -> StoredF32 {
        StoredF32::from(bp.to_f32())
    }
}

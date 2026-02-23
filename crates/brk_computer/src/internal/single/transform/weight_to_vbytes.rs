use brk_types::{StoredU64, Weight};
use vecdb::UnaryTransform;

/// Weight -> StoredU64 virtual bytes (vbytes = ceil(weight/4))
pub struct WeightToVbytes;

impl UnaryTransform<Weight, StoredU64> for WeightToVbytes {
    #[inline(always)]
    fn apply(weight: Weight) -> StoredU64 {
        StoredU64::from(weight.to_vbytes_floor())
    }
}

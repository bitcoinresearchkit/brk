use brk_types::{StoredF32, Weight};
use vecdb::UnaryTransform;

/// Weight -> StoredF32 percentage (weight / MAX_BLOCK × 100)
/// Used for computing block fullness as a percentage of max capacity
pub struct WeightToFullness;

impl UnaryTransform<Weight, StoredF32> for WeightToFullness {
    #[inline(always)]
    fn apply(weight: Weight) -> StoredF32 {
        StoredF32::from(weight.fullness())
    }
}

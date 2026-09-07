use brk_types::{VSize, Weight};
use vecdb::UnaryTransform;

pub struct WeightToVSize;

impl UnaryTransform<Weight, VSize> for WeightToVSize {
    #[inline(always)]
    fn apply(weight: Weight) -> VSize {
        VSize::from(weight)
    }
}

use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{DeltaOp, LazyDeltaVec, VecValue};

use crate::{IndexSources, Resolutions};
use bitview_compute::NumericValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDeltaFromHeight<S, T, Op: 'static>
where
    S: VecValue,
    T: NumericValue + JsonSchema,
{
    pub height: LazyDeltaVec<Height, S, T, Op>,
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<S, T, Op> LazyDeltaFromHeight<S, T, Op>
where
    S: VecValue,
    T: NumericValue + JsonSchema,
    Op: DeltaOp<S, T>,
{
    pub fn new(
        name: &str,
        version: Version,
        height: LazyDeltaVec<Height, S, T, Op>,
        indexes: &IndexSources,
    ) -> Self {
        let resolutions = Resolutions::from_source(name, &height, version, indexes);

        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }
}

use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{DeltaSub, LazyDeltaVec, ReadableBoxedVec, ReadableCloneableVec};

use crate::{IndexSources, NumericValue, Resolutions};

/// A single lazy rolling-sum slot from height: the lazy delta vec + its resolution views.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyRollingSumFromHeight<T>
where
    T: NumericValue + JsonSchema,
{
    pub height: LazyDeltaVec<Height, T, T, DeltaSub>,
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<T: NumericValue + JsonSchema> LazyRollingSumFromHeight<T> {
    pub fn new(
        name: &str,
        version: Version,
        cumulative: ReadableBoxedVec<Height, T>,
        window_start: &impl ReadableCloneableVec<Height, Height>,
        indexes: &IndexSources,
    ) -> Self {
        let cached = window_start.read_only_boxed_clone();
        let height = LazyDeltaVec::new(name, version, cumulative, cached.version(), move || {
            cached.snapshot()
        });
        let resolutions = Resolutions::from_source(name, &height, version, indexes);
        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }
}

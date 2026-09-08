use bitview_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{DeltaAvg, LazyDeltaVec, ReadableBoxedVec, ReadableCloneableVec};

use crate::{IndexSources, Resolutions};
use bitview_compute::NumericValue;

/// A single lazy rolling-average slot from height: the lazy delta vec + its resolution views.
/// Output is always StoredF32 regardless of input type T.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyRollingAvgFromHeight<T>
where
    T: NumericValue + JsonSchema,
{
    pub height: LazyDeltaVec<Height, T, StoredF32, DeltaAvg>,
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<StoredF32>>,
}

impl<T: NumericValue + JsonSchema> LazyRollingAvgFromHeight<T> {
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

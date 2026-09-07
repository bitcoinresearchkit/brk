use bitview_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{DeltaAvg, LazyDeltaVec, ReadOnlyClone, ReadableBoxedVec};

use crate::{CachedWindowStartVec, IndexSources, NumericValue, Resolutions};

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
        cached_start: &CachedWindowStartVec,
        indexes: &IndexSources,
    ) -> Self {
        let cached = cached_start.read_only_clone();
        let height = LazyDeltaVec::new(name, version, cumulative, cached.version(), move || {
            cached.snapshot()
        });
        let resolutions = Resolutions::from_height_source(name, height.clone(), version, indexes);
        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }
}

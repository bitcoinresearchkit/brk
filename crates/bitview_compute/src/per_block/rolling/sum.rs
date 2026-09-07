use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{DeltaSub, LazyDeltaVec, ReadOnlyClone, ReadableBoxedVec};

use crate::{CachedWindowStartVec, IndexSources, NumericValue, Resolutions};

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

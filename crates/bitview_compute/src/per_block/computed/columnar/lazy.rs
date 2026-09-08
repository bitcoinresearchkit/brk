use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    Budgeted, CachedVec, ColumnId, Formattable, LazyColumnVec, PcoVec, PcoVecValue,
    ReadOnlyColumnarVec, ReadableColumnarVec,
};

use crate::{CachePolicy, IndexSources, Resolutions};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyColumnPerBlock<T, C, S: CachePolicy = Budgeted>
where
    T: PcoVecValue + Formattable + PartialOrd + Serialize + JsonSchema,
    C: ColumnId,
{
    pub height: CachedVec<LazyColumnVec<ReadOnlyColumnarVec<PcoVec<Height, T>, C>, C>, S>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<T, C, S: CachePolicy> LazyColumnPerBlock<T, C, S>
where
    T: PcoVecValue + Formattable + PartialOrd + Serialize + JsonSchema + 'static,
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        column: C,
        indexes: &IndexSources,
    ) -> Self {
        let height = S::wrap(source.column(name, version, column));
        let resolutions = Resolutions::from_source(name, &height, version, indexes);

        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }
}

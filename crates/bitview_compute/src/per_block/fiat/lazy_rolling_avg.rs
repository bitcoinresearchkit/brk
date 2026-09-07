use bitview_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::ReadableCloneableVec;

use crate::{
    AvgCentsToUsd, CachedWindowStartVec, FiatType, LazyPerBlock, LazyRollingAvgFromHeight,
};

#[derive(Clone, Traversable)]
pub struct LazyRollingAvgFiatFromHeight<C: FiatType> {
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, StoredF32>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: LazyRollingAvgFromHeight<C>,
}

impl<C: FiatType> LazyRollingAvgFiatFromHeight<C> {
    pub fn new(
        name: &str,
        version: Version,
        cumulative: &(impl ReadableCloneableVec<Height, C> + 'static),
        cached_start: &CachedWindowStartVec,
        indexes: &crate::IndexSources,
    ) -> Self {
        let cents = LazyRollingAvgFromHeight::new(
            &format!("{name}_cents"),
            version,
            cumulative.read_only_boxed_clone(),
            cached_start,
            indexes,
        );
        let usd = LazyPerBlock::from_resolutions::<AvgCentsToUsd>(
            name,
            version,
            cents.height.read_only_boxed_clone(),
            &cents.resolutions,
        );

        Self { usd, cents }
    }
}

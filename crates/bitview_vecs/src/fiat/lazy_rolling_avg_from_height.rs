use bitview_transforms::AvgCentsToUsd;
use bitview_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::ReadableCloneableVec;

use crate::{FiatType, IndexSources, LazyPerBlock, LazyRollingAvgFromHeight};

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
        cumulative: &impl ReadableCloneableVec<Height, C>,
        window_start: &impl ReadableCloneableVec<Height, Height>,
        indexes: &IndexSources,
    ) -> Self {
        let cents = LazyRollingAvgFromHeight::new(
            &format!("{name}_cents"),
            version,
            cumulative.read_only_boxed_clone(),
            window_start,
            indexes,
        );
        let usd =
            LazyPerBlock::from_resolutions::<AvgCentsToUsd>(name, version, &cents.resolutions);

        Self { usd, cents }
    }
}

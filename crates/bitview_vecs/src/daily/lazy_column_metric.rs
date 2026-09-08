use bitview_traversable::Traversable;
use brk_types::{Day1, Version};
use vecdb::{
    Budgeted, CacheBudget, CachedVec, CachedVecStrategy, ColumnId, LazyColumnVec, PcoVec,
    PcoVecValue, ReadOnlyColumnarVec, ReadableCloneableVec, ReadableColumnarVec,
};

use crate::{DailyMappings, DailyValue, DailyViews};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyColumnDailyMetric<T, C, S: CachedVecStrategy = Budgeted>
where
    T: DailyValue + PcoVecValue,
    C: ColumnId,
{
    pub day1: CachedVec<LazyColumnVec<ReadOnlyColumnarVec<PcoVec<Day1, T>, C>, C>, S>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T, C, S: CachedVecStrategy> LazyColumnDailyMetric<T, C, S>
where
    T: DailyValue + PcoVecValue,
    C: ColumnId,
{
    pub fn new(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Day1, T>, C>,
        column: C,
        mappings: &DailyMappings,
    ) -> Self {
        let day1 = S::wrap(source.column(name, version, column), cache);
        let views = Box::new(DailyViews::new(
            name,
            day1.read_only_boxed_clone(),
            version,
            mappings,
        ));

        Self { day1, views }
    }
}

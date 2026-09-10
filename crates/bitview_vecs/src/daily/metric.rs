use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Day1, Version};
use vecdb::{
    Budgeted, CacheBudget, CachedVec, CachedVecStrategy, Database, EagerVec, ImportableVec, PcoVec,
    PcoVecValue, Rw, StorageMode,
};

use crate::{DailyMappings, DailyValue, DailyViews};

#[derive(Traversable)]
#[traversable(merge)]
pub struct DailyMetric<T, M: StorageMode = Rw, S: CachedVecStrategy = Budgeted>
where
    T: DailyValue + PcoVecValue,
{
    pub day1: CachedVec<M::Stored<EagerVec<PcoVec<Day1, T>>>, S>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T, S: CachedVecStrategy> DailyMetric<T, Rw, S>
where
    T: DailyValue + PcoVecValue,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let day1 = S::wrap(
            EagerVec::<PcoVec<Day1, T>>::forced_import(db, name, version)?,
            cache,
        );
        let views = Box::new(DailyViews::new(name, &day1, version, mappings));

        Ok(Self { day1, views })
    }
}

use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Day1, Version};
use vecdb::{
    Budgeted, CacheBudget, CachePolicy, Database, EagerVec, ImportOptions, ImportableVec, PcoVec,
    PcoVecValue, Rw, StorageMode,
};

use crate::{DailyMappings, DailyValue, DailyViews};

#[derive(Traversable)]
#[traversable(merge)]
pub struct DailyMetric<T, M: StorageMode = Rw, S: CachePolicy = Budgeted>
where
    T: DailyValue + PcoVecValue,
{
    pub day1: M::Stored<EagerVec<PcoVec<Day1, T, S>>>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T, S: CachePolicy> DailyMetric<T, Rw, S>
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
        let day1 = EagerVec::<PcoVec<Day1, T, S>>::forced_import_with(
            ImportOptions::new(db, name, version).with_cache_budget(cache),
        )?;
        let views = Box::new(DailyViews::new(name, &day1, version, mappings));

        Ok(Self { day1, views })
    }
}

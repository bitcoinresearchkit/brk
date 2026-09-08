use brk_error::Result;

use bitview_traversable::Traversable;
use brk_types::{Day1, Version};
use vecdb::{
    Budgeted, CachedVec, Database, EagerVec, ImportableVec, PcoVec, PcoVecValue, Rw, StorageMode,
};

use super::{DailyMappings, DailyValue, DailyViews};
use crate::CachePolicy;

#[derive(Traversable)]
#[traversable(merge)]
pub struct DailyMetric<T, M: StorageMode = Rw, S: CachePolicy = Budgeted>
where
    T: DailyValue + PcoVecValue,
{
    pub day1: CachedVec<M::Stored<EagerVec<PcoVec<Day1, T>>>, S>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T, S: CachePolicy> DailyMetric<T, Rw, S>
where
    T: DailyValue + PcoVecValue,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let day1 = S::wrap(EagerVec::<PcoVec<Day1, T>>::forced_import(
            db, name, version,
        )?);
        let source = day1.read_only_boxed_clone();
        let views = Box::new(DailyViews::new(name, source, version, mappings));

        Ok(Self { day1, views })
    }
}

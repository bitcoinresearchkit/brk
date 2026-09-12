use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Day1, Version};
use vecdb::{Database, PcoVecValue, Rw, StorageMode};

use crate::{CachedSeries, DailyMappings, DailyValue, DailyViews, import_cached};

#[derive(Traversable)]
#[traversable(merge)]
pub struct DailyMetric<T, M: StorageMode = Rw>
where
    T: DailyValue + PcoVecValue,
{
    pub day1: CachedSeries<Day1, T, M>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T> DailyMetric<T>
where
    T: DailyValue + PcoVecValue,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let day1 = import_cached(db, name, version)?;
        let views = Box::new(DailyViews::new(name, &day1, version, mappings));

        Ok(Self { day1, views })
    }
}

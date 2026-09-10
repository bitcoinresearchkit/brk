use bitview_traversable::Traversable;
use brk_types::{Day1, Version};
use vecdb::{LazyVec, ReadableCloneableVec, UnaryTransform, VecValue};

use crate::{DailyMappings, DailyValue, DailyViews};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDailyMetric<T, S>
where
    T: DailyValue,
    S: VecValue,
{
    pub day1: LazyVec<Day1, T, Day1, S>,
    #[traversable(flatten)]
    pub views: Box<DailyViews<T>>,
}

impl<T, S> LazyDailyMetric<T, S>
where
    T: DailyValue,
    S: VecValue,
{
    pub fn from_source<F>(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Day1, S> + ?Sized),
        mappings: &DailyMappings,
    ) -> Self
    where
        F: UnaryTransform<S, T>,
    {
        let day1 = LazyVec::transformed::<F>(name, version, source.read_only_boxed_clone());
        let views = Box::new(DailyViews::new(name, &day1, version, mappings));

        Self { day1, views }
    }
}

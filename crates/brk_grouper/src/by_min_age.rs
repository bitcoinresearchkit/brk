use brk_traversable::Traversable;
use rayon::prelude::*;

use super::{Filter, TimeFilter, AGE_BOUNDARIES};

#[derive(Default, Clone, Traversable)]
pub struct ByMinAge<T> {
    pub _1d: T,
    pub _1w: T,
    pub _1m: T,
    pub _2m: T,
    pub _3m: T,
    pub _4m: T,
    pub _5m: T,
    pub _6m: T,
    pub _1y: T,
    pub _2y: T,
    pub _3y: T,
    pub _4y: T,
    pub _5y: T,
    pub _6y: T,
    pub _7y: T,
    pub _8y: T,
    pub _10y: T,
    pub _12y: T,
}

impl<T> ByMinAge<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            _1d: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[0]))),
            _1w: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[1]))),
            _1m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[2]))),
            _2m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[3]))),
            _3m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[4]))),
            _4m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[5]))),
            _5m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[6]))),
            _6m: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[7]))),
            _1y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[8]))),
            _2y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[9]))),
            _3y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[10]))),
            _4y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[11]))),
            _5y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[12]))),
            _6y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[13]))),
            _7y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[14]))),
            _8y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[15]))),
            _10y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[16]))),
            _12y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[17]))),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self._1d, &self._1w, &self._1m, &self._2m, &self._3m, &self._4m, &self._5m, &self._6m,
            &self._1y, &self._2y, &self._3y, &self._4y, &self._5y, &self._6y, &self._7y, &self._8y,
            &self._10y, &self._12y,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._1d,
            &mut self._1w,
            &mut self._1m,
            &mut self._2m,
            &mut self._3m,
            &mut self._4m,
            &mut self._5m,
            &mut self._6m,
            &mut self._1y,
            &mut self._2y,
            &mut self._3y,
            &mut self._4y,
            &mut self._5y,
            &mut self._6y,
            &mut self._7y,
            &mut self._8y,
            &mut self._10y,
            &mut self._12y,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self._1d,
            &mut self._1w,
            &mut self._1m,
            &mut self._2m,
            &mut self._3m,
            &mut self._4m,
            &mut self._5m,
            &mut self._6m,
            &mut self._1y,
            &mut self._2y,
            &mut self._3y,
            &mut self._4y,
            &mut self._5y,
            &mut self._6y,
            &mut self._7y,
            &mut self._8y,
            &mut self._10y,
            &mut self._12y,
        ]
        .into_par_iter()
    }
}

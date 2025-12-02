use brk_traversable::Traversable;

use super::{Filter, TimeFilter};

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
            _1d: create(Filter::Time(TimeFilter::GreaterOrEqual(1))),
            _1w: create(Filter::Time(TimeFilter::GreaterOrEqual(7))),
            _1m: create(Filter::Time(TimeFilter::GreaterOrEqual(30))),
            _2m: create(Filter::Time(TimeFilter::GreaterOrEqual(2 * 30))),
            _3m: create(Filter::Time(TimeFilter::GreaterOrEqual(3 * 30))),
            _4m: create(Filter::Time(TimeFilter::GreaterOrEqual(4 * 30))),
            _5m: create(Filter::Time(TimeFilter::GreaterOrEqual(5 * 30))),
            _6m: create(Filter::Time(TimeFilter::GreaterOrEqual(6 * 30))),
            _1y: create(Filter::Time(TimeFilter::GreaterOrEqual(365))),
            _2y: create(Filter::Time(TimeFilter::GreaterOrEqual(2 * 365))),
            _3y: create(Filter::Time(TimeFilter::GreaterOrEqual(3 * 365))),
            _4y: create(Filter::Time(TimeFilter::GreaterOrEqual(4 * 365))),
            _5y: create(Filter::Time(TimeFilter::GreaterOrEqual(5 * 365))),
            _6y: create(Filter::Time(TimeFilter::GreaterOrEqual(6 * 365))),
            _7y: create(Filter::Time(TimeFilter::GreaterOrEqual(7 * 365))),
            _8y: create(Filter::Time(TimeFilter::GreaterOrEqual(8 * 365))),
            _10y: create(Filter::Time(TimeFilter::GreaterOrEqual(10 * 365))),
            _12y: create(Filter::Time(TimeFilter::GreaterOrEqual(12 * 365))),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self._1d,
            &self._1w,
            &self._1m,
            &self._2m,
            &self._3m,
            &self._4m,
            &self._5m,
            &self._6m,
            &self._1y,
            &self._2y,
            &self._3y,
            &self._4y,
            &self._5y,
            &self._6y,
            &self._7y,
            &self._8y,
            &self._10y,
            &self._12y,
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
}

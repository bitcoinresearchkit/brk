use brk_traversable::Traversable;
use rayon::prelude::*;
use serde::Serialize;

use super::{
    CohortName, Filter, TimeFilter, HOURS_10Y, HOURS_12Y, HOURS_1D, HOURS_1M, HOURS_1W, HOURS_1Y,
    HOURS_2M, HOURS_2Y, HOURS_3M, HOURS_3Y, HOURS_4M, HOURS_4Y, HOURS_5M, HOURS_5Y, HOURS_6M,
    HOURS_6Y, HOURS_7Y, HOURS_8Y,
};

/// Min age thresholds in hours
pub const MIN_AGE_HOURS: ByMinAge<usize> = ByMinAge {
    _1d: HOURS_1D,
    _1w: HOURS_1W,
    _1m: HOURS_1M,
    _2m: HOURS_2M,
    _3m: HOURS_3M,
    _4m: HOURS_4M,
    _5m: HOURS_5M,
    _6m: HOURS_6M,
    _1y: HOURS_1Y,
    _2y: HOURS_2Y,
    _3y: HOURS_3Y,
    _4y: HOURS_4Y,
    _5y: HOURS_5Y,
    _6y: HOURS_6Y,
    _7y: HOURS_7Y,
    _8y: HOURS_8Y,
    _10y: HOURS_10Y,
    _12y: HOURS_12Y,
};

/// Min age filters (GreaterOrEqual threshold in hours)
pub const MIN_AGE_FILTERS: ByMinAge<Filter> = ByMinAge {
    _1d: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._1d)),
    _1w: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._1w)),
    _1m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._1m)),
    _2m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._2m)),
    _3m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._3m)),
    _4m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._4m)),
    _5m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._5m)),
    _6m: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._6m)),
    _1y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._1y)),
    _2y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._2y)),
    _3y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._3y)),
    _4y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._4y)),
    _5y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._5y)),
    _6y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._6y)),
    _7y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._7y)),
    _8y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._8y)),
    _10y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._10y)),
    _12y: Filter::Time(TimeFilter::GreaterOrEqual(MIN_AGE_HOURS._12y)),
};

/// Min age names
pub const MIN_AGE_NAMES: ByMinAge<CohortName> = ByMinAge {
    _1d: CohortName::new("at_least_1d_old", "1d+", "At Least 1 Day Old"),
    _1w: CohortName::new("at_least_1w_old", "1w+", "At Least 1 Week Old"),
    _1m: CohortName::new("at_least_1m_old", "1m+", "At Least 1 Month Old"),
    _2m: CohortName::new("at_least_2m_old", "2m+", "At Least 2 Months Old"),
    _3m: CohortName::new("at_least_3m_old", "3m+", "At Least 3 Months Old"),
    _4m: CohortName::new("at_least_4m_old", "4m+", "At Least 4 Months Old"),
    _5m: CohortName::new("at_least_5m_old", "5m+", "At Least 5 Months Old"),
    _6m: CohortName::new("at_least_6m_old", "6m+", "At Least 6 Months Old"),
    _1y: CohortName::new("at_least_1y_old", "1y+", "At Least 1 Year Old"),
    _2y: CohortName::new("at_least_2y_old", "2y+", "At Least 2 Years Old"),
    _3y: CohortName::new("at_least_3y_old", "3y+", "At Least 3 Years Old"),
    _4y: CohortName::new("at_least_4y_old", "4y+", "At Least 4 Years Old"),
    _5y: CohortName::new("at_least_5y_old", "5y+", "At Least 5 Years Old"),
    _6y: CohortName::new("at_least_6y_old", "6y+", "At Least 6 Years Old"),
    _7y: CohortName::new("at_least_7y_old", "7y+", "At Least 7 Years Old"),
    _8y: CohortName::new("at_least_8y_old", "8y+", "At Least 8 Years Old"),
    _10y: CohortName::new("at_least_10y_old", "10y+", "At Least 10 Years Old"),
    _12y: CohortName::new("at_least_12y_old", "12y+", "At Least 12 Years Old"),
};

#[derive(Default, Clone, Traversable, Serialize)]
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

impl ByMinAge<CohortName> {
    pub const fn names() -> &'static Self {
        &MIN_AGE_NAMES
    }
}

impl<T> ByMinAge<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = MIN_AGE_FILTERS;
        let n = MIN_AGE_NAMES;
        Self {
            _1d: create(f._1d.clone(), n._1d.id),
            _1w: create(f._1w.clone(), n._1w.id),
            _1m: create(f._1m.clone(), n._1m.id),
            _2m: create(f._2m.clone(), n._2m.id),
            _3m: create(f._3m.clone(), n._3m.id),
            _4m: create(f._4m.clone(), n._4m.id),
            _5m: create(f._5m.clone(), n._5m.id),
            _6m: create(f._6m.clone(), n._6m.id),
            _1y: create(f._1y.clone(), n._1y.id),
            _2y: create(f._2y.clone(), n._2y.id),
            _3y: create(f._3y.clone(), n._3y.id),
            _4y: create(f._4y.clone(), n._4y.id),
            _5y: create(f._5y.clone(), n._5y.id),
            _6y: create(f._6y.clone(), n._6y.id),
            _7y: create(f._7y.clone(), n._7y.id),
            _8y: create(f._8y.clone(), n._8y.id),
            _10y: create(f._10y.clone(), n._10y.id),
            _12y: create(f._12y.clone(), n._12y.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        let f = MIN_AGE_FILTERS;
        let n = MIN_AGE_NAMES;
        Ok(Self {
            _1d: create(f._1d.clone(), n._1d.id)?,
            _1w: create(f._1w.clone(), n._1w.id)?,
            _1m: create(f._1m.clone(), n._1m.id)?,
            _2m: create(f._2m.clone(), n._2m.id)?,
            _3m: create(f._3m.clone(), n._3m.id)?,
            _4m: create(f._4m.clone(), n._4m.id)?,
            _5m: create(f._5m.clone(), n._5m.id)?,
            _6m: create(f._6m.clone(), n._6m.id)?,
            _1y: create(f._1y.clone(), n._1y.id)?,
            _2y: create(f._2y.clone(), n._2y.id)?,
            _3y: create(f._3y.clone(), n._3y.id)?,
            _4y: create(f._4y.clone(), n._4y.id)?,
            _5y: create(f._5y.clone(), n._5y.id)?,
            _6y: create(f._6y.clone(), n._6y.id)?,
            _7y: create(f._7y.clone(), n._7y.id)?,
            _8y: create(f._8y.clone(), n._8y.id)?,
            _10y: create(f._10y.clone(), n._10y.id)?,
            _12y: create(f._12y.clone(), n._12y.id)?,
        })
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

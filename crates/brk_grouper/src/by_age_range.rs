use std::ops::Range;

use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Serialize;

use super::{CohortName, Filter, TimeFilter};

// Age boundary constants in days
pub const DAYS_1D: usize = 1;
pub const DAYS_1W: usize = 7;
pub const DAYS_1M: usize = 30;
pub const DAYS_2M: usize = 2 * 30;
pub const DAYS_3M: usize = 3 * 30;
pub const DAYS_4M: usize = 4 * 30;
pub const DAYS_5M: usize = 5 * 30;
pub const DAYS_6M: usize = 6 * 30;
pub const DAYS_1Y: usize = 365;
pub const DAYS_2Y: usize = 2 * 365;
pub const DAYS_3Y: usize = 3 * 365;
pub const DAYS_4Y: usize = 4 * 365;
pub const DAYS_5Y: usize = 5 * 365;
pub const DAYS_6Y: usize = 6 * 365;
pub const DAYS_7Y: usize = 7 * 365;
pub const DAYS_8Y: usize = 8 * 365;
pub const DAYS_10Y: usize = 10 * 365;
pub const DAYS_12Y: usize = 12 * 365;
pub const DAYS_15Y: usize = 15 * 365;

/// Age boundaries in days. Defines the cohort ranges:
/// [0, B[0]), [B[0], B[1]), [B[1], B[2]), ..., [B[n-1], ∞)
pub const AGE_BOUNDARIES: [usize; 19] = [
    DAYS_1D, DAYS_1W, DAYS_1M, DAYS_2M, DAYS_3M, DAYS_4M, DAYS_5M, DAYS_6M, DAYS_1Y, DAYS_2Y,
    DAYS_3Y, DAYS_4Y, DAYS_5Y, DAYS_6Y, DAYS_7Y, DAYS_8Y, DAYS_10Y, DAYS_12Y, DAYS_15Y,
];

/// Age range bounds (end = usize::MAX means unbounded)
pub const AGE_RANGE_BOUNDS: ByAgeRange<Range<usize>> = ByAgeRange {
    up_to_1d: 0..DAYS_1D,
    _1d_to_1w: DAYS_1D..DAYS_1W,
    _1w_to_1m: DAYS_1W..DAYS_1M,
    _1m_to_2m: DAYS_1M..DAYS_2M,
    _2m_to_3m: DAYS_2M..DAYS_3M,
    _3m_to_4m: DAYS_3M..DAYS_4M,
    _4m_to_5m: DAYS_4M..DAYS_5M,
    _5m_to_6m: DAYS_5M..DAYS_6M,
    _6m_to_1y: DAYS_6M..DAYS_1Y,
    _1y_to_2y: DAYS_1Y..DAYS_2Y,
    _2y_to_3y: DAYS_2Y..DAYS_3Y,
    _3y_to_4y: DAYS_3Y..DAYS_4Y,
    _4y_to_5y: DAYS_4Y..DAYS_5Y,
    _5y_to_6y: DAYS_5Y..DAYS_6Y,
    _6y_to_7y: DAYS_6Y..DAYS_7Y,
    _7y_to_8y: DAYS_7Y..DAYS_8Y,
    _8y_to_10y: DAYS_8Y..DAYS_10Y,
    _10y_to_12y: DAYS_10Y..DAYS_12Y,
    _12y_to_15y: DAYS_12Y..DAYS_15Y,
    from_15y: DAYS_15Y..usize::MAX,
};

/// Age range filters
pub const AGE_RANGE_FILTERS: ByAgeRange<Filter> = ByAgeRange {
    up_to_1d: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS.up_to_1d)),
    _1d_to_1w: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._1d_to_1w)),
    _1w_to_1m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._1w_to_1m)),
    _1m_to_2m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._1m_to_2m)),
    _2m_to_3m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._2m_to_3m)),
    _3m_to_4m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._3m_to_4m)),
    _4m_to_5m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._4m_to_5m)),
    _5m_to_6m: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._5m_to_6m)),
    _6m_to_1y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._6m_to_1y)),
    _1y_to_2y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._1y_to_2y)),
    _2y_to_3y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._2y_to_3y)),
    _3y_to_4y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._3y_to_4y)),
    _4y_to_5y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._4y_to_5y)),
    _5y_to_6y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._5y_to_6y)),
    _6y_to_7y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._6y_to_7y)),
    _7y_to_8y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._7y_to_8y)),
    _8y_to_10y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._8y_to_10y)),
    _10y_to_12y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._10y_to_12y)),
    _12y_to_15y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._12y_to_15y)),
    from_15y: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS.from_15y)),
};

/// Age range names
pub const AGE_RANGE_NAMES: ByAgeRange<CohortName> = ByAgeRange {
    up_to_1d: CohortName::new("up_to_1d_old", "<1d", "Up to 1 Day Old"),
    _1d_to_1w: CohortName::new("at_least_1d_up_to_1w_old", "1d-1w", "1 Day to 1 Week Old"),
    _1w_to_1m: CohortName::new("at_least_1w_up_to_1m_old", "1w-1m", "1 Week to 1 Month Old"),
    _1m_to_2m: CohortName::new("at_least_1m_up_to_2m_old", "1m-2m", "1 to 2 Months Old"),
    _2m_to_3m: CohortName::new("at_least_2m_up_to_3m_old", "2m-3m", "2 to 3 Months Old"),
    _3m_to_4m: CohortName::new("at_least_3m_up_to_4m_old", "3m-4m", "3 to 4 Months Old"),
    _4m_to_5m: CohortName::new("at_least_4m_up_to_5m_old", "4m-5m", "4 to 5 Months Old"),
    _5m_to_6m: CohortName::new("at_least_5m_up_to_6m_old", "5m-6m", "5 to 6 Months Old"),
    _6m_to_1y: CohortName::new("at_least_6m_up_to_1y_old", "6m-1y", "6 Months to 1 Year Old"),
    _1y_to_2y: CohortName::new("at_least_1y_up_to_2y_old", "1y-2y", "1 to 2 Years Old"),
    _2y_to_3y: CohortName::new("at_least_2y_up_to_3y_old", "2y-3y", "2 to 3 Years Old"),
    _3y_to_4y: CohortName::new("at_least_3y_up_to_4y_old", "3y-4y", "3 to 4 Years Old"),
    _4y_to_5y: CohortName::new("at_least_4y_up_to_5y_old", "4y-5y", "4 to 5 Years Old"),
    _5y_to_6y: CohortName::new("at_least_5y_up_to_6y_old", "5y-6y", "5 to 6 Years Old"),
    _6y_to_7y: CohortName::new("at_least_6y_up_to_7y_old", "6y-7y", "6 to 7 Years Old"),
    _7y_to_8y: CohortName::new("at_least_7y_up_to_8y_old", "7y-8y", "7 to 8 Years Old"),
    _8y_to_10y: CohortName::new("at_least_8y_up_to_10y_old", "8y-10y", "8 to 10 Years Old"),
    _10y_to_12y: CohortName::new("at_least_10y_up_to_12y_old", "10y-12y", "10 to 12 Years Old"),
    _12y_to_15y: CohortName::new("at_least_12y_up_to_15y_old", "12y-15y", "12 to 15 Years Old"),
    from_15y: CohortName::new("at_least_15y_old", "15y+", "15+ Years Old"),
};

impl ByAgeRange<CohortName> {
    pub const fn names() -> &'static Self {
        &AGE_RANGE_NAMES
    }
}

#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByAgeRange<T> {
    pub up_to_1d: T,
    pub _1d_to_1w: T,
    pub _1w_to_1m: T,
    pub _1m_to_2m: T,
    pub _2m_to_3m: T,
    pub _3m_to_4m: T,
    pub _4m_to_5m: T,
    pub _5m_to_6m: T,
    pub _6m_to_1y: T,
    pub _1y_to_2y: T,
    pub _2y_to_3y: T,
    pub _3y_to_4y: T,
    pub _4y_to_5y: T,
    pub _5y_to_6y: T,
    pub _6y_to_7y: T,
    pub _7y_to_8y: T,
    pub _8y_to_10y: T,
    pub _10y_to_12y: T,
    pub _12y_to_15y: T,
    pub from_15y: T,
}

impl<T> ByAgeRange<T> {
    /// Get mutable reference by days old. O(1).
    #[inline]
    pub fn get_mut_by_days_old(&mut self, days_old: usize) -> &mut T {
        match days_old {
            0..DAYS_1D => &mut self.up_to_1d,
            DAYS_1D..DAYS_1W => &mut self._1d_to_1w,
            DAYS_1W..DAYS_1M => &mut self._1w_to_1m,
            DAYS_1M..DAYS_2M => &mut self._1m_to_2m,
            DAYS_2M..DAYS_3M => &mut self._2m_to_3m,
            DAYS_3M..DAYS_4M => &mut self._3m_to_4m,
            DAYS_4M..DAYS_5M => &mut self._4m_to_5m,
            DAYS_5M..DAYS_6M => &mut self._5m_to_6m,
            DAYS_6M..DAYS_1Y => &mut self._6m_to_1y,
            DAYS_1Y..DAYS_2Y => &mut self._1y_to_2y,
            DAYS_2Y..DAYS_3Y => &mut self._2y_to_3y,
            DAYS_3Y..DAYS_4Y => &mut self._3y_to_4y,
            DAYS_4Y..DAYS_5Y => &mut self._4y_to_5y,
            DAYS_5Y..DAYS_6Y => &mut self._5y_to_6y,
            DAYS_6Y..DAYS_7Y => &mut self._6y_to_7y,
            DAYS_7Y..DAYS_8Y => &mut self._7y_to_8y,
            DAYS_8Y..DAYS_10Y => &mut self._8y_to_10y,
            DAYS_10Y..DAYS_12Y => &mut self._10y_to_12y,
            DAYS_12Y..DAYS_15Y => &mut self._12y_to_15y,
            _ => &mut self.from_15y,
        }
    }

    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = AGE_RANGE_FILTERS;
        let n = AGE_RANGE_NAMES;
        Self {
            up_to_1d: create(f.up_to_1d.clone(), n.up_to_1d.id),
            _1d_to_1w: create(f._1d_to_1w.clone(), n._1d_to_1w.id),
            _1w_to_1m: create(f._1w_to_1m.clone(), n._1w_to_1m.id),
            _1m_to_2m: create(f._1m_to_2m.clone(), n._1m_to_2m.id),
            _2m_to_3m: create(f._2m_to_3m.clone(), n._2m_to_3m.id),
            _3m_to_4m: create(f._3m_to_4m.clone(), n._3m_to_4m.id),
            _4m_to_5m: create(f._4m_to_5m.clone(), n._4m_to_5m.id),
            _5m_to_6m: create(f._5m_to_6m.clone(), n._5m_to_6m.id),
            _6m_to_1y: create(f._6m_to_1y.clone(), n._6m_to_1y.id),
            _1y_to_2y: create(f._1y_to_2y.clone(), n._1y_to_2y.id),
            _2y_to_3y: create(f._2y_to_3y.clone(), n._2y_to_3y.id),
            _3y_to_4y: create(f._3y_to_4y.clone(), n._3y_to_4y.id),
            _4y_to_5y: create(f._4y_to_5y.clone(), n._4y_to_5y.id),
            _5y_to_6y: create(f._5y_to_6y.clone(), n._5y_to_6y.id),
            _6y_to_7y: create(f._6y_to_7y.clone(), n._6y_to_7y.id),
            _7y_to_8y: create(f._7y_to_8y.clone(), n._7y_to_8y.id),
            _8y_to_10y: create(f._8y_to_10y.clone(), n._8y_to_10y.id),
            _10y_to_12y: create(f._10y_to_12y.clone(), n._10y_to_12y.id),
            _12y_to_15y: create(f._12y_to_15y.clone(), n._12y_to_15y.id),
            from_15y: create(f.from_15y.clone(), n.from_15y.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        let f = AGE_RANGE_FILTERS;
        let n = AGE_RANGE_NAMES;
        Ok(Self {
            up_to_1d: create(f.up_to_1d.clone(), n.up_to_1d.id)?,
            _1d_to_1w: create(f._1d_to_1w.clone(), n._1d_to_1w.id)?,
            _1w_to_1m: create(f._1w_to_1m.clone(), n._1w_to_1m.id)?,
            _1m_to_2m: create(f._1m_to_2m.clone(), n._1m_to_2m.id)?,
            _2m_to_3m: create(f._2m_to_3m.clone(), n._2m_to_3m.id)?,
            _3m_to_4m: create(f._3m_to_4m.clone(), n._3m_to_4m.id)?,
            _4m_to_5m: create(f._4m_to_5m.clone(), n._4m_to_5m.id)?,
            _5m_to_6m: create(f._5m_to_6m.clone(), n._5m_to_6m.id)?,
            _6m_to_1y: create(f._6m_to_1y.clone(), n._6m_to_1y.id)?,
            _1y_to_2y: create(f._1y_to_2y.clone(), n._1y_to_2y.id)?,
            _2y_to_3y: create(f._2y_to_3y.clone(), n._2y_to_3y.id)?,
            _3y_to_4y: create(f._3y_to_4y.clone(), n._3y_to_4y.id)?,
            _4y_to_5y: create(f._4y_to_5y.clone(), n._4y_to_5y.id)?,
            _5y_to_6y: create(f._5y_to_6y.clone(), n._5y_to_6y.id)?,
            _6y_to_7y: create(f._6y_to_7y.clone(), n._6y_to_7y.id)?,
            _7y_to_8y: create(f._7y_to_8y.clone(), n._7y_to_8y.id)?,
            _8y_to_10y: create(f._8y_to_10y.clone(), n._8y_to_10y.id)?,
            _10y_to_12y: create(f._10y_to_12y.clone(), n._10y_to_12y.id)?,
            _12y_to_15y: create(f._12y_to_15y.clone(), n._12y_to_15y.id)?,
            from_15y: create(f.from_15y.clone(), n.from_15y.id)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self.up_to_1d,
            &self._1d_to_1w,
            &self._1w_to_1m,
            &self._1m_to_2m,
            &self._2m_to_3m,
            &self._3m_to_4m,
            &self._4m_to_5m,
            &self._5m_to_6m,
            &self._6m_to_1y,
            &self._1y_to_2y,
            &self._2y_to_3y,
            &self._3y_to_4y,
            &self._4y_to_5y,
            &self._5y_to_6y,
            &self._6y_to_7y,
            &self._7y_to_8y,
            &self._8y_to_10y,
            &self._10y_to_12y,
            &self._12y_to_15y,
            &self.from_15y,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.up_to_1d,
            &mut self._1d_to_1w,
            &mut self._1w_to_1m,
            &mut self._1m_to_2m,
            &mut self._2m_to_3m,
            &mut self._3m_to_4m,
            &mut self._4m_to_5m,
            &mut self._5m_to_6m,
            &mut self._6m_to_1y,
            &mut self._1y_to_2y,
            &mut self._2y_to_3y,
            &mut self._3y_to_4y,
            &mut self._4y_to_5y,
            &mut self._5y_to_6y,
            &mut self._6y_to_7y,
            &mut self._7y_to_8y,
            &mut self._8y_to_10y,
            &mut self._10y_to_12y,
            &mut self._12y_to_15y,
            &mut self.from_15y,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self.up_to_1d,
            &mut self._1d_to_1w,
            &mut self._1w_to_1m,
            &mut self._1m_to_2m,
            &mut self._2m_to_3m,
            &mut self._3m_to_4m,
            &mut self._4m_to_5m,
            &mut self._5m_to_6m,
            &mut self._6m_to_1y,
            &mut self._1y_to_2y,
            &mut self._2y_to_3y,
            &mut self._3y_to_4y,
            &mut self._4y_to_5y,
            &mut self._5y_to_6y,
            &mut self._6y_to_7y,
            &mut self._7y_to_8y,
            &mut self._8y_to_10y,
            &mut self._10y_to_12y,
            &mut self._12y_to_15y,
            &mut self.from_15y,
        ]
        .into_par_iter()
    }
}


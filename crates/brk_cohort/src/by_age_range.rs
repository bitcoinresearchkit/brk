use std::ops::Range;

use brk_types::Age;
use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Serialize;

use super::{CohortName, Filter, TimeFilter};

// Age boundary constants in hours
pub const HOURS_1H: usize = 1;
pub const HOURS_1D: usize = 24;
pub const HOURS_1W: usize = 24 * 7;
pub const HOURS_1M: usize = 24 * 30;
pub const HOURS_2M: usize = 24 * 2 * 30;
pub const HOURS_3M: usize = 24 * 3 * 30;
pub const HOURS_4M: usize = 24 * 4 * 30;
pub const HOURS_5M: usize = 24 * 5 * 30; // STH/LTH threshold
pub const HOURS_6M: usize = 24 * 6 * 30;
pub const HOURS_1Y: usize = 24 * 365;
pub const HOURS_2Y: usize = 24 * 2 * 365;
pub const HOURS_3Y: usize = 24 * 3 * 365;
pub const HOURS_4Y: usize = 24 * 4 * 365;
pub const HOURS_5Y: usize = 24 * 5 * 365;
pub const HOURS_6Y: usize = 24 * 6 * 365;
pub const HOURS_7Y: usize = 24 * 7 * 365;
pub const HOURS_8Y: usize = 24 * 8 * 365;
pub const HOURS_10Y: usize = 24 * 10 * 365;
pub const HOURS_12Y: usize = 24 * 12 * 365;
pub const HOURS_15Y: usize = 24 * 15 * 365;

/// Age boundaries in hours. Defines the cohort ranges:
/// [0, 1h), [1h, 1d), [1d, 1w), [1w, 1m), ..., [15y, ∞)
pub const AGE_BOUNDARIES: [usize; 20] = [
    HOURS_1H, HOURS_1D, HOURS_1W, HOURS_1M, HOURS_2M, HOURS_3M, HOURS_4M,
    HOURS_5M, HOURS_6M, HOURS_1Y, HOURS_2Y, HOURS_3Y, HOURS_4Y, HOURS_5Y,
    HOURS_6Y, HOURS_7Y, HOURS_8Y, HOURS_10Y, HOURS_12Y, HOURS_15Y,
];

/// Age range bounds (end = usize::MAX means unbounded)
pub const AGE_RANGE_BOUNDS: ByAgeRange<Range<usize>> = ByAgeRange {
    up_to_1h: 0..HOURS_1H,
    _1h_to_1d: HOURS_1H..HOURS_1D,
    _1d_to_1w: HOURS_1D..HOURS_1W,
    _1w_to_1m: HOURS_1W..HOURS_1M,
    _1m_to_2m: HOURS_1M..HOURS_2M,
    _2m_to_3m: HOURS_2M..HOURS_3M,
    _3m_to_4m: HOURS_3M..HOURS_4M,
    _4m_to_5m: HOURS_4M..HOURS_5M,
    _5m_to_6m: HOURS_5M..HOURS_6M,
    _6m_to_1y: HOURS_6M..HOURS_1Y,
    _1y_to_2y: HOURS_1Y..HOURS_2Y,
    _2y_to_3y: HOURS_2Y..HOURS_3Y,
    _3y_to_4y: HOURS_3Y..HOURS_4Y,
    _4y_to_5y: HOURS_4Y..HOURS_5Y,
    _5y_to_6y: HOURS_5Y..HOURS_6Y,
    _6y_to_7y: HOURS_6Y..HOURS_7Y,
    _7y_to_8y: HOURS_7Y..HOURS_8Y,
    _8y_to_10y: HOURS_8Y..HOURS_10Y,
    _10y_to_12y: HOURS_10Y..HOURS_12Y,
    _12y_to_15y: HOURS_12Y..HOURS_15Y,
    from_15y: HOURS_15Y..usize::MAX,
};

/// Age range filters
pub const AGE_RANGE_FILTERS: ByAgeRange<Filter> = ByAgeRange {
    up_to_1h: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS.up_to_1h)),
    _1h_to_1d: Filter::Time(TimeFilter::Range(AGE_RANGE_BOUNDS._1h_to_1d)),
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
    up_to_1h: CohortName::new("under_1h_old", "<1h", "Under 1 Hour Old"),
    _1h_to_1d: CohortName::new("1h_to_1d_old", "1h-1d", "1 Hour to 1 Day Old"),
    _1d_to_1w: CohortName::new("1d_to_1w_old", "1d-1w", "1 Day to 1 Week Old"),
    _1w_to_1m: CohortName::new("1w_to_1m_old", "1w-1m", "1 Week to 1 Month Old"),
    _1m_to_2m: CohortName::new("1m_to_2m_old", "1m-2m", "1 to 2 Months Old"),
    _2m_to_3m: CohortName::new("2m_to_3m_old", "2m-3m", "2 to 3 Months Old"),
    _3m_to_4m: CohortName::new("3m_to_4m_old", "3m-4m", "3 to 4 Months Old"),
    _4m_to_5m: CohortName::new("4m_to_5m_old", "4m-5m", "4 to 5 Months Old"),
    _5m_to_6m: CohortName::new("5m_to_6m_old", "5m-6m", "5 to 6 Months Old"),
    _6m_to_1y: CohortName::new("6m_to_1y_old", "6m-1y", "6 Months to 1 Year Old"),
    _1y_to_2y: CohortName::new("1y_to_2y_old", "1y-2y", "1 to 2 Years Old"),
    _2y_to_3y: CohortName::new("2y_to_3y_old", "2y-3y", "2 to 3 Years Old"),
    _3y_to_4y: CohortName::new("3y_to_4y_old", "3y-4y", "3 to 4 Years Old"),
    _4y_to_5y: CohortName::new("4y_to_5y_old", "4y-5y", "4 to 5 Years Old"),
    _5y_to_6y: CohortName::new("5y_to_6y_old", "5y-6y", "5 to 6 Years Old"),
    _6y_to_7y: CohortName::new("6y_to_7y_old", "6y-7y", "6 to 7 Years Old"),
    _7y_to_8y: CohortName::new("7y_to_8y_old", "7y-8y", "7 to 8 Years Old"),
    _8y_to_10y: CohortName::new("8y_to_10y_old", "8y-10y", "8 to 10 Years Old"),
    _10y_to_12y: CohortName::new("10y_to_12y_old", "10y-12y", "10 to 12 Years Old"),
    _12y_to_15y: CohortName::new("12y_to_15y_old", "12y-15y", "12 to 15 Years Old"),
    from_15y: CohortName::new("over_15y_old", "15y+", "15+ Years Old"),
};

impl ByAgeRange<CohortName> {
    pub const fn names() -> &'static Self {
        &AGE_RANGE_NAMES
    }
}

#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByAgeRange<T> {
    pub up_to_1h: T,
    pub _1h_to_1d: T,
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
    /// Get mutable reference by Age. O(1).
    #[inline]
    pub fn get_mut(&mut self, age: Age) -> &mut T {
        match age.hours() {
            0..HOURS_1H => &mut self.up_to_1h,
            HOURS_1H..HOURS_1D => &mut self._1h_to_1d,
            HOURS_1D..HOURS_1W => &mut self._1d_to_1w,
            HOURS_1W..HOURS_1M => &mut self._1w_to_1m,
            HOURS_1M..HOURS_2M => &mut self._1m_to_2m,
            HOURS_2M..HOURS_3M => &mut self._2m_to_3m,
            HOURS_3M..HOURS_4M => &mut self._3m_to_4m,
            HOURS_4M..HOURS_5M => &mut self._4m_to_5m,
            HOURS_5M..HOURS_6M => &mut self._5m_to_6m,
            HOURS_6M..HOURS_1Y => &mut self._6m_to_1y,
            HOURS_1Y..HOURS_2Y => &mut self._1y_to_2y,
            HOURS_2Y..HOURS_3Y => &mut self._2y_to_3y,
            HOURS_3Y..HOURS_4Y => &mut self._3y_to_4y,
            HOURS_4Y..HOURS_5Y => &mut self._4y_to_5y,
            HOURS_5Y..HOURS_6Y => &mut self._5y_to_6y,
            HOURS_6Y..HOURS_7Y => &mut self._6y_to_7y,
            HOURS_7Y..HOURS_8Y => &mut self._7y_to_8y,
            HOURS_8Y..HOURS_10Y => &mut self._8y_to_10y,
            HOURS_10Y..HOURS_12Y => &mut self._10y_to_12y,
            HOURS_12Y..HOURS_15Y => &mut self._12y_to_15y,
            _ => &mut self.from_15y,
        }
    }

    /// Get reference by Age. O(1).
    #[inline]
    pub fn get(&self, age: Age) -> &T {
        match age.hours() {
            0..HOURS_1H => &self.up_to_1h,
            HOURS_1H..HOURS_1D => &self._1h_to_1d,
            HOURS_1D..HOURS_1W => &self._1d_to_1w,
            HOURS_1W..HOURS_1M => &self._1w_to_1m,
            HOURS_1M..HOURS_2M => &self._1m_to_2m,
            HOURS_2M..HOURS_3M => &self._2m_to_3m,
            HOURS_3M..HOURS_4M => &self._3m_to_4m,
            HOURS_4M..HOURS_5M => &self._4m_to_5m,
            HOURS_5M..HOURS_6M => &self._5m_to_6m,
            HOURS_6M..HOURS_1Y => &self._6m_to_1y,
            HOURS_1Y..HOURS_2Y => &self._1y_to_2y,
            HOURS_2Y..HOURS_3Y => &self._2y_to_3y,
            HOURS_3Y..HOURS_4Y => &self._3y_to_4y,
            HOURS_4Y..HOURS_5Y => &self._4y_to_5y,
            HOURS_5Y..HOURS_6Y => &self._5y_to_6y,
            HOURS_6Y..HOURS_7Y => &self._6y_to_7y,
            HOURS_7Y..HOURS_8Y => &self._7y_to_8y,
            HOURS_8Y..HOURS_10Y => &self._8y_to_10y,
            HOURS_10Y..HOURS_12Y => &self._10y_to_12y,
            HOURS_12Y..HOURS_15Y => &self._12y_to_15y,
            _ => &self.from_15y,
        }
    }

    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = AGE_RANGE_FILTERS;
        let n = AGE_RANGE_NAMES;
        Self {
            up_to_1h: create(f.up_to_1h.clone(), n.up_to_1h.id),
            _1h_to_1d: create(f._1h_to_1d.clone(), n._1h_to_1d.id),
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
            up_to_1h: create(f.up_to_1h.clone(), n.up_to_1h.id)?,
            _1h_to_1d: create(f._1h_to_1d.clone(), n._1h_to_1d.id)?,
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
            &self.up_to_1h,
            &self._1h_to_1d,
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
            &mut self.up_to_1h,
            &mut self._1h_to_1d,
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
            &mut self.up_to_1h,
            &mut self._1h_to_1d,
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

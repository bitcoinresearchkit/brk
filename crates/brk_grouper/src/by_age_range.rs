use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{Filter, TimeFilter};

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

#[derive(Default, Clone, Traversable)]
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
        F: FnMut(Filter) -> T,
    {
        Self {
            up_to_1d: create(Filter::Time(TimeFilter::Range(0..DAYS_1D))),
            _1d_to_1w: create(Filter::Time(TimeFilter::Range(DAYS_1D..DAYS_1W))),
            _1w_to_1m: create(Filter::Time(TimeFilter::Range(DAYS_1W..DAYS_1M))),
            _1m_to_2m: create(Filter::Time(TimeFilter::Range(DAYS_1M..DAYS_2M))),
            _2m_to_3m: create(Filter::Time(TimeFilter::Range(DAYS_2M..DAYS_3M))),
            _3m_to_4m: create(Filter::Time(TimeFilter::Range(DAYS_3M..DAYS_4M))),
            _4m_to_5m: create(Filter::Time(TimeFilter::Range(DAYS_4M..DAYS_5M))),
            _5m_to_6m: create(Filter::Time(TimeFilter::Range(DAYS_5M..DAYS_6M))),
            _6m_to_1y: create(Filter::Time(TimeFilter::Range(DAYS_6M..DAYS_1Y))),
            _1y_to_2y: create(Filter::Time(TimeFilter::Range(DAYS_1Y..DAYS_2Y))),
            _2y_to_3y: create(Filter::Time(TimeFilter::Range(DAYS_2Y..DAYS_3Y))),
            _3y_to_4y: create(Filter::Time(TimeFilter::Range(DAYS_3Y..DAYS_4Y))),
            _4y_to_5y: create(Filter::Time(TimeFilter::Range(DAYS_4Y..DAYS_5Y))),
            _5y_to_6y: create(Filter::Time(TimeFilter::Range(DAYS_5Y..DAYS_6Y))),
            _6y_to_7y: create(Filter::Time(TimeFilter::Range(DAYS_6Y..DAYS_7Y))),
            _7y_to_8y: create(Filter::Time(TimeFilter::Range(DAYS_7Y..DAYS_8Y))),
            _8y_to_10y: create(Filter::Time(TimeFilter::Range(DAYS_8Y..DAYS_10Y))),
            _10y_to_12y: create(Filter::Time(TimeFilter::Range(DAYS_10Y..DAYS_12Y))),
            _12y_to_15y: create(Filter::Time(TimeFilter::Range(DAYS_12Y..DAYS_15Y))),
            from_15y: create(Filter::Time(TimeFilter::GreaterOrEqual(DAYS_15Y))),
        }
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


use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{Filter, TimeFilter};

/// Age boundaries in days. Defines the cohort ranges:
/// [0, B[0]), [B[0], B[1]), [B[1], B[2]), ..., [B[n-1], ∞)
pub const AGE_BOUNDARIES: [usize; 19] = [
    1,         // up_to_1d | _1d_to_1w
    7,         // _1d_to_1w | _1w_to_1m
    30,        // _1w_to_1m | _1m_to_2m
    2 * 30,    // _1m_to_2m | _2m_to_3m
    3 * 30,    // _2m_to_3m | _3m_to_4m
    4 * 30,    // _3m_to_4m | _4m_to_5m
    5 * 30,    // _4m_to_5m | _5m_to_6m
    6 * 30,    // _5m_to_6m | _6m_to_1y
    365,       // _6m_to_1y | _1y_to_2y
    2 * 365,   // _1y_to_2y | _2y_to_3y
    3 * 365,   // _2y_to_3y | _3y_to_4y
    4 * 365,   // _3y_to_4y | _4y_to_5y
    5 * 365,   // _4y_to_5y | _5y_to_6y
    6 * 365,   // _5y_to_6y | _6y_to_7y
    7 * 365,   // _6y_to_7y | _7y_to_8y
    8 * 365,   // _7y_to_8y | _8y_to_10y
    10 * 365,  // _8y_to_10y | _10y_to_12y
    12 * 365,  // _10y_to_12y | _12y_to_15y
    15 * 365,  // _12y_to_15y | from_15y
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
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            up_to_1d: create(Filter::Time(TimeFilter::Range(0..AGE_BOUNDARIES[0]))),
            _1d_to_1w: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[0]..AGE_BOUNDARIES[1]))),
            _1w_to_1m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[1]..AGE_BOUNDARIES[2]))),
            _1m_to_2m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[2]..AGE_BOUNDARIES[3]))),
            _2m_to_3m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[3]..AGE_BOUNDARIES[4]))),
            _3m_to_4m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[4]..AGE_BOUNDARIES[5]))),
            _4m_to_5m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[5]..AGE_BOUNDARIES[6]))),
            _5m_to_6m: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[6]..AGE_BOUNDARIES[7]))),
            _6m_to_1y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[7]..AGE_BOUNDARIES[8]))),
            _1y_to_2y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[8]..AGE_BOUNDARIES[9]))),
            _2y_to_3y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[9]..AGE_BOUNDARIES[10]))),
            _3y_to_4y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[10]..AGE_BOUNDARIES[11]))),
            _4y_to_5y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[11]..AGE_BOUNDARIES[12]))),
            _5y_to_6y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[12]..AGE_BOUNDARIES[13]))),
            _6y_to_7y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[13]..AGE_BOUNDARIES[14]))),
            _7y_to_8y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[14]..AGE_BOUNDARIES[15]))),
            _8y_to_10y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[15]..AGE_BOUNDARIES[16]))),
            _10y_to_12y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[16]..AGE_BOUNDARIES[17]))),
            _12y_to_15y: create(Filter::Time(TimeFilter::Range(AGE_BOUNDARIES[17]..AGE_BOUNDARIES[18]))),
            from_15y: create(Filter::Time(TimeFilter::GreaterOrEqual(AGE_BOUNDARIES[18]))),
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


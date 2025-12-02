use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{Filter, TimeFilter};

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
            up_to_1d: create(Filter::Time(TimeFilter::Range(0..1))),
            _1d_to_1w: create(Filter::Time(TimeFilter::Range(1..7))),
            _1w_to_1m: create(Filter::Time(TimeFilter::Range(7..30))),
            _1m_to_2m: create(Filter::Time(TimeFilter::Range(30..2 * 30))),
            _2m_to_3m: create(Filter::Time(TimeFilter::Range(2 * 30..3 * 30))),
            _3m_to_4m: create(Filter::Time(TimeFilter::Range(3 * 30..4 * 30))),
            _4m_to_5m: create(Filter::Time(TimeFilter::Range(4 * 30..5 * 30))),
            _5m_to_6m: create(Filter::Time(TimeFilter::Range(5 * 30..6 * 30))),
            _6m_to_1y: create(Filter::Time(TimeFilter::Range(6 * 30..365))),
            _1y_to_2y: create(Filter::Time(TimeFilter::Range(365..2 * 365))),
            _2y_to_3y: create(Filter::Time(TimeFilter::Range(2 * 365..3 * 365))),
            _3y_to_4y: create(Filter::Time(TimeFilter::Range(3 * 365..4 * 365))),
            _4y_to_5y: create(Filter::Time(TimeFilter::Range(4 * 365..5 * 365))),
            _5y_to_6y: create(Filter::Time(TimeFilter::Range(5 * 365..6 * 365))),
            _6y_to_7y: create(Filter::Time(TimeFilter::Range(6 * 365..7 * 365))),
            _7y_to_8y: create(Filter::Time(TimeFilter::Range(7 * 365..8 * 365))),
            _8y_to_10y: create(Filter::Time(TimeFilter::Range(8 * 365..10 * 365))),
            _10y_to_12y: create(Filter::Time(TimeFilter::Range(10 * 365..12 * 365))),
            _12y_to_15y: create(Filter::Time(TimeFilter::Range(12 * 365..15 * 365))),
            from_15y: create(Filter::Time(TimeFilter::GreaterOrEqual(15 * 365))),
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


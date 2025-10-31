use brk_traversable::Traversable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Filtered;

use super::Filter;

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

impl<T> From<ByAgeRange<T>> for ByAgeRange<Filtered<T>> {
    #[inline]
    fn from(value: ByAgeRange<T>) -> Self {
        Self {
            up_to_1d: (Filter::LowerThan(1), value.up_to_1d).into(),
            _1d_to_1w: (Filter::Range(1..7), value._1d_to_1w).into(),
            _1w_to_1m: (Filter::Range(7..30), value._1w_to_1m).into(),
            _1m_to_2m: (Filter::Range(30..2 * 30), value._1m_to_2m).into(),
            _2m_to_3m: (Filter::Range(2 * 30..3 * 30), value._2m_to_3m).into(),
            _3m_to_4m: (Filter::Range(3 * 30..4 * 30), value._3m_to_4m).into(),
            _4m_to_5m: (Filter::Range(4 * 30..5 * 30), value._4m_to_5m).into(),
            _5m_to_6m: (Filter::Range(5 * 30..6 * 30), value._5m_to_6m).into(),
            _6m_to_1y: (Filter::Range(6 * 30..365), value._6m_to_1y).into(),
            _1y_to_2y: (Filter::Range(365..2 * 365), value._1y_to_2y).into(),
            _2y_to_3y: (Filter::Range(2 * 365..3 * 365), value._2y_to_3y).into(),
            _3y_to_4y: (Filter::Range(3 * 365..4 * 365), value._3y_to_4y).into(),
            _4y_to_5y: (Filter::Range(4 * 365..5 * 365), value._4y_to_5y).into(),
            _5y_to_6y: (Filter::Range(5 * 365..6 * 365), value._5y_to_6y).into(),
            _6y_to_7y: (Filter::Range(6 * 365..7 * 365), value._6y_to_7y).into(),
            _7y_to_8y: (Filter::Range(7 * 365..8 * 365), value._7y_to_8y).into(),
            _8y_to_10y: (Filter::Range(8 * 365..10 * 365), value._8y_to_10y).into(),
            _10y_to_12y: (Filter::Range(10 * 365..12 * 365), value._10y_to_12y).into(),
            _12y_to_15y: (Filter::Range(12 * 365..15 * 365), value._12y_to_15y).into(),
            from_15y: (Filter::GreaterOrEqual(15 * 365), value.from_15y).into(),
        }
    }
}

impl<T> ByAgeRange<T> {
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

impl<T> ByAgeRange<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self.up_to_1d.1,
            &self._1d_to_1w.1,
            &self._1w_to_1m.1,
            &self._1m_to_2m.1,
            &self._2m_to_3m.1,
            &self._3m_to_4m.1,
            &self._4m_to_5m.1,
            &self._5m_to_6m.1,
            &self._6m_to_1y.1,
            &self._1y_to_2y.1,
            &self._2y_to_3y.1,
            &self._3y_to_4y.1,
            &self._4y_to_5y.1,
            &self._5y_to_6y.1,
            &self._6y_to_7y.1,
            &self._7y_to_8y.1,
            &self._8y_to_10y.1,
            &self._10y_to_12y.1,
            &self._12y_to_15y.1,
            &self.from_15y.1,
        ]
        .into_iter()
    }
}

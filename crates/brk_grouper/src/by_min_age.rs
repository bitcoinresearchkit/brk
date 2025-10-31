use brk_traversable::Traversable;

use crate::Filtered;

use super::Filter;

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

impl<T> ByMinAge<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self._1d.1,
            &self._1w.1,
            &self._1m.1,
            &self._2m.1,
            &self._3m.1,
            &self._4m.1,
            &self._5m.1,
            &self._6m.1,
            &self._1y.1,
            &self._2y.1,
            &self._3y.1,
            &self._4y.1,
            &self._5y.1,
            &self._6y.1,
            &self._7y.1,
            &self._8y.1,
            &self._10y.1,
            &self._12y.1,
        ]
        .into_iter()
    }
}

impl<T> From<ByMinAge<T>> for ByMinAge<Filtered<T>> {
    #[inline]
    fn from(value: ByMinAge<T>) -> Self {
        Self {
            _1d: (Filter::GreaterOrEqual(1), value._1d).into(),
            _1w: (Filter::GreaterOrEqual(7), value._1w).into(),
            _1m: (Filter::GreaterOrEqual(30), value._1m).into(),
            _2m: (Filter::GreaterOrEqual(2 * 30), value._2m).into(),
            _3m: (Filter::GreaterOrEqual(3 * 30), value._3m).into(),
            _4m: (Filter::GreaterOrEqual(4 * 30), value._4m).into(),
            _5m: (Filter::GreaterOrEqual(5 * 30), value._5m).into(),
            _6m: (Filter::GreaterOrEqual(6 * 30), value._6m).into(),
            _1y: (Filter::GreaterOrEqual(365), value._1y).into(),
            _2y: (Filter::GreaterOrEqual(2 * 365), value._2y).into(),
            _3y: (Filter::GreaterOrEqual(3 * 365), value._3y).into(),
            _4y: (Filter::GreaterOrEqual(4 * 365), value._4y).into(),
            _5y: (Filter::GreaterOrEqual(5 * 365), value._5y).into(),
            _6y: (Filter::GreaterOrEqual(6 * 365), value._6y).into(),
            _7y: (Filter::GreaterOrEqual(7 * 365), value._7y).into(),
            _8y: (Filter::GreaterOrEqual(8 * 365), value._8y).into(),
            _10y: (Filter::GreaterOrEqual(10 * 365), value._10y).into(),
            _12y: (Filter::GreaterOrEqual(12 * 365), value._12y).into(),
        }
    }
}

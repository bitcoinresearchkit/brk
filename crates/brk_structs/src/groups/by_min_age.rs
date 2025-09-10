use super::GroupFilter;

#[derive(Default, Clone)]
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

impl<T> ByMinAge<(GroupFilter, T)> {
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

impl<T> From<ByMinAge<T>> for ByMinAge<(GroupFilter, T)> {
    fn from(value: ByMinAge<T>) -> Self {
        Self {
            _1d: (GroupFilter::GreaterOrEqual(1), value._1d),
            _1w: (GroupFilter::GreaterOrEqual(7), value._1w),
            _1m: (GroupFilter::GreaterOrEqual(30), value._1m),
            _2m: (GroupFilter::GreaterOrEqual(2 * 30), value._2m),
            _3m: (GroupFilter::GreaterOrEqual(3 * 30), value._3m),
            _4m: (GroupFilter::GreaterOrEqual(4 * 30), value._4m),
            _5m: (GroupFilter::GreaterOrEqual(5 * 30), value._5m),
            _6m: (GroupFilter::GreaterOrEqual(6 * 30), value._6m),
            _1y: (GroupFilter::GreaterOrEqual(365), value._1y),
            _2y: (GroupFilter::GreaterOrEqual(2 * 365), value._2y),
            _3y: (GroupFilter::GreaterOrEqual(3 * 365), value._3y),
            _4y: (GroupFilter::GreaterOrEqual(4 * 365), value._4y),
            _5y: (GroupFilter::GreaterOrEqual(5 * 365), value._5y),
            _6y: (GroupFilter::GreaterOrEqual(6 * 365), value._6y),
            _7y: (GroupFilter::GreaterOrEqual(7 * 365), value._7y),
            _8y: (GroupFilter::GreaterOrEqual(8 * 365), value._8y),
            _10y: (GroupFilter::GreaterOrEqual(10 * 365), value._10y),
            _12y: (GroupFilter::GreaterOrEqual(12 * 365), value._12y),
        }
    }
}

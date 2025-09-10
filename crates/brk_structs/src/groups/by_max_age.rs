use super::GroupFilter;

#[derive(Default, Clone)]
pub struct ByMaxAge<T> {
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
    pub _15y: T,
}

impl<T> ByMaxAge<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
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
            &mut self._15y,
        ]
        .into_iter()
    }
}

impl<T> ByMaxAge<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
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
            &self._15y.1,
        ]
        .into_iter()
    }
}

impl<T> From<ByMaxAge<T>> for ByMaxAge<(GroupFilter, T)> {
    fn from(value: ByMaxAge<T>) -> Self {
        Self {
            _1w: (GroupFilter::LowerThan(7), value._1w),
            _1m: (GroupFilter::LowerThan(30), value._1m),
            _2m: (GroupFilter::LowerThan(2 * 30), value._2m),
            _3m: (GroupFilter::LowerThan(3 * 30), value._3m),
            _4m: (GroupFilter::LowerThan(4 * 30), value._4m),
            _5m: (GroupFilter::LowerThan(5 * 30), value._5m),
            _6m: (GroupFilter::LowerThan(6 * 30), value._6m),
            _1y: (GroupFilter::LowerThan(365), value._1y),
            _2y: (GroupFilter::LowerThan(2 * 365), value._2y),
            _3y: (GroupFilter::LowerThan(3 * 365), value._3y),
            _4y: (GroupFilter::LowerThan(4 * 365), value._4y),
            _5y: (GroupFilter::LowerThan(5 * 365), value._5y),
            _6y: (GroupFilter::LowerThan(6 * 365), value._6y),
            _7y: (GroupFilter::LowerThan(7 * 365), value._7y),
            _8y: (GroupFilter::LowerThan(8 * 365), value._8y),
            _10y: (GroupFilter::LowerThan(10 * 365), value._10y),
            _12y: (GroupFilter::LowerThan(12 * 365), value._12y),
            _15y: (GroupFilter::LowerThan(15 * 365), value._15y),
        }
    }
}

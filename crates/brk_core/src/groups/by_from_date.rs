use super::GroupFilter;

#[derive(Default, Clone)]
pub struct GroupedByFromDate<T> {
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
    pub _15y: T,
}

impl<T> GroupedByFromDate<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 18] {
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
            &mut self._15y,
        ]
    }
}

impl<T> GroupedByFromDate<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 18] {
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
            &self._15y.1,
        ]
    }
}

impl<T> From<GroupedByFromDate<T>> for GroupedByFromDate<(GroupFilter, T)> {
    fn from(value: GroupedByFromDate<T>) -> Self {
        Self {
            _1d: (GroupFilter::From(1), value._1d),
            _1w: (GroupFilter::From(7), value._1w),
            _1m: (GroupFilter::From(30), value._1m),
            _2m: (GroupFilter::From(2 * 30), value._2m),
            _3m: (GroupFilter::From(3 * 30), value._3m),
            _4m: (GroupFilter::From(4 * 30), value._4m),
            _5m: (GroupFilter::From(5 * 30), value._5m),
            _6m: (GroupFilter::From(6 * 30), value._6m),
            _1y: (GroupFilter::From(365), value._1y),
            _2y: (GroupFilter::From(2 * 365), value._2y),
            _3y: (GroupFilter::From(3 * 365), value._3y),
            _4y: (GroupFilter::From(4 * 365), value._4y),
            _5y: (GroupFilter::From(5 * 365), value._5y),
            _6y: (GroupFilter::From(6 * 365), value._6y),
            _7y: (GroupFilter::From(7 * 365), value._7y),
            _8y: (GroupFilter::From(8 * 365), value._8y),
            _10y: (GroupFilter::From(10 * 365), value._10y),
            _15y: (GroupFilter::From(15 * 365), value._15y),
        }
    }
}

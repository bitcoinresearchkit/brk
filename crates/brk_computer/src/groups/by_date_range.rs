use super::GroupFilter;

#[derive(Default, Clone)]
pub struct GroupedByDateRange<T> {
    pub start_to_1d: T,
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
    pub _10y_to_15y: T,
    pub _15y_to_end: T,
}

impl<T> From<GroupedByDateRange<T>> for GroupedByDateRange<(GroupFilter, T)> {
    fn from(value: GroupedByDateRange<T>) -> Self {
        Self {
            start_to_1d: (GroupFilter::To(1), value.start_to_1d),
            _1d_to_1w: (GroupFilter::Range(1..7), value._1d_to_1w),
            _1w_to_1m: (GroupFilter::Range(7..30), value._1w_to_1m),
            _1m_to_2m: (GroupFilter::Range(30..2 * 30), value._1m_to_2m),
            _2m_to_3m: (GroupFilter::Range(2 * 30..3 * 30), value._2m_to_3m),
            _3m_to_4m: (GroupFilter::Range(3 * 30..4 * 30), value._3m_to_4m),
            _4m_to_5m: (GroupFilter::Range(4 * 30..5 * 30), value._4m_to_5m),
            _5m_to_6m: (GroupFilter::Range(5 * 30..6 * 30), value._5m_to_6m),
            _6m_to_1y: (GroupFilter::Range(6 * 30..365), value._6m_to_1y),
            _1y_to_2y: (GroupFilter::Range(365..2 * 365), value._1y_to_2y),
            _2y_to_3y: (GroupFilter::Range(2 * 365..3 * 365), value._2y_to_3y),
            _3y_to_4y: (GroupFilter::Range(3 * 365..4 * 365), value._3y_to_4y),
            _4y_to_5y: (GroupFilter::Range(4 * 365..5 * 365), value._4y_to_5y),
            _5y_to_6y: (GroupFilter::Range(5 * 365..6 * 365), value._5y_to_6y),
            _6y_to_7y: (GroupFilter::Range(6 * 365..7 * 365), value._6y_to_7y),
            _7y_to_8y: (GroupFilter::Range(7 * 365..8 * 365), value._7y_to_8y),
            _8y_to_10y: (GroupFilter::Range(8 * 365..10 * 365), value._8y_to_10y),
            _10y_to_15y: (GroupFilter::Range(10 * 365..15 * 365), value._10y_to_15y),
            _15y_to_end: (GroupFilter::From(15 * 365), value._15y_to_end),
        }
    }
}

impl<T> GroupedByDateRange<T> {
    pub fn as_vec(&mut self) -> [&T; 19] {
        [
            &self.start_to_1d,
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
            &self._10y_to_15y,
            &self._15y_to_end,
        ]
    }

    pub fn as_mut_vec(&mut self) -> [&mut T; 19] {
        [
            &mut self.start_to_1d,
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
            &mut self._10y_to_15y,
            &mut self._15y_to_end,
        ]
    }
}

impl<T> GroupedByDateRange<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 19] {
        [
            &self.start_to_1d.1,
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
            &self._10y_to_15y.1,
            &self._15y_to_end.1,
        ]
    }
}

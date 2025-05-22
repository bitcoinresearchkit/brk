use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsByRange<T> {
    pub _1d_to_1w: T,
    pub _1w_to_1m: T,
    pub _1m_to_3m: T,
    pub _3m_to_6m: T,
    pub _6m_to_1y: T,
    pub _1y_to_2y: T,
    pub _2y_to_3y: T,
    pub _3y_to_4y: T,
    pub _4y_to_5y: T,
    pub _5y_to_7y: T,
    pub _7y_to_10y: T,
    pub _10y_to_15y: T,
}

impl<T> From<OutputsByRange<T>> for OutputsByRange<(OutputFilter, T)> {
    fn from(value: OutputsByRange<T>) -> Self {
        Self {
            _1d_to_1w: (OutputFilter::Range(1..7), value._1d_to_1w),
            _1w_to_1m: (OutputFilter::Range(7..30), value._1w_to_1m),
            _1m_to_3m: (OutputFilter::Range(30..3 * 30), value._1m_to_3m),
            _3m_to_6m: (OutputFilter::Range(3 * 30..6 * 30), value._3m_to_6m),
            _6m_to_1y: (OutputFilter::Range(6 * 30..365), value._6m_to_1y),
            _1y_to_2y: (OutputFilter::Range(365..2 * 365), value._1y_to_2y),
            _2y_to_3y: (OutputFilter::Range(2 * 365..3 * 365), value._2y_to_3y),
            _3y_to_4y: (OutputFilter::Range(3 * 365..4 * 365), value._3y_to_4y),
            _4y_to_5y: (OutputFilter::Range(4 * 365..5 * 365), value._4y_to_5y),
            _5y_to_7y: (OutputFilter::Range(5 * 365..7 * 365), value._5y_to_7y),
            _7y_to_10y: (OutputFilter::Range(7 * 365..10 * 365), value._7y_to_10y),
            _10y_to_15y: (OutputFilter::Range(10 * 365..15 * 365), value._10y_to_15y),
        }
    }
}

impl<T> OutputsByRange<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 12] {
        [
            &mut self._1d_to_1w,
            &mut self._1w_to_1m,
            &mut self._1m_to_3m,
            &mut self._3m_to_6m,
            &mut self._6m_to_1y,
            &mut self._1y_to_2y,
            &mut self._2y_to_3y,
            &mut self._3y_to_4y,
            &mut self._4y_to_5y,
            &mut self._5y_to_7y,
            &mut self._7y_to_10y,
            &mut self._10y_to_15y,
        ]
    }
}

impl<T> OutputsByRange<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 12] {
        [
            &self._1d_to_1w.1,
            &self._1w_to_1m.1,
            &self._1m_to_3m.1,
            &self._3m_to_6m.1,
            &self._6m_to_1y.1,
            &self._1y_to_2y.1,
            &self._2y_to_3y.1,
            &self._3y_to_4y.1,
            &self._4y_to_5y.1,
            &self._5y_to_7y.1,
            &self._7y_to_10y.1,
            &self._10y_to_15y.1,
        ]
    }
}

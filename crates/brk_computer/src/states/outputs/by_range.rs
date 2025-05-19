#[derive(Default, Clone)]
pub struct OutputsByRange<T> {
    pub _1d_to_1w: T,
    pub _1w_to_1m: T,
    pub _1m_to_3m: T,
    pub _3m_to_6m: T,
    pub _6m_to_1y: T,
    pub _1y_to_2y: T,
    pub _2y_to_3y: T,
    pub _3y_to_5y: T,
    pub _5y_to_7y: T,
    pub _7y_to_10y: T,
    pub _10y_to_15y: T,
}

impl<T> OutputsByRange<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![
            &mut self._1d_to_1w,
            &mut self._1w_to_1m,
            &mut self._1m_to_3m,
            &mut self._3m_to_6m,
            &mut self._6m_to_1y,
            &mut self._1y_to_2y,
            &mut self._2y_to_3y,
            &mut self._3y_to_5y,
            &mut self._5y_to_7y,
            &mut self._7y_to_10y,
            &mut self._10y_to_15y,
        ]
    }
}

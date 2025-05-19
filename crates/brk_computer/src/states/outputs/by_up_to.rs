#[derive(Default, Clone)]
pub struct OutputsByUpTo<T> {
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
    pub _5y: T,
    pub _7y: T,
    pub _10y: T,
    pub _15y: T,
}

impl<T> OutputsByUpTo<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![
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
            &mut self._5y,
            &mut self._7y,
            &mut self._10y,
            &mut self._15y,
        ]
    }
}

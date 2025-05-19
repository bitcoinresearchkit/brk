#[derive(Default, Clone)]
pub struct OutputsByFrom<T> {
    pub _1y: T,
    pub _2y: T,
    pub _4y: T,
    pub _10y: T,
    pub _15y: T,
}

impl<T> OutputsByFrom<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![
            &mut self._1y,
            &mut self._2y,
            &mut self._4y,
            &mut self._10y,
            &mut self._15y,
        ]
    }
}

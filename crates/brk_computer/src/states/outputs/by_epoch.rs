#[derive(Default, Clone)]
pub struct OutputsByEpoch<T> {
    pub _1: T,
    pub _2: T,
    pub _3: T,
    pub _4: T,
    pub _5: T,
}

impl<T> OutputsByEpoch<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![
            &mut self._1,
            &mut self._2,
            &mut self._3,
            &mut self._4,
            &mut self._5,
        ]
    }
}

use brk_core::{HalvingEpoch, Height};

use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsByEpoch<T> {
    pub _0: T,
    pub _1: T,
    pub _2: T,
    pub _3: T,
    pub _4: T,
}

impl<T> From<OutputsByEpoch<T>> for OutputsByEpoch<(OutputFilter, T)> {
    fn from(value: OutputsByEpoch<T>) -> Self {
        Self {
            _0: (OutputFilter::Epoch(HalvingEpoch::new(0)), value._0),
            _1: (OutputFilter::Epoch(HalvingEpoch::new(1)), value._1),
            _2: (OutputFilter::Epoch(HalvingEpoch::new(2)), value._2),
            _3: (OutputFilter::Epoch(HalvingEpoch::new(3)), value._3),
            _4: (OutputFilter::Epoch(HalvingEpoch::new(4)), value._4),
        }
    }
}

impl<T> OutputsByEpoch<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 5] {
        [
            &mut self._0,
            &mut self._1,
            &mut self._2,
            &mut self._3,
            &mut self._4,
        ]
    }

    pub fn mut_vec_from_height(&mut self, height: Height) -> &mut T {
        let epoch = HalvingEpoch::from(height);
        if epoch == HalvingEpoch::new(0) {
            &mut self._0
        } else if epoch == HalvingEpoch::new(1) {
            &mut self._1
        } else if epoch == HalvingEpoch::new(2) {
            &mut self._2
        } else if epoch == HalvingEpoch::new(3) {
            &mut self._3
        } else if epoch == HalvingEpoch::new(4) {
            &mut self._4
        } else {
            todo!("")
        }
    }
}

impl<T> OutputsByEpoch<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 5] {
        [&self._0.1, &self._1.1, &self._2.1, &self._3.1, &self._4.1]
    }
}

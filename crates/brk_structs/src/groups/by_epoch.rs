use crate::{HalvingEpoch, Height};

use super::GroupFilter;

#[derive(Default, Clone)]
pub struct ByEpoch<T> {
    pub _0: T,
    pub _1: T,
    pub _2: T,
    pub _3: T,
    pub _4: T,
}

impl<T> From<ByEpoch<T>> for ByEpoch<(GroupFilter, T)> {
    fn from(value: ByEpoch<T>) -> Self {
        Self {
            _0: (GroupFilter::Epoch(HalvingEpoch::new(0)), value._0),
            _1: (GroupFilter::Epoch(HalvingEpoch::new(1)), value._1),
            _2: (GroupFilter::Epoch(HalvingEpoch::new(2)), value._2),
            _3: (GroupFilter::Epoch(HalvingEpoch::new(3)), value._3),
            _4: (GroupFilter::Epoch(HalvingEpoch::new(4)), value._4),
        }
    }
}

impl<T> ByEpoch<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._0,
            &mut self._1,
            &mut self._2,
            &mut self._3,
            &mut self._4,
        ]
        .into_iter()
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

impl<T> ByEpoch<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self._0.1, &self._1.1, &self._2.1, &self._3.1, &self._4.1].into_iter()
    }
}

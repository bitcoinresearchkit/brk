use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, Height};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Filter;

#[derive(Default, Clone, Traversable)]
pub struct ByEpoch<T> {
    pub _0: T,
    pub _1: T,
    pub _2: T,
    pub _3: T,
    pub _4: T,
}

impl<T> ByEpoch<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            _0: create(Filter::Epoch(HalvingEpoch::new(0))),
            _1: create(Filter::Epoch(HalvingEpoch::new(1))),
            _2: create(Filter::Epoch(HalvingEpoch::new(2))),
            _3: create(Filter::Epoch(HalvingEpoch::new(3))),
            _4: create(Filter::Epoch(HalvingEpoch::new(4))),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self._0, &self._1, &self._2, &self._3, &self._4].into_iter()
    }

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

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self._0,
            &mut self._1,
            &mut self._2,
            &mut self._3,
            &mut self._4,
        ]
        .into_par_iter()
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

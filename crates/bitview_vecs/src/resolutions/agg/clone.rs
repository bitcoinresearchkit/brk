use std::marker::PhantomData;

use vecdb::{VecIndex, VecValue};

use super::LazyAggVec;

impl<I, O, S1I, S1T, Strat> Clone for LazyAggVec<I, O, S1I, S1T, Strat>
where
    I: VecIndex,
    O: VecValue,
    S1I: VecIndex,
    S1T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version,
            source: self.source.clone(),
            mapping: self.mapping.clone(),
            _phantom: PhantomData,
        }
    }
}

use std::marker::PhantomData;

use crate::{VecIndex, VecValue};

use super::LazyAggVec;

impl<I, O, S1I, S2T, S1T, Strat> Clone for LazyAggVec<I, O, S1I, S2T, S1T, Strat>
where
    I: VecIndex,
    O: VecValue,
    S1I: VecIndex,
    S2T: VecValue,
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

use std::marker::PhantomData;

use crate::{VecIndex, VecValue};

use super::LazyDeltaVec;

impl<I, S, T, Op> Clone for LazyDeltaVec<I, S, T, Op>
where
    I: VecIndex,
    S: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            base_version: self.base_version,
            source: self.source.clone(),
            window_starts: self.window_starts.clone(),
            _op: PhantomData,
        }
    }
}

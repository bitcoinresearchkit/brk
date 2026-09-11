use crate::{TypedVec, VecIndex, VecValue, cache::CachePolicy};

use super::{super::RawStrategy, ReadOnlyRawVec};

impl<I, T, S, C: CachePolicy> TypedVec for ReadOnlyRawVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: RawStrategy<T>,
{
    type I = I;
    type T = T;
}

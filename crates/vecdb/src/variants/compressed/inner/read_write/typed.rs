use crate::{TypedVec, VecIndex, VecValue, cache::CachePolicy};

use super::{super::CompressionStrategy, ReadWriteCompressedVec};

impl<I, T, S, C: CachePolicy> TypedVec for ReadWriteCompressedVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: CompressionStrategy<T>,
{
    type I = I;
    type T = T;
}

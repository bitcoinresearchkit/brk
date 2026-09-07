use std::marker::PhantomData;

use vecdb::{ReadableBoxedVec, ReadableVec, UnaryTransform, VecIndex, VecValue};

pub struct Identity<T>(PhantomData<T>);

impl<T: VecValue> UnaryTransform<T, T> for Identity<T> {
    #[inline(always)]
    fn apply(v: T) -> T {
        v
    }

    fn read_into<I: VecIndex>(
        source: &ReadableBoxedVec<I, T>,
        from: usize,
        to: usize,
        out: &mut Vec<T>,
    ) {
        source.read_into_at(from, to, out);
    }

    fn for_each_chunk<I: VecIndex>(
        source: &ReadableBoxedVec<I, T>,
        from: usize,
        to: usize,
        f: &mut dyn FnMut(usize, &[T]),
    ) {
        source.for_each_chunk_at(from, to, f);
    }
}

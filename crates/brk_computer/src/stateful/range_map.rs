use std::collections::BTreeMap;

use vecdb::{CompressedVec, Pco, RawVec, VecIndex, VecValue};

#[derive(Debug)]
pub struct RangeMap<I, T>(BTreeMap<I, T>);

impl<I, T> RangeMap<I, T>
where
    I: VecIndex,
    T: VecIndex,
{
    pub fn get(&self, key: I) -> Option<&T> {
        self.0.range(..=key).next_back().map(|(&min, value)| {
            if min > key {
                unreachable!()
            }
            value
        })
    }
}

impl<I, T> From<&RawVec<I, T>> for RangeMap<T, I>
where
    I: VecIndex,
    T: VecIndex + VecValue,
{
    #[inline]
    fn from(vec: &RawVec<I, T>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(i, v)| (v, I::from(i)))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<I, T> From<&CompressedVec<I, T>> for RangeMap<T, I>
where
    I: VecIndex,
    T: VecIndex + Pco,
{
    #[inline]
    fn from(vec: &CompressedVec<I, T>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(i, v)| (v, I::from(i)))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

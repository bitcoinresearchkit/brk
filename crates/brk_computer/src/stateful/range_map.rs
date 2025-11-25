use std::collections::BTreeMap;

use vecdb::{BytesVec, BytesVecValue, PcoVec, PcoVecValue, VecIndex};

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

impl<I, T> From<&BytesVec<I, T>> for RangeMap<T, I>
where
    I: VecIndex,
    T: VecIndex + BytesVecValue,
{
    #[inline]
    fn from(vec: &BytesVec<I, T>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(i, v)| (v, I::from(i)))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<I, T> From<&PcoVec<I, T>> for RangeMap<T, I>
where
    I: VecIndex,
    T: VecIndex + PcoVecValue,
{
    #[inline]
    fn from(vec: &PcoVec<I, T>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(i, v)| (v, I::from(i)))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

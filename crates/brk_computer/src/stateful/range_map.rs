use std::collections::BTreeMap;

use brk_vecs::{StampedVec, StoredIndex, StoredType};

#[derive(Debug)]
pub struct RangeMap<I, T>(BTreeMap<I, T>);

impl<I, T> RangeMap<I, T>
where
    I: StoredIndex,
    T: StoredIndex,
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

impl<I, T> From<&StampedVec<I, T>> for RangeMap<T, I>
where
    I: StoredIndex,
    T: StoredIndex + StoredType,
{
    fn from(vec: &StampedVec<I, T>) -> Self {
        Self(
            vec.into_iter()
                .map(|(i, v)| (v.into_owned(), i))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

//! Main block processing loop.
//!
//! Iterates through blocks, processing outputs (receive) and inputs (send) in parallel.

use std::collections::BTreeMap;

use brk_types::{Height, TxOutIndex};
use vecdb::{BytesVec, BytesVecValue, PcoVec, PcoVecValue, VecIndex};

/// Maps ranges of indices to their corresponding height.
/// Used to efficiently look up which block a txoutindex belongs to.
#[derive(Debug)]
pub struct RangeMap<I, T>(BTreeMap<I, T>);

impl<I, T> RangeMap<I, T>
where
    I: VecIndex,
    T: VecIndex,
{
    /// Look up value for a key using range search.
    /// Returns the value associated with the largest key <= given key.
    #[inline]
    pub fn get(&self, key: I) -> Option<&T> {
        self.0.range(..=key).next_back().map(|(_, value)| value)
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
                .collect(),
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
                .collect(),
        )
    }
}

/// Creates a RangeMap from height_to_first_txoutindex for fast txoutindex -> height lookups.
pub fn build_txoutindex_to_height_map(
    height_to_first_txoutindex: &PcoVec<Height, TxOutIndex>,
) -> RangeMap<TxOutIndex, Height> {
    RangeMap::from(height_to_first_txoutindex)
}

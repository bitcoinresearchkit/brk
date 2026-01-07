//! Lazy first-value aggregation.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, IterableBoxedVec, LazyVecFrom2, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(wrap = "first")]
pub struct LazyFirst<I, T, S1I, S2T>(pub LazyVecFrom2<I, T, S1I, T, I, S2T>)
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue;

impl<I, T, S1I, S2T> LazyFirst<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    pub fn from_source(
        name: &str,
        version: Version,
        source: IterableBoxedVec<S1I, T>,
        len_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self(LazyVecFrom2::init(
            name,
            version + VERSION,
            source,
            len_source,
            |i: I, source, len_source| {
                if i.to_usize() >= len_source.vec_len() {
                    return None;
                }
                source.get_at(S1I::min_from(i))
            },
        ))
    }
}

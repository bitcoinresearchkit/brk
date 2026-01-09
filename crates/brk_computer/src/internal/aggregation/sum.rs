//! Lazy sum-value aggregation.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, IterableBoxedVec, LazyVecFrom2, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazySum<I, T, S1I, S2T>(pub LazyVecFrom2<I, T, S1I, T, I, S2T>)
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue;

impl<I, T, S1I, S2T> LazySum<I, T, S1I, S2T>
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
        Self::from_source_inner(&format!("{name}_sum"), version, source, len_source)
    }

    /// Create from source without adding _sum suffix.
    pub fn from_source_raw(
        name: &str,
        version: Version,
        source: IterableBoxedVec<S1I, T>,
        len_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self::from_source_inner(name, version, source, len_source)
    }

    fn from_source_inner(
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
                let mut sum = T::from(0);
                let mut has_values = false;
                for v in S1I::inclusive_range_from(i, source.vec_len())
                    .flat_map(|i| source.get_at(i))
                {
                    sum += v;
                    has_values = true;
                }
                if !has_values {
                    return None;
                }
                Some(sum)
            },
        ))
    }
}


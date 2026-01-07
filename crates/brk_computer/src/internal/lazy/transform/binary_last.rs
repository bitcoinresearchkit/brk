//! Lazy binary transform for Last-only.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2, VecIndex};

use crate::internal::{ComputedVecValue, LazyLast};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(wrap = "last")]
pub struct LazyTransform2Last<I, T, S1T, S2T>(pub LazyVecFrom2<I, T, I, S1T, I, S2T>)
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue;

impl<I, T, S1T, S2T> LazyTransform2Last<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_lazy_last<
        F: BinaryTransform<S1T, S2T, T>,
        S1I: VecIndex + 'static,
        S2I: VecIndex + 'static,
        S1S2T,
        S2S2T,
    >(
        name: &str,
        version: Version,
        source1: &LazyLast<I, S1T, S1I, S1S2T>,
        source2: &LazyLast<I, S2T, S2I, S2S2T>,
    ) -> Self
    where
        S1S2T: vecdb::VecValue,
        S2S2T: vecdb::VecValue,
    {
        Self(LazyVecFrom2::transformed::<F>(
            name,
            version,
            source1.boxed_clone(),
            source2.boxed_clone(),
        ))
    }

    pub fn from_vecs<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: IterableBoxedVec<I, S1T>,
        source2: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self(LazyVecFrom2::transformed::<F>(
            name, version, source1, source2,
        ))
    }
}

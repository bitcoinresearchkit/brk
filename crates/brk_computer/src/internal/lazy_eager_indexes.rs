//! LazyEagerIndexes - lazy per-period transform of EagerIndexes.
//!
//! Used for lazy currency transforms (e.g., cents→dollars, cents→sats)
//! of eagerly computed per-period data like OHLC.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Minute1, Minute10, Minute30, Minute5, Month1,
    Month3, Month6, Version, Week1, Year1, Year10, Hour1, Hour4, Hour12,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes_from,
    internal::{ComputedVecValue, EagerIndexes, Indexes},
};

pub type LazyEagerIndexesInner<T, S> = Indexes<
    LazyVecFrom1<Minute1, T, Minute1, S>,
    LazyVecFrom1<Minute5, T, Minute5, S>,
    LazyVecFrom1<Minute10, T, Minute10, S>,
    LazyVecFrom1<Minute30, T, Minute30, S>,
    LazyVecFrom1<Hour1, T, Hour1, S>,
    LazyVecFrom1<Hour4, T, Hour4, S>,
    LazyVecFrom1<Hour12, T, Hour12, S>,
    LazyVecFrom1<Day1, T, Day1, S>,
    LazyVecFrom1<Day3, T, Day3, S>,
    LazyVecFrom1<Week1, T, Week1, S>,
    LazyVecFrom1<Month1, T, Month1, S>,
    LazyVecFrom1<Month3, T, Month3, S>,
    LazyVecFrom1<Month6, T, Month6, S>,
    LazyVecFrom1<Year1, T, Year1, S>,
    LazyVecFrom1<Year10, T, Year10, S>,
    LazyVecFrom1<HalvingEpoch, T, HalvingEpoch, S>,
    LazyVecFrom1<DifficultyEpoch, T, DifficultyEpoch, S>,
>;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyEagerIndexes<T, S>(pub LazyEagerIndexesInner<T, S>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S: ComputedVecValue;

impl<T, S> LazyEagerIndexes<T, S>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S: ComputedVecValue + PartialOrd + JsonSchema,
{
    /// Create lazy per-period transforms from an EagerIndexes source.
    pub(crate) fn from_eager_indexes<Transform: UnaryTransform<S, T>>(
        name: &str,
        version: Version,
        source: &EagerIndexes<S>,
    ) -> Self {
        macro_rules! period {
            ($idx:ident) => {
                LazyVecFrom1::transformed::<Transform>(
                    &format!("{name}_{}", stringify!($idx)),
                    version,
                    source.$idx.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period))
    }
}

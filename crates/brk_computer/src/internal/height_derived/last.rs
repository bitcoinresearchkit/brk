//! ComputedHeightDerivedLast — sparse time periods + dense epochs (last value).

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, FromCoarserIndex, HalvingEpoch, Height, Hour1, Hour4, Hour12,
    Minute1, Minute5, Minute10, Minute30, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Cursor, LazyAggVec, ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, VecIndex, VecValue,
};

use crate::{
    indexes, indexes_from,
    internal::{ComputedVecValue, Indexes, NumericValue},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ComputedHeightDerivedLast<T>(
    #[allow(clippy::type_complexity)]
    pub  Indexes<
        LazyAggVec<Minute1, Option<T>, Height, Height, T>,
        LazyAggVec<Minute5, Option<T>, Height, Height, T>,
        LazyAggVec<Minute10, Option<T>, Height, Height, T>,
        LazyAggVec<Minute30, Option<T>, Height, Height, T>,
        LazyAggVec<Hour1, Option<T>, Height, Height, T>,
        LazyAggVec<Hour4, Option<T>, Height, Height, T>,
        LazyAggVec<Hour12, Option<T>, Height, Height, T>,
        LazyAggVec<Day1, Option<T>, Height, Height, T>,
        LazyAggVec<Day3, Option<T>, Height, Height, T>,
        LazyAggVec<Week1, Option<T>, Height, Height, T>,
        LazyAggVec<Month1, Option<T>, Height, Height, T>,
        LazyAggVec<Month3, Option<T>, Height, Height, T>,
        LazyAggVec<Month6, Option<T>, Height, Height, T>,
        LazyAggVec<Year1, Option<T>, Height, Height, T>,
        LazyAggVec<Year10, Option<T>, Height, Height, T>,
        LazyAggVec<HalvingEpoch, T, Height, HalvingEpoch>,
        LazyAggVec<DifficultyEpoch, T, Height, DifficultyEpoch>,
    >,
)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

/// Already read-only (no StorageMode); cloning is sufficient.
impl<T> ReadOnlyClone for ComputedHeightDerivedLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    type ReadOnly = Self;
    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedLast<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        name: &str,
        height_source: ReadableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazyAggVec::sparse_from_first_index(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        fn for_each_range_from_coarser<
            I: VecIndex,
            O: VecValue,
            S1I: VecIndex + FromCoarserIndex<I>,
            S2T: VecValue,
        >(
            from: usize,
            to: usize,
            source: &ReadableBoxedVec<S1I, O>,
            mapping: &ReadableBoxedVec<I, S2T>,
            f: &mut dyn FnMut(O),
        ) {
            let mapping_len = mapping.len();
            let source_len = source.len();
            let mut cursor = Cursor::from_dyn(&**source);
            for i in from..to {
                if i >= mapping_len {
                    break;
                }
                let target = S1I::max_from(I::from(i), source_len);
                if cursor.position() <= target {
                    cursor.advance(target - cursor.position());
                    if let Some(v) = cursor.next() {
                        f(v);
                    }
                } else if let Some(v) = source.collect_one_at(target) {
                    f(v);
                }
            }
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazyAggVec::new(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.identity.read_only_boxed_clone(),
                    for_each_range_from_coarser,
                )
            };
        }

        Self(indexes_from!(period, epoch))
    }
}

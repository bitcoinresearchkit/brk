use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, FromCoarserIndex, HalvingEpoch, Height, Hour1, Hour4, Hour12,
    Minute10, Minute30, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Cursor, LazyAggVec, ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, VecIndex, VecValue,
};

use crate::{
    indexes,
    internal::{ComputedVecValue, NumericValue, PerPeriod},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ComputedHeightDerived<T>(
    #[allow(clippy::type_complexity)]
    pub  PerPeriod<
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

impl<T> ReadOnlyClone for ComputedHeightDerived<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    type ReadOnly = Self;
    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> ComputedHeightDerived<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        name: &str,
        height_source: ReadableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Self {
        macro_rules! period {
            ($idx:ident) => {
                LazyAggVec::sparse_from_first_index(
                    name,
                    version,
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
                    version,
                    height_source.clone(),
                    indexes.$idx.identity.read_only_boxed_clone(),
                    for_each_range_from_coarser,
                )
            };
        }

        Self(PerPeriod {
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: epoch!(halvingepoch),
            difficultyepoch: epoch!(difficultyepoch),
        })
    }
}

//! ComputedHeightDerivedLast - lazy time periods + epochs (last value).
//!
//! Newtype on `Indexes` with `LazyLast` per field.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec};

use crate::{
    indexes,
    indexes_from,
    internal::{ComputedVecValue, Indexes, LazyLast, NumericValue},
};

/// All 17 time-period/epoch `LazyLast` vecs, packed as a newtype on `Indexes`.
pub type ComputedHeightDerivedLastInner<T> = Indexes<
    LazyLast<Minute1, T, Height, Height>,
    LazyLast<Minute5, T, Height, Height>,
    LazyLast<Minute10, T, Height, Height>,
    LazyLast<Minute30, T, Height, Height>,
    LazyLast<Hour1, T, Height, Height>,
    LazyLast<Hour4, T, Height, Height>,
    LazyLast<Hour12, T, Height, Height>,
    LazyLast<Day1, T, Height, Height>,
    LazyLast<Day3, T, Height, Height>,
    LazyLast<Week1, T, Height, Height>,
    LazyLast<Month1, T, Height, Height>,
    LazyLast<Month3, T, Height, Height>,
    LazyLast<Month6, T, Height, Height>,
    LazyLast<Year1, T, Height, Height>,
    LazyLast<Year10, T, Height, Height>,
    LazyLast<HalvingEpoch, T, Height, HalvingEpoch>,
    LazyLast<DifficultyEpoch, T, Height, DifficultyEpoch>,
>;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ComputedHeightDerivedLast<T>(pub ComputedHeightDerivedLastInner<T>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

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
                LazyLast::from_height_source(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazyLast::from_source(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.identity.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period, epoch))
    }
}

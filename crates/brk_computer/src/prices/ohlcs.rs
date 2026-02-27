use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Cents, Close, Day1, Day3, DifficultyEpoch, HalvingEpoch, High, Hour1, Hour4, Hour12, Low,
    Minute1, Minute5, Minute10, Minute30, Month1, Month3, Month6, OHLCCents, Open, Version, Week1,
    Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    BytesVec, BytesVecValue, Database, EagerVec, Exit, Formattable, ImportableVec, LazyVecFrom1,
    ReadableCloneableVec, Rw, StorageMode, UnaryTransform,
};

use crate::{
    ComputeIndexes, indexes_from,
    internal::{ComputedHeightDerivedLast, EagerIndexes, Indexes},
};

// ── EagerOhlcIndexes ─────────────────────────────────────────────────

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct OhlcVecs<T, M: StorageMode = Rw>(
    #[allow(clippy::type_complexity)]
    pub  Indexes<
        <M as StorageMode>::Stored<EagerVec<BytesVec<Minute1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Minute5, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Minute10, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Minute30, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Hour1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Hour4, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Hour12, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Day1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Day3, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Week1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Month1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Month3, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Month6, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Year1, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<Year10, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<HalvingEpoch, T>>>,
        <M as StorageMode>::Stored<EagerVec<BytesVec<DifficultyEpoch, T>>>,
    >,
)
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema;

const EAGER_VERSION: Version = Version::ZERO;

impl<T> OhlcVecs<T>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema,
{
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let v = version + EAGER_VERSION;

        macro_rules! period {
            ($idx:ident) => {
                ImportableVec::forced_import(db, name, v)?
            };
        }

        Ok(Self(indexes_from!(period)))
    }
}

impl OhlcVecs<OHLCCents> {
    pub(crate) fn compute_from_split(
        &mut self,
        starting_indexes: &ComputeIndexes,
        open: &EagerIndexes<Cents>,
        high: &EagerIndexes<Cents>,
        low: &EagerIndexes<Cents>,
        close: &ComputedHeightDerivedLast<Cents>,
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! period {
            ($field:ident) => {
                self.0.$field.compute_transform4(
                    starting_indexes.$field,
                    &open.$field,
                    &high.$field,
                    &low.$field,
                    &close.$field,
                    |(idx, o, h, l, c, _)| {
                        (
                            idx,
                            OHLCCents {
                                open: Open::new(o),
                                high: High::new(h),
                                low: Low::new(l),
                                close: Close::new(c.unwrap_or_default()),
                            },
                        )
                    },
                    exit,
                )?;
            };
        }

        macro_rules! epoch {
            ($field:ident) => {
                self.0.$field.compute_transform4(
                    starting_indexes.$field,
                    &open.$field,
                    &high.$field,
                    &low.$field,
                    &close.$field,
                    |(idx, o, h, l, c, _)| {
                        (
                            idx,
                            OHLCCents {
                                open: Open::new(o),
                                high: High::new(h),
                                low: Low::new(l),
                                close: Close::new(c),
                            },
                        )
                    },
                    exit,
                )?;
            };
        }

        period!(minute1);
        period!(minute5);
        period!(minute10);
        period!(minute30);
        period!(hour1);
        period!(hour4);
        period!(hour12);
        period!(day1);
        period!(day3);
        period!(week1);
        period!(month1);
        period!(month3);
        period!(month6);
        period!(year1);
        period!(year10);
        epoch!(halvingepoch);
        epoch!(difficultyepoch);

        Ok(())
    }
}

// ── LazyOhlcIndexes ──────────────────────────────────────────────────

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyOhlcVecs<T, S>(
    #[allow(clippy::type_complexity)]
    pub  Indexes<
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
    >,
)
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema,
    S: BytesVecValue;

impl<T, S> LazyOhlcVecs<T, S>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema,
    S: BytesVecValue + Formattable + Serialize + JsonSchema,
{
    pub(crate) fn from_eager_ohlc_indexes<Transform: UnaryTransform<S, T>>(
        name: &str,
        version: Version,
        source: &OhlcVecs<S>,
    ) -> Self {
        macro_rules! period {
            ($idx:ident) => {
                LazyVecFrom1::transformed::<Transform>(
                    name,
                    version,
                    source.$idx.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period))
    }
}

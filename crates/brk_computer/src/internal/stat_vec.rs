use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

macro_rules! define_stat_vec {
    ($name:ident, $suffix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Deref, DerefMut, Traversable)]
        pub struct $name<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
            pub M::Stored<EagerVec<PcoVec<I, T>>>,
        );

        impl<I: VecIndex, T: ComputedVecValue + JsonSchema> $name<I, T> {
            pub(crate) fn forced_import(
                db: &Database,
                name: &str,
                version: Version,
            ) -> Result<Self> {
                Ok(Self(EagerVec::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                )?))
            }

            pub fn read_only_clone(&self) -> $name<I, T, Ro> {
                $name(StoredVec::read_only_clone(&self.0))
            }
        }
    };
}

macro_rules! define_stat_vec_transparent {
    ($name:ident, $suffix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Deref, DerefMut, Traversable)]
        #[traversable(transparent)]
        pub struct $name<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
            pub M::Stored<EagerVec<PcoVec<I, T>>>,
        );

        impl<I: VecIndex, T: ComputedVecValue + JsonSchema> $name<I, T> {
            pub(crate) fn forced_import(
                db: &Database,
                name: &str,
                version: Version,
            ) -> Result<Self> {
                Ok(Self(EagerVec::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                )?))
            }

            pub fn read_only_clone(&self) -> $name<I, T, Ro> {
                $name(StoredVec::read_only_clone(&self.0))
            }
        }
    };
}

define_stat_vec!(AverageVec, "average", "Average value in an aggregation period");
define_stat_vec!(MinVec, "min", "Minimum value in an aggregation period");
define_stat_vec!(MaxVec, "max", "Maximum value in an aggregation period");
define_stat_vec!(Pct10Vec, "pct10", "10th percentile in an aggregation period");
define_stat_vec!(Pct25Vec, "pct25", "25th percentile in an aggregation period");
define_stat_vec!(MedianVec, "median", "Median (50th percentile) in an aggregation period");
define_stat_vec!(Pct75Vec, "pct75", "75th percentile in an aggregation period");
define_stat_vec!(Pct90Vec, "pct90", "90th percentile in an aggregation period");

define_stat_vec_transparent!(CumulativeVec, "cumulative", "Cumulative sum across aggregation periods");

/// Sum of values in an aggregation period
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct SumVec<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
    pub M::Stored<EagerVec<PcoVec<I, T>>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> SumVec<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(
            db,
            &format!("{name}_sum"),
            version,
        )?))
    }

    #[inline]
    pub(crate) fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    pub fn read_only_clone(&self) -> SumVec<I, T, Ro> {
        SumVec(StoredVec::read_only_clone(&self.0))
    }
}

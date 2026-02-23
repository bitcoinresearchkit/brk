//! Percentile vec types for aggregation periods.

use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

macro_rules! define_percentile_vec {
    ($name:ident, $suffix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Deref, DerefMut, Traversable)]
        pub struct $name<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
            pub M::Stored<EagerVec<PcoVec<I, T>>>,
        );

        impl<I: VecIndex, T: ComputedVecValue + JsonSchema> $name<I, T> {
            pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
                Ok(Self(EagerVec::forced_import(db, &format!("{name}_{}", $suffix), version)?))
            }

            pub fn read_only_clone(&self) -> $name<I, T, Ro> {
                $name(StoredVec::read_only_clone(&self.0))
            }
        }
    };
}

define_percentile_vec!(Pct10Vec, "pct10", "10th percentile in an aggregation period");
define_percentile_vec!(Pct25Vec, "pct25", "25th percentile in an aggregation period");
define_percentile_vec!(MedianVec, "median", "Median (50th percentile) in an aggregation period");
define_percentile_vec!(Pct75Vec, "pct75", "75th percentile in an aggregation period");
define_percentile_vec!(Pct90Vec, "pct90", "90th percentile in an aggregation period");

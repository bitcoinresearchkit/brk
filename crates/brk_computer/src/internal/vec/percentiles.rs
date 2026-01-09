//! Percentile vec types for aggregation periods.

use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, VecIndex, Version};

use crate::internal::ComputedVecValue;

macro_rules! define_percentile_vec {
    ($name:ident, $suffix:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Deref, DerefMut, Traversable)]
        pub struct $name<I: VecIndex, T: ComputedVecValue + JsonSchema>(
            pub EagerVec<PcoVec<I, T>>,
        );

        impl<I: VecIndex, T: ComputedVecValue + JsonSchema> $name<I, T> {
            pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
                Ok(Self(EagerVec::forced_import(db, &format!("{name}_{}", $suffix), version)?))
            }

            #[inline]
            pub fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
                &self.0
            }
        }
    };
}

define_percentile_vec!(Pct10Vec, "pct10", "10th percentile in an aggregation period");
define_percentile_vec!(Pct25Vec, "pct25", "25th percentile in an aggregation period");
define_percentile_vec!(MedianVec, "median", "Median (50th percentile) in an aggregation period");
define_percentile_vec!(Pct75Vec, "pct75", "75th percentile in an aggregation period");
define_percentile_vec!(Pct90Vec, "pct90", "90th percentile in an aggregation period");

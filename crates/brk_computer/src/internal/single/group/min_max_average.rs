use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, ReadableBoxedVec, ReadableCloneableVec, Ro, Rw, StorageMode, VecIndex, Version};

use crate::internal::{AverageVec, ComputedVecValue};

use super::MinMax;

/// Average + MinMax (for TxIndex day1 aggregation - no percentiles)
#[derive(Traversable)]
pub struct MinMaxAverage<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    pub average: AverageVec<I, T, M>,
    #[traversable(flatten)]
    pub minmax: MinMax<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MinMaxAverage<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            average: AverageVec::forced_import(db, name, version)?,
            minmax: MinMax::forced_import(db, name, version)?,
        })
    }

    // Boxed accessors
    pub(crate) fn boxed_average(&self) -> ReadableBoxedVec<I, T> {
        self.average.0.read_only_boxed_clone()
    }

    pub(crate) fn boxed_min(&self) -> ReadableBoxedVec<I, T> {
        self.minmax.min.0.read_only_boxed_clone()
    }

    pub(crate) fn boxed_max(&self) -> ReadableBoxedVec<I, T> {
        self.minmax.max.0.read_only_boxed_clone()
    }

    pub fn read_only_clone(&self) -> MinMaxAverage<I, T, Ro> {
        MinMaxAverage {
            average: self.average.read_only_clone(),
            minmax: self.minmax.read_only_clone(),
        }
    }
}

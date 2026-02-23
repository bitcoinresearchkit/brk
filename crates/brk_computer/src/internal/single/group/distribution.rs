use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{
    Database, Exit, ReadableBoxedVec, ReadableVec, Ro, Rw, StorageMode, VecIndex, VecValue, Version,
};

use crate::internal::ComputedVecValue;

use super::{MinMaxAverage, Percentiles};

/// Distribution stats (average + minmax + percentiles)
#[derive(Traversable)]
pub struct Distribution<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub min_max_average: MinMaxAverage<I, T, M>,
    #[traversable(flatten)]
    pub percentiles: Percentiles<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Distribution<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            min_max_average: MinMaxAverage::forced_import(db, name, version)?,
            percentiles: Percentiles::forced_import(db, name, version)?,
        })
    }

    /// Compute distribution stats, skipping first N items from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub(crate) fn compute_with_skip<A>(
        &mut self,
        max_from: I,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<I, A>,
        count_indexes: &impl ReadableVec<I, brk_types::StoredU64>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        crate::internal::compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None, // first
            None, // last
            Some(&mut self.min_max_average.minmax.min.0),
            Some(&mut self.min_max_average.minmax.max.0),
            Some(&mut self.min_max_average.average.0),
            None, // sum
            None, // cumulative
            Some(&mut self.percentiles.median.0),
            Some(&mut self.percentiles.pct10.0),
            Some(&mut self.percentiles.pct25.0),
            Some(&mut self.percentiles.pct75.0),
            Some(&mut self.percentiles.pct90.0),
        )
    }

    // Boxed accessors
    pub(crate) fn boxed_average(&self) -> ReadableBoxedVec<I, T> {
        self.min_max_average.boxed_average()
    }

    pub(crate) fn boxed_min(&self) -> ReadableBoxedVec<I, T> {
        self.min_max_average.boxed_min()
    }

    pub(crate) fn boxed_max(&self) -> ReadableBoxedVec<I, T> {
        self.min_max_average.boxed_max()
    }

    pub fn read_only_clone(&self) -> Distribution<I, T, Ro> {
        Distribution {
            min_max_average: self.min_max_average.read_only_clone(),
            percentiles: self.percentiles.read_only_clone(),
        }
    }
}

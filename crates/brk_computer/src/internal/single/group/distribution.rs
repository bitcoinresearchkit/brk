use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{AnyVec, Database, Exit, IterableBoxedVec, IterableVec, VecIndex, VecValue, Version};

use crate::internal::{AverageVec, ComputedVecValue, MaxVec, MinVec};

use super::{MinMaxAverage, Percentiles};

/// Distribution stats (average + minmax + percentiles)
#[derive(Clone, Traversable)]
pub struct Distribution<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub min_max_average: MinMaxAverage<I, T>,
    #[traversable(flatten)]
    pub percentiles: Percentiles<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Distribution<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            min_max_average: MinMaxAverage::forced_import(db, name, version)?,
            percentiles: Percentiles::forced_import(db, name, version)?,
        })
    }

    /// Compute distribution stats from source data.
    ///
    /// This computes: average, min, max, percentiles (pct10, pct25, median, pct75, pct90)
    pub fn compute<A>(
        &mut self,
        max_from: I,
        source: &impl IterableVec<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, brk_types::StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        self.compute_with_skip(max_from, source, first_indexes, count_indexes, exit, 0)
    }

    /// Compute distribution stats, skipping first N items from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub fn compute_with_skip<A>(
        &mut self,
        max_from: I,
        source: &impl IterableVec<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, brk_types::StoredU64>,
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

    pub fn len(&self) -> usize {
        self.min_max_average
            .len()
            .min(self.percentiles.pct10.0.len())
            .min(self.percentiles.pct25.0.len())
            .min(self.percentiles.median.0.len())
            .min(self.percentiles.pct75.0.len())
            .min(self.percentiles.pct90.0.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }

    // Accessors
    pub fn average(&self) -> &AverageVec<I, T> {
        &self.min_max_average.average
    }

    pub fn min(&self) -> &MinVec<I, T> {
        self.min_max_average.min()
    }

    pub fn max(&self) -> &MaxVec<I, T> {
        self.min_max_average.max()
    }

    // Boxed accessors
    pub fn boxed_average(&self) -> IterableBoxedVec<I, T> {
        self.min_max_average.boxed_average()
    }

    pub fn boxed_min(&self) -> IterableBoxedVec<I, T> {
        self.min_max_average.boxed_min()
    }

    pub fn boxed_max(&self) -> IterableBoxedVec<I, T> {
        self.min_max_average.boxed_max()
    }
}

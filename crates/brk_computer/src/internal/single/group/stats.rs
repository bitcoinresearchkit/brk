use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec, VecIndex, VecValue, Version};

use crate::internal::{AverageVec, ComputedVecValue, CumulativeVec, MaxVec, MinVec, SumVec};

use super::{MinMaxAverage, SumCum};

/// Sum + Cumulative + Average + Min + Max. Like `Full` but without percentiles.
#[derive(Clone, Traversable)]
pub struct Stats<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub sum_cum: SumCum<I, T>,
    #[traversable(flatten)]
    pub min_max_average: MinMaxAverage<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Stats<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            sum_cum: SumCum::forced_import(db, name, version)?,
            min_max_average: MinMaxAverage::forced_import(db, name, version)?,
        })
    }

    /// Compute sum, cumulative, average, and minmax from source data.
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
        crate::internal::compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            0, // min_skip_count
            None, // first
            None, // last
            Some(&mut self.min_max_average.minmax.min.0),
            Some(&mut self.min_max_average.minmax.max.0),
            Some(&mut self.min_max_average.average.0),
            Some(&mut self.sum_cum.sum.0),
            Some(&mut self.sum_cum.cumulative.0),
            None, // median
            None, // pct10
            None, // pct25
            None, // pct75
            None, // pct90
        )
    }

    pub fn len(&self) -> usize {
        self.sum_cum.len().min(self.min_max_average.len())
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

    pub fn sum(&self) -> &SumVec<I, T> {
        &self.sum_cum.sum
    }

    pub fn cumulative(&self) -> &CumulativeVec<I, T> {
        &self.sum_cum.cumulative
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

    pub fn boxed_sum(&self) -> IterableBoxedVec<I, T> {
        self.sum_cum.sum.0.boxed_clone()
    }

    pub fn boxed_cumulative(&self) -> IterableBoxedVec<I, T> {
        self.sum_cum.cumulative.0.boxed_clone()
    }
}

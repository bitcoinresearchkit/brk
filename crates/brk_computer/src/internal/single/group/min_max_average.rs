use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{AnyVec, Database, Exit, IterableVec, VecIndex, VecValue, Version};

use crate::internal::{AverageVec, ComputedVecValue};

use super::MinMax;

/// Average + MinMax (for TxIndex dateindex aggregation - no percentiles)
#[derive(Clone, Traversable)]
pub struct MinMaxAverage<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    pub average: AverageVec<I, T>,
    #[traversable(flatten)]
    pub minmax: MinMax<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MinMaxAverage<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            average: AverageVec::forced_import(db, name, version)?,
            minmax: MinMax::forced_import(db, name, version)?,
        })
    }

    /// Compute average and minmax from source data.
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
            None, // first
            None, // last
            Some(&mut self.minmax.min.0),
            Some(&mut self.minmax.max.0),
            Some(&mut self.average.0),
            None, // sum
            None, // cumulative
            None, // median
            None, // pct10
            None, // pct25
            None, // pct75
            None, // pct90
        )
    }

    /// Compute from aligned source (for coarser time periods).
    pub fn compute_from_aligned<A>(
        &mut self,
        max_from: I,
        source: &MinMaxAverage<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, brk_types::StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        crate::internal::compute_aggregations_from_aligned(
            max_from,
            first_indexes,
            count_indexes,
            exit,
            // Source vecs
            None, // first
            None, // last
            Some(&source.minmax.min.0),
            Some(&source.minmax.max.0),
            Some(&source.average.0),
            None, // sum
            // Target vecs
            None, // first
            None, // last
            Some(&mut self.minmax.min.0),
            Some(&mut self.minmax.max.0),
            Some(&mut self.average.0),
            None, // sum
            None, // cumulative
        )
    }

    pub fn len(&self) -> usize {
        self.average
            .0
            .len()
            .min(self.minmax.min.0.len())
            .min(self.minmax.max.0.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }
}

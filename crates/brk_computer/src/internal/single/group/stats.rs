use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{AnyVec, Database, Exit, IterableVec, VecIndex, VecValue, Version};

use crate::internal::{AverageVec, ComputedVecValue};

use super::{MinMax, SumCum};

/// Sum + Cumulative + Average + Min + Max. Like `Full` but without percentiles.
#[derive(Clone, Traversable)]
pub struct Stats<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub sum_cum: SumCum<I, T>,
    #[traversable(flatten)]
    pub average: AverageVec<I, T>,
    #[traversable(flatten)]
    pub minmax: MinMax<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Stats<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            sum_cum: SumCum::forced_import(db, name, version)?,
            average: AverageVec::forced_import(db, name, version)?,
            minmax: MinMax::forced_import(db, name, version)?,
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
            None, // first
            None, // last
            Some(&mut self.minmax.min.0),
            Some(&mut self.minmax.max.0),
            Some(&mut self.average.0),
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
        self.sum_cum
            .len()
            .min(self.average.0.len())
            .min(self.minmax.min.0.len())
            .min(self.minmax.max.0.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }
}

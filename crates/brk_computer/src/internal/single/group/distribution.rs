use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{AnyVec, Database, Exit, IterableVec, VecIndex, VecValue, Version};

use crate::internal::{AverageVec, ComputedVecValue};

use super::{MinMax, Percentiles};

/// Distribution stats (average + minmax + percentiles)
#[derive(Clone, Traversable)]
pub struct Distribution<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub average: AverageVec<I, T>,
    #[traversable(flatten)]
    pub minmax: MinMax<I, T>,
    pub percentiles: Percentiles<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Distribution<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            average: AverageVec::forced_import(db, name, version)?,
            minmax: MinMax::forced_import(db, name, version)?,
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
            Some(&mut self.percentiles.median.0),
            Some(&mut self.percentiles.pct10.0),
            Some(&mut self.percentiles.pct25.0),
            Some(&mut self.percentiles.pct75.0),
            Some(&mut self.percentiles.pct90.0),
        )
    }

    pub fn len(&self) -> usize {
        self.average
            .0
            .len()
            .min(self.minmax.min.0.len())
            .min(self.minmax.max.0.len())
            .min(self.percentiles.pct10.0.len())
            .min(self.percentiles.pct25.0.len())
            .min(self.percentiles.median.0.len())
            .min(self.percentiles.pct75.0.len())
            .min(self.percentiles.pct90.0.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }
}

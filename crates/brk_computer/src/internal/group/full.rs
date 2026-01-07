use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableVec, VecIndex, VecValue, Version};

use crate::internal::ComputedVecValue;

use super::{Distribution, SumCum};

/// Full stats aggregate: distribution + sum_cum
/// Matches the common full_stats() pattern: average + minmax + percentiles + sum + cumulative
#[derive(Clone, Traversable)]
pub struct Full<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    pub distribution: Distribution<I, T>,
    pub sum_cum: SumCum<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Full<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            distribution: Distribution::forced_import(db, name, version)?,
            sum_cum: SumCum::forced_import(db, name, version)?,
        })
    }

    /// Compute all stats from source data.
    ///
    /// This computes: average, min, max, percentiles (pct10, pct25, median, pct75, pct90), sum, cumulative
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
            Some(&mut self.distribution.minmax.min.0),
            Some(&mut self.distribution.minmax.max.0),
            Some(&mut self.distribution.average.0),
            Some(&mut self.sum_cum.sum.0),
            Some(&mut self.sum_cum.cumulative.0),
            Some(&mut self.distribution.percentiles.median.0),
            Some(&mut self.distribution.percentiles.pct10.0),
            Some(&mut self.distribution.percentiles.pct25.0),
            Some(&mut self.distribution.percentiles.pct75.0),
            Some(&mut self.distribution.percentiles.pct90.0),
        )
    }

    pub fn len(&self) -> usize {
        self.distribution.len().min(self.sum_cum.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }

    /// Compute from aligned source (for coarser time periods like week from dateindex).
    ///
    /// NOTE: Percentiles cannot be derived from finer percentiles - they are skipped.
    pub fn compute_from_aligned<A>(
        &mut self,
        max_from: I,
        source: &Full<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, brk_types::StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        // Note: Percentiles cannot be derived from finer percentiles, so we skip them
        crate::internal::compute_aggregations_from_aligned(
            max_from,
            first_indexes,
            count_indexes,
            exit,
            // Source vecs
            None, // first not in Full
            None, // last not in Full
            Some(&source.distribution.minmax.min.0),
            Some(&source.distribution.minmax.max.0),
            Some(&source.distribution.average.0),
            Some(&source.sum_cum.sum.0),
            // Target vecs
            None, // first
            None, // last
            Some(&mut self.distribution.minmax.min.0),
            Some(&mut self.distribution.minmax.max.0),
            Some(&mut self.distribution.average.0),
            Some(&mut self.sum_cum.sum.0),
            Some(&mut self.sum_cum.cumulative.0),
        )
    }
}

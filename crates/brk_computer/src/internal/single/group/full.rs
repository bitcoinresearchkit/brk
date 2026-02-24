use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{
    Database, Exit, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Ro, Rw, StorageMode,
    VecIndex, VecValue, Version,
};

use crate::internal::ComputedVecValue;

use super::{Distribution, SumCum};

/// Full stats aggregate: distribution + sum_cum
/// Matches the common full_stats() pattern: average + minmax + percentiles + sum + cumulative
#[derive(Traversable)]
pub struct Full<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub distribution: Distribution<I, T, M>,
    #[traversable(flatten)]
    pub sum_cum: SumCum<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Full<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            distribution: Distribution::forced_import(db, name, version)?,
            sum_cum: SumCum::forced_import(db, name, version)?,
        })
    }

    /// Compute all stats, skipping first N items from all calculations.
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
            Some(&mut self.distribution.min_max_average.minmax.min.0),
            Some(&mut self.distribution.min_max_average.minmax.max.0),
            Some(&mut self.distribution.min_max_average.average.0),
            Some(&mut self.sum_cum.sum.0),
            Some(&mut self.sum_cum.cumulative.0),
            Some(&mut self.distribution.percentiles.median.0),
            Some(&mut self.distribution.percentiles.pct10.0),
            Some(&mut self.distribution.percentiles.pct25.0),
            Some(&mut self.distribution.percentiles.pct75.0),
            Some(&mut self.distribution.percentiles.pct90.0),
        )
    }

    pub(crate) fn boxed_sum(&self) -> ReadableBoxedVec<I, T> {
        self.sum_cum.sum.0.read_only_boxed_clone()
    }

    pub fn read_only_clone(&self) -> Full<I, T, Ro> {
        Full {
            distribution: self.distribution.read_only_clone(),
            sum_cum: self.sum_cum.read_only_clone(),
        }
    }
}

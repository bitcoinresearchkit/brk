use brk_error::Result;
use schemars::JsonSchema;
use vecdb::{
    Database, Exit, ReadableVec, Ro, Rw, VecIndex, VecValue, Version,
};

use crate::internal::{
    AverageVec, ComputedVecValue, DistributionStats, MaxVec, MedianVec, MinVec, Pct10Vec,
    Pct25Vec, Pct75Vec, Pct90Vec,
};

/// Distribution stats (average + min + max + percentiles) — concrete vec type alias.
pub type Distribution<I, T, M = Rw> = DistributionStats<
    AverageVec<I, T, M>,
    MinVec<I, T, M>,
    MaxVec<I, T, M>,
    Pct10Vec<I, T, M>,
    Pct25Vec<I, T, M>,
    MedianVec<I, T, M>,
    Pct75Vec<I, T, M>,
    Pct90Vec<I, T, M>,
>;

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Distribution<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            average: AverageVec::forced_import(db, name, version)?,
            min: MinVec::forced_import(db, name, version)?,
            max: MaxVec::forced_import(db, name, version)?,
            pct10: Pct10Vec::forced_import(db, name, version)?,
            pct25: Pct25Vec::forced_import(db, name, version)?,
            median: MedianVec::forced_import(db, name, version)?,
            pct75: Pct75Vec::forced_import(db, name, version)?,
            pct90: Pct90Vec::forced_import(db, name, version)?,
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
            Some(&mut self.min.0),
            Some(&mut self.max.0),
            Some(&mut self.average.0),
            None, // sum
            None, // cumulative
            Some(&mut self.median.0),
            Some(&mut self.pct10.0),
            Some(&mut self.pct25.0),
            Some(&mut self.pct75.0),
            Some(&mut self.pct90.0),
        )
    }

    /// Compute distribution stats from all items in a rolling window of groups.
    ///
    /// For each index `i`, reads all source items from groups `window_starts[i]..=i`
    /// and computes distribution stats across the entire window.
    pub(crate) fn compute_from_window<A>(
        &mut self,
        max_from: I,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<I, A>,
        count_indexes: &impl ReadableVec<I, brk_types::StoredU64>,
        window_starts: &impl ReadableVec<I, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        crate::internal::compute_aggregations_windowed(
            max_from,
            source,
            first_indexes,
            count_indexes,
            window_starts,
            exit,
            &mut self.min.0,
            &mut self.max.0,
            &mut self.average.0,
            &mut self.median.0,
            &mut self.pct10.0,
            &mut self.pct25.0,
            &mut self.pct75.0,
            &mut self.pct90.0,
        )
    }

    pub fn read_only_clone(&self) -> Distribution<I, T, Ro> {
        DistributionStats {
            average: self.average.read_only_clone(),
            min: self.min.read_only_clone(),
            max: self.max.read_only_clone(),
            pct10: self.pct10.read_only_clone(),
            pct25: self.pct25.read_only_clone(),
            median: self.median.read_only_clone(),
            pct75: self.pct75.read_only_clone(),
            pct90: self.pct90.read_only_clone(),
        }
    }
}

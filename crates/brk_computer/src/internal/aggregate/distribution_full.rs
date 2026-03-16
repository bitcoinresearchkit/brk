use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableVec, Ro, Rw, StorageMode, StoredVec,
    VecIndex, VecValue, Version,
};

use crate::internal::{ComputedVecValue, algo::compute_aggregations};

use super::Distribution;

/// Full stats aggregate: sum + cumulative + distribution
#[derive(Traversable)]
pub struct DistributionFull<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    pub sum: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub cumulative: M::Stored<EagerVec<PcoVec<I, T>>>,
    #[traversable(flatten)]
    pub distribution: Distribution<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> DistributionFull<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            distribution: Distribution::forced_import(db, name, version)?,
            sum: EagerVec::forced_import(db, &format!("{name}_sum"), version)?,
            cumulative: EagerVec::forced_import(db, &format!("{name}_cumulative"), version)?,
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
        compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None, // first
            None, // last
            Some(&mut self.distribution.min),
            Some(&mut self.distribution.max),
            Some(&mut self.distribution.average),
            Some(&mut self.sum),
            Some(&mut self.cumulative),
            Some(&mut self.distribution.median),
            Some(&mut self.distribution.pct10),
            Some(&mut self.distribution.pct25),
            Some(&mut self.distribution.pct75),
            Some(&mut self.distribution.pct90),
        )
    }

    pub fn read_only_clone(&self) -> DistributionFull<I, T, Ro> {
        DistributionFull {
            distribution: self.distribution.read_only_clone(),
            sum: StoredVec::read_only_clone(&self.sum),
            cumulative: StoredVec::read_only_clone(&self.cumulative),
        }
    }
}

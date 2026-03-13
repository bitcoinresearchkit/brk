use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableVec, Ro, Rw, StorageMode, StoredVec,
    VecIndex, VecValue, Version,
};

use crate::internal::{
    ComputedVecValue, DistributionStats,
    algo::{compute_aggregations, compute_aggregations_nblock_window},
};

#[derive(Traversable)]
pub struct Distribution<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    pub average: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub min: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub max: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub pct10: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub pct25: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub median: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub pct75: M::Stored<EagerVec<PcoVec<I, T>>>,
    pub pct90: M::Stored<EagerVec<PcoVec<I, T>>>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Distribution<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let s = DistributionStats::<()>::SUFFIXES;
        Ok(Self {
            average: EagerVec::forced_import(db, &format!("{name}_{}", s[0]), version)?,
            min: EagerVec::forced_import(db, &format!("{name}_{}", s[1]), version)?,
            max: EagerVec::forced_import(db, &format!("{name}_{}", s[2]), version)?,
            pct10: EagerVec::forced_import(db, &format!("{name}_{}", s[3]), version)?,
            pct25: EagerVec::forced_import(db, &format!("{name}_{}", s[4]), version)?,
            median: EagerVec::forced_import(db, &format!("{name}_{}", s[5]), version)?,
            pct75: EagerVec::forced_import(db, &format!("{name}_{}", s[6]), version)?,
            pct90: EagerVec::forced_import(db, &format!("{name}_{}", s[7]), version)?,
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
        compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None, // first
            None, // last
            Some(&mut self.min),
            Some(&mut self.max),
            Some(&mut self.average),
            None, // sum
            None, // cumulative
            Some(&mut self.median),
            Some(&mut self.pct10),
            Some(&mut self.pct25),
            Some(&mut self.pct75),
            Some(&mut self.pct90),
        )
    }

    /// Compute distribution stats from a fixed n-block rolling window.
    ///
    /// For each index `i`, aggregates all source items from blocks `max(0, i - n_blocks + 1)..=i`.
    pub(crate) fn compute_from_nblocks<A>(
        &mut self,
        max_from: I,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<I, A>,
        count_indexes: &impl ReadableVec<I, brk_types::StoredU64>,
        n_blocks: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        compute_aggregations_nblock_window(
            max_from,
            source,
            first_indexes,
            count_indexes,
            n_blocks,
            exit,
            &mut self.min,
            &mut self.max,
            &mut self.average,
            &mut self.median,
            &mut self.pct10,
            &mut self.pct25,
            &mut self.pct75,
            &mut self.pct90,
        )
    }

    pub fn read_only_clone(&self) -> Distribution<I, T, Ro> {
        Distribution {
            average: StoredVec::read_only_clone(&self.average),
            min: StoredVec::read_only_clone(&self.min),
            max: StoredVec::read_only_clone(&self.max),
            pct10: StoredVec::read_only_clone(&self.pct10),
            pct25: StoredVec::read_only_clone(&self.pct25),
            median: StoredVec::read_only_clone(&self.median),
            pct75: StoredVec::read_only_clone(&self.pct75),
            pct90: StoredVec::read_only_clone(&self.pct90),
        }
    }
}

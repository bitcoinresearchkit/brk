use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{
    AnyVec, Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec, VecIndex,
    VecValue, Version,
};

use crate::internal::{ComputedVecValue, CumulativeVec, SumVec};

/// Sum + Cumulative (12% of usage)
#[derive(Clone, Traversable)]
pub struct SumCum<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub sum: SumVec<I, T>,
    #[traversable(flatten)]
    pub cumulative: CumulativeVec<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> SumCum<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            sum: SumVec::forced_import(db, name, version)?,
            cumulative: CumulativeVec::forced_import(db, name, version)?,
        })
    }

    /// Import with raw sum name (no _sum suffix) for cases where sum should merge with base.
    pub fn forced_import_sum_raw(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            sum: SumVec::forced_import_raw(db, name, version)?,
            cumulative: CumulativeVec::forced_import(db, name, version)?,
        })
    }

    /// Compute sum and cumulative from source data.
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
            0,    // min_skip_count
            None, // first
            None, // last
            None, // min
            None, // max
            None, // average
            Some(&mut self.sum.0),
            Some(&mut self.cumulative.0),
            None, // median
            None, // pct10
            None, // pct25
            None, // pct75
            None, // pct90
        )
    }

    pub fn len(&self) -> usize {
        self.sum.0.len().min(self.cumulative.0.len())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(self.len()))
    }

    /// Compute from aligned source (for coarser time periods like week from dateindex).
    pub fn compute_from_aligned<A>(
        &mut self,
        max_from: I,
        source: &SumCum<A, T>,
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
            None, // min
            None, // max
            None, // average
            Some(&source.sum.0),
            // Target vecs
            None, // first
            None, // last
            None, // min
            None, // max
            None, // average
            Some(&mut self.sum.0),
            Some(&mut self.cumulative.0),
        )
    }

    // Boxed accessors
    pub fn boxed_sum(&self) -> IterableBoxedVec<I, T> {
        self.sum.0.boxed_clone()
    }

    pub fn boxed_cumulative(&self) -> IterableBoxedVec<I, T> {
        self.cumulative.0.boxed_clone()
    }
}

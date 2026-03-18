use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    CheckedSub, Database, Exit, ReadableVec, Rw, StorageMode,
    VecIndex, VecValue, Version,
};

use crate::{
    indexes,
    internal::{
        ComputedVecValue, DistributionStats, NumericValue, PerBlock,
        algo::{compute_aggregations, compute_aggregations_nblock_window},
    },
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PerBlockDistribution<T: ComputedVecValue + PartialOrd + JsonSchema, M: StorageMode = Rw>(
    pub DistributionStats<PerBlock<T, M>>,
);

impl<T: NumericValue + JsonSchema> PerBlockDistribution<T> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(DistributionStats::try_from_fn(|suffix| {
            PerBlock::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }

    pub(crate) fn compute_with_skip<A>(
        &mut self,
        max_from: Height,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<Height, A>,
        count_indexes: &impl ReadableVec<Height, brk_types::StoredU64>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        let s = &mut self.0;
        compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None,
            None,
            Some(&mut s.min.height),
            Some(&mut s.max.height),
            Some(&mut s.average.height),
            None,
            None,
            Some(&mut s.median.height),
            Some(&mut s.pct10.height),
            Some(&mut s.pct25.height),
            Some(&mut s.pct75.height),
            Some(&mut s.pct90.height),
        )
    }

    pub(crate) fn compute_from_nblocks<A>(
        &mut self,
        max_from: Height,
        source: &(impl ReadableVec<A, T> + Sized),
        first_indexes: &impl ReadableVec<Height, A>,
        count_indexes: &impl ReadableVec<Height, brk_types::StoredU64>,
        n_blocks: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        T: CheckedSub,
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        let s = &mut self.0;
        compute_aggregations_nblock_window(
            max_from,
            source,
            first_indexes,
            count_indexes,
            n_blocks,
            exit,
            &mut s.min.height,
            &mut s.max.height,
            &mut s.average.height,
            &mut s.median.height,
            &mut s.pct10.height,
            &mut s.pct25.height,
            &mut s.pct75.height,
            &mut s.pct90.height,
        )
    }
}

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;
use vecdb::{
    Database, Exit, ReadableVec, Rw, StorageMode,
    VecIndex, VecValue, Version,
};

use crate::{
    indexes,
    internal::{ComputedVecValue, NumericValue, PerBlock, algo::compute_aggregations},
};

use super::PerBlockDistribution;

#[derive(Traversable)]
pub struct PerBlockDistributionFull<T: ComputedVecValue + PartialOrd + JsonSchema, M: StorageMode = Rw> {
    pub sum: PerBlock<T, M>,
    pub cumulative: PerBlock<T, M>,
    #[traversable(flatten)]
    pub distribution: PerBlockDistribution<T, M>,
}

impl<T: NumericValue + JsonSchema> PerBlockDistributionFull<T> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            distribution: PerBlockDistribution::forced_import(db, name, version, indexes)?,
            sum: PerBlock::forced_import(db, &format!("{name}_sum"), version, indexes)?,
            cumulative: PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?,
        })
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
        let d = &mut self.distribution.0;
        compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None,
            None,
            Some(&mut d.min.height),
            Some(&mut d.max.height),
            Some(&mut d.average.height),
            Some(&mut self.sum.height),
            Some(&mut self.cumulative.height),
            Some(&mut d.median.height),
            Some(&mut d.pct10.height),
            Some(&mut d.pct25.height),
            Some(&mut d.pct75.height),
            Some(&mut d.pct90.height),
        )
    }
}

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode, VecIndex, VecValue, Version};

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, NumericValue, PerBlock, RollingComplete, WindowStarts,
        algo::compute_aggregations,
    },
};

#[derive(Traversable)]
pub struct PerBlockAggregated<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub sum: PerBlock<T, M>,
    pub cumulative: PerBlock<T, M>,
    pub rolling: RollingComplete<T, M>,
}

impl<T> PerBlockAggregated<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let sum = PerBlock::forced_import(db, &format!("{name}_sum"), version, indexes)?;
        let cumulative =
            PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let rolling = RollingComplete::forced_import(
            db,
            name,
            version,
            indexes,
            &cumulative.height,
            cached_starts,
        )?;

        Ok(Self {
            sum,
            cumulative,
            rolling,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute<A>(
        &mut self,
        max_from: Height,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<Height, A>,
        count_indexes: &impl ReadableVec<Height, brk_types::StoredU64>,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        compute_aggregations(
            max_from,
            source,
            first_indexes,
            count_indexes,
            exit,
            skip_count,
            None,
            None,
            None,
            None,
            None,
            Some(&mut self.sum.height),
            Some(&mut self.cumulative.height),
            None,
            None,
            None,
            None,
            None,
        )?;
        self.rolling
            .compute(max_from, windows, &self.sum.height, exit)?;
        Ok(())
    }
}

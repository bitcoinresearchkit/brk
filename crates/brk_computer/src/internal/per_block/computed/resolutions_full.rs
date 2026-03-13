//! ResolutionsFull - LazyAggVec index views + cumulative (from height) + RollingFull.
//!
//! For metrics derived from indexer sources (no stored height vec).
//! Cumulative gets its own ComputedPerBlock so it has LazyAggVec index views too.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlock, NumericValue, RollingFull, WindowStarts},
};

#[derive(Traversable)]
pub struct ResolutionsFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub cumulative: ComputedPerBlock<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingFull<T, M>,
}

impl<T> ResolutionsFull<T>
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
        let cumulative =
            ComputedPerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let rolling = RollingFull::forced_import(
            db,
            name,
            version,
            indexes,
            &cumulative.height,
            cached_starts,
        )?;

        Ok(Self {
            cumulative,
            rolling,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        height_source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, height_source, exit)?;
        self.rolling
            .compute(max_from, windows, height_source, exit)?;
        Ok(())
    }
}

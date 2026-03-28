//! PerBlockRolling - LazyAggVec index views + cumulative (from height) + RollingComplete.
//!
//! For metrics derived from indexer sources (no stored height vec).
//! Cumulative gets its own PerBlock so it has LazyAggVec index views too.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, NumericValue, PerBlock, RollingComplete, WindowStarts},
};

#[derive(Traversable)]
pub struct PerBlockRolling<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub cumulative: PerBlock<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingComplete<T, M>,
}

impl<T> PerBlockRolling<T>
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

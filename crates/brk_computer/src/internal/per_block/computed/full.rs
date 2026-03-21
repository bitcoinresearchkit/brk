//! PerBlockFull - base EagerVec + cumulative PerBlock + RollingComplete.
//!
//! For metrics with stored per-block data, cumulative sums, and rolling windows.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, NumericValue, PerBlock, RollingComplete, WindowStarts},
};

#[derive(Traversable)]
pub struct PerBlockFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub block: M::Stored<EagerVec<PcoVec<Height, T>>>,
    pub cumulative: PerBlock<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingComplete<T, M>,
}

impl<T> PerBlockFull<T>
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
        let block = EagerVec::forced_import(db, name, version)?;
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
            block,
            cumulative,
            rolling,
        })
    }

    /// Compute base data via closure, then cumulative + rolling distribution.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_base: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
    {
        compute_base(&mut self.block)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.block, exit)?;
        self.rolling.compute(max_from, windows, &self.block, exit)?;
        Ok(())
    }
}

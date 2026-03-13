//! PerBlockFull - base PerBlock + cumulative PerBlock + RollingComplete.
//!
//! For metrics with stored per-block data, cumulative sums, and rolling windows.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlock, NumericValue, RollingComplete, WindowStarts},
};

#[derive(Traversable)]
pub struct PerBlockFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub base: PerBlock<T, M>,
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
        let base = PerBlock::forced_import(db, name, version, indexes)?;
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
            base,
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
        compute_base(&mut self.base.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.base.height, exit)?;
        self.rolling
            .compute(max_from, windows, &self.base.height, exit)?;
        Ok(())
    }
}

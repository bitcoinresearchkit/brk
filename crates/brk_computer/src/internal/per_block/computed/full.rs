//! ComputedPerBlockFull - raw ComputedPerBlock + cumulative ComputedPerBlock + RollingFull.
//!
//! For metrics with stored per-block data, cumulative sums, and rolling windows.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlock, NumericValue, RollingFull, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedPerBlockFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub cumulative: ComputedPerBlock<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingFull<T, M>,
}

impl<T> ComputedPerBlockFull<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let raw = ComputedPerBlock::forced_import(db, name, version, indexes)?;
        let cumulative =
            ComputedPerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let rolling = RollingFull::forced_import(db, name, version, indexes)?;

        Ok(Self {
            raw,
            cumulative,
            rolling,
        })
    }

    /// Compute raw data via closure, then cumulative + rolling.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_raw: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        compute_raw(&mut self.raw.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.raw.height, exit)?;
        self.rolling
            .compute(max_from, windows, &self.raw.height, exit)?;
        Ok(())
    }
}

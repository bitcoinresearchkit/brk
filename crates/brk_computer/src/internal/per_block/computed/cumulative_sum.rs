//! ComputedPerBlockCumulativeSum - raw ComputedPerBlock + cumulative ComputedPerBlock + RollingWindows (sum).
//!
//! Like ComputedPerBlockFull but with rolling sum only (no distribution).
//! Used for count metrics where distribution stats aren't meaningful.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlock, NumericValue, RollingWindows, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedPerBlockCumulativeSum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub cumulative: ComputedPerBlock<T, M>,
    pub sum: RollingWindows<T, M>,
}

impl<T> ComputedPerBlockCumulativeSum<T>
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
        let sum = RollingWindows::forced_import(db, name, version, indexes)?;

        Ok(Self {
            raw,
            cumulative,
            sum,
        })
    }

    /// Compute raw data via closure, then cumulative + rolling sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_raw: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        compute_raw(&mut self.raw.height)?;
        self.compute_rest(max_from, windows, exit)
    }

    /// Compute cumulative + rolling sum from already-populated raw data.
    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.raw.height, exit)?;
        self.sum
            .compute_rolling_sum(max_from, windows, &self.raw.height, exit)?;
        Ok(())
    }
}

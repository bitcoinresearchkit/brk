//! ComputedPerBlockCumulativeSum - stored height + LazyAggVec + cumulative (from height) + RollingWindows (sum).
//!
//! Like ComputedPerBlockFull but with rolling sum only (no distribution).
//! Used for count metrics where distribution stats aren't meaningful.
//! Cumulative gets its own ComputedPerBlock so it has LazyAggVec index views too.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlock, NumericValue, RollingWindows, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedPerBlockCumulativeSum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
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
        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;
        let cumulative =
            ComputedPerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let rolling = RollingWindows::forced_import(db, name, version, indexes)?;

        Ok(Self {
            height,
            cumulative,
            sum: rolling,
        })
    }

    /// Compute height data via closure, then cumulative + rolling sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        compute_height(&mut self.height)?;
        self.compute_rest(max_from, windows, exit)
    }

    /// Compute cumulative + rolling sum from already-populated height data.
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
            .compute_cumulative(max_from, &self.height, exit)?;
        self.sum
            .compute_rolling_sum(max_from, windows, &self.height, exit)?;
        Ok(())
    }
}

//! ComputedPerBlockCumulative - raw ComputedPerBlock + cumulative ComputedPerBlock.
//!
//! Like ComputedPerBlockCumulativeSum but without RollingWindows.
//! Used for distribution metrics where rolling is optional per cohort.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlock, NumericValue},
};

#[derive(Traversable)]
pub struct ComputedPerBlockCumulative<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub cumulative: ComputedPerBlock<T, M>,
}

impl<T> ComputedPerBlockCumulative<T>
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

        Ok(Self { raw, cumulative })
    }

    /// Compute raw data via closure, then cumulative only (no rolling).
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        exit: &Exit,
        compute_raw: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default,
    {
        compute_raw(&mut self.raw.height)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-filled raw vec.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        T: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.raw.height, exit)?;
        Ok(())
    }
}

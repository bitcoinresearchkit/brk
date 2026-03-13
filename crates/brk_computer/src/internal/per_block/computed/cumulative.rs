//! ComputedPerBlockCumulative - raw ComputedPerBlock + cumulative ComputedPerBlock.
//!
//! Like ComputedPerBlockCumulativeWithSums but without RollingWindows.
//! Used for distribution metrics where rolling is optional per cohort.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, Rw, StorageMode};

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

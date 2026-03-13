//! PerBlockCumulative - base PerBlock + cumulative PerBlock.
//!
//! Like PerBlockCumulativeWithSums but without RollingWindows.
//! Used for distribution metrics where rolling is optional per cohort.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PerBlock, NumericValue},
};

#[derive(Traversable)]
pub struct PerBlockCumulative<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub base: PerBlock<T, M>,
    pub cumulative: PerBlock<T, M>,
}

impl<T> PerBlockCumulative<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let base = PerBlock::forced_import(db, name, version, indexes)?;
        let cumulative =
            PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;

        Ok(Self { base, cumulative })
    }

    /// Compute cumulative from already-filled base vec.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        T: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.base.height, exit)?;
        Ok(())
    }
}

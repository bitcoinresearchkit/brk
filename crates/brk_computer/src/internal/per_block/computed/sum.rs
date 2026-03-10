//! ComputedPerBlockSum - raw ComputedPerBlock + RollingWindows (sum only).
//!
//! Like ComputedPerBlockCumulativeSum but without the cumulative vec.

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
pub struct ComputedPerBlockSum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub sum: RollingWindows<T, M>,
}

impl<T> ComputedPerBlockSum<T>
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
        let sum = RollingWindows::forced_import(db, &format!("{name}_sum"), version, indexes)?;

        Ok(Self { raw, sum })
    }

    /// Compute raw data via closure, then rolling sum.
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
        self.sum
            .compute_rolling_sum(max_from, windows, &self.raw.height, exit)?;
        Ok(())
    }
}

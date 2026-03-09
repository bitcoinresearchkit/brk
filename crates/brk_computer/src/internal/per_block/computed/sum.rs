//! ComputedPerBlockSum - stored height + RollingWindows (sum only).
//!
//! Like ComputedPerBlockCumulativeSum but without the cumulative vec.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{NumericValue, RollingWindows, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedPerBlockSum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
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
        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;
        let sum = RollingWindows::forced_import(db, &format!("{name}_sum"), version, indexes)?;

        Ok(Self { height, sum })
    }

    /// Compute height data via closure, then rolling sum.
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
        self.sum
            .compute_rolling_sum(max_from, windows, &self.height, exit)?;
        Ok(())
    }
}

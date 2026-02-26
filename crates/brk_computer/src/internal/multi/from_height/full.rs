//! ComputedFromHeightFull - Full (distribution + sum + cumulative) + RollingFull.
//!
//! For metrics aggregated per-block from finer-grained sources (e.g., per-tx data),
//! where we want full per-block stats plus rolling window stats.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{Full, NumericValue, RollingFull, WindowStarts},
};

#[derive(Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[traversable(flatten)]
    pub height: Full<Height, T, M>,
    #[traversable(flatten)]
    pub rolling: RollingFull<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightFull<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = Full::forced_import(db, name, v)?;
        let rolling = RollingFull::forced_import(db, name, v, indexes)?;

        Ok(Self { height, rolling })
    }

    /// Compute Full stats via closure, then rolling windows from the per-block sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_full: impl FnOnce(&mut Full<Height, T>) -> Result<()>,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        compute_full(&mut self.height)?;
        self.rolling.compute(
            max_from,
            windows,
            self.height.sum_cumulative.sum.inner(),
            exit,
        )?;
        Ok(())
    }
}

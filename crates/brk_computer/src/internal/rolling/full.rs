//! RollingFull - Sum + Distribution per rolling window.
//!
//! 36 stored height vecs per metric (4 sum + 32 distribution), each with 17 index views.

use std::ops::SubAssign;

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedVecValue, NumericValue, RollingDistribution, RollingWindows, WindowStarts},
};

/// Sum (4 windows) + Distribution (8 stats × 4 windows) = 36 stored height vecs.
#[derive(Traversable)]
pub struct RollingFull<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub sum: RollingWindows<T, M>,
    #[traversable(flatten)]
    pub distribution: RollingDistribution<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> RollingFull<T>
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
        Ok(Self {
            sum: RollingWindows::forced_import(db, &format!("{name}_sum"), v, indexes)?,
            distribution: RollingDistribution::forced_import(db, name, v, indexes)?,
        })
    }

    /// Compute rolling sum + all 8 distribution stats across all 4 windows.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        self.sum
            .compute_rolling_sum(max_from, windows, source, exit)?;
        self.distribution
            .compute_distribution(max_from, windows, source, exit)?;
        Ok(())
    }
}

//! PerBlockAggregated - DistributionFull (distribution + sum + cumulative) + RollingComplete.
//!
//! For metrics aggregated per-block from finer-grained sources (e.g., per-tx data),
//! where we want full per-block stats plus rolling window stats.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, DistributionFull, NumericValue, RollingComplete, WindowStarts},
};

#[derive(Traversable)]
pub struct PerBlockAggregated<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[traversable(flatten)]
    pub full: DistributionFull<Height, T, M>,
    pub rolling: RollingComplete<T, M>,
}

impl<T> PerBlockAggregated<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let full = DistributionFull::forced_import(db, name, version)?;
        let rolling = RollingComplete::forced_import(
            db,
            name,
            version,
            indexes,
            &full.cumulative,
            cached_starts,
        )?;

        Ok(Self { full, rolling })
    }

    /// Compute DistributionFull stats via closure, then rolling distribution from the per-block sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_full: impl FnOnce(&mut DistributionFull<Height, T>) -> Result<()>,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
    {
        compute_full(&mut self.full)?;
        self.rolling
            .compute(max_from, windows, &self.full.sum, exit)?;
        Ok(())
    }
}

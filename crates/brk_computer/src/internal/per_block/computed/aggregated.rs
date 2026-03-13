//! ComputedPerBlockAggregated - Full (distribution + sum + cumulative) + RollingFull.
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
    internal::{CachedWindowStarts, Full, NumericValue, RollingFull, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedPerBlockAggregated<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[traversable(flatten)]
    pub full: Full<Height, T, M>,
    pub rolling: RollingFull<T, M>,
}

impl<T> ComputedPerBlockAggregated<T>
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
        let full = Full::forced_import(db, name, version)?;
        let rolling = RollingFull::forced_import(
            db,
            name,
            version,
            indexes,
            &full.cumulative,
            cached_starts,
        )?;

        Ok(Self { full, rolling })
    }

    /// Compute Full stats via closure, then rolling distribution from the per-block sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_full: impl FnOnce(&mut Full<Height, T>) -> Result<()>,
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
